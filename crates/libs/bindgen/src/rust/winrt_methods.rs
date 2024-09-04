use super::*;

// TODO take Signature instead of MethodDef (wherever MethodDef is found)
pub fn writer(
    writer: &Writer,
    def: metadata::TypeDef,
    generic_types: &[metadata::Type],
    kind: metadata::InterfaceKind,
    method: metadata::MethodDef,
    method_names: &mut MethodNames,
    virtual_names: &mut MethodNames,
) -> TokenStream {
    let signature = metadata::method_def_signature(def.namespace(), method, generic_types);
    let params = if kind == metadata::InterfaceKind::Composable {
        &signature.params[..signature.params.len() - 2]
    } else {
        &signature.params
    };

    let name = if kind == metadata::InterfaceKind::Composable && params.is_empty() {
        quote!(new)
    } else {
        method_names.add(method)
    };

    let interface_name = writer.type_def_name(def, generic_types);
    let vname = virtual_names.add(method);
    let generics = writer.constraint_generics(params);
    let where_clause = writer.where_clause(params);
    let mut cfg = cfg::signature_cfg(writer, method);
    cfg::type_def_cfg_combine(writer, def, generic_types, &mut cfg);
    let features = writer.cfg_features(&cfg);

    let args = if kind == metadata::InterfaceKind::Composable {
        let args = gen_winrt_abi_args(writer, params);
        quote! { #args core::ptr::null_mut(), &mut core::ptr::null_mut(), }
    } else {
        gen_winrt_abi_args(writer, params)
    };

    let params = gen_winrt_params(writer, params);
    let noexcept = metadata::method_def_is_noexcept(method);

    let return_type_tokens = match &signature.return_type {
        metadata::Type::Void => quote! { () },
        _ => {
            let tokens = writer.type_name(&signature.return_type);
            if signature.return_type.is_winrt_array() {
                quote! { windows_core::Array<#tokens> }
            } else {
                quote! { #tokens }
            }
        }
    };

    let return_type_tokens = if noexcept {
        if metadata::type_is_nullable(&signature.return_type) {
            quote! { -> Option<#return_type_tokens> }
        } else if signature.return_type == metadata::Type::Void {
            quote! {}
        } else {
            quote! { -> #return_type_tokens }
        }
    } else {
        quote! { -> windows_core::Result<#return_type_tokens> }
    };

    let return_arg = match &signature.return_type {
        metadata::Type::Void => quote! {},
        _ => {
            if signature.return_type.is_winrt_array() {
                let return_type = writer.type_name(&signature.return_type);
                quote! { windows_core::Array::<#return_type>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _ }
            } else {
                quote! { &mut result__ }
            }
        }
    };

    let vcall = quote! { (windows_core::Interface::vtable(this).#vname)(windows_core::Interface::as_raw(this), #args #return_arg) };

    let vcall = match &signature.return_type {
        metadata::Type::Void => {
            if noexcept {
                quote! {
                    let hresult__ = #vcall;
                    debug_assert!(hresult__.0 == 0);
                }
            } else {
                quote! {
                    #vcall.ok()
                }
            }
        }
        _ if signature.return_type.is_winrt_array() => {
            if noexcept {
                quote! {
                    let mut result__ = core::mem::MaybeUninit::zeroed();
                    let hresult__ = #vcall;
                    debug_assert!(hresult__.0 == 0);
                    result__.assume_init()
                }
            } else {
                quote! {
                    let mut result__ = core::mem::MaybeUninit::zeroed();
                    #vcall
                        .map(|| result__.assume_init())
                }
            }
        }
        _ => {
            if noexcept {
                if metadata::type_is_blittable(&signature.return_type) {
                    quote! {
                    let mut result__ = core::mem::zeroed();
                    let hresult__ = #vcall;
                    debug_assert!(hresult__.0 == 0);
                    result__ }
                } else {
                    quote! {
                    let mut result__ = core::mem::zeroed();
                    let hresult__ = #vcall;
                    debug_assert!(hresult__.0 == 0);
                    core::mem::transmute(result__) }
                }
            } else if metadata::type_is_blittable(&signature.return_type) {
                quote! {
                let mut result__ = core::mem::zeroed();
                #vcall
                .map(||result__) }
            } else {
                quote! { let mut result__ = core::mem::zeroed();
                #vcall.and_then(|| windows_core::Type::from_abi(result__)) }
            }
        }
    };

    match kind {
        metadata::InterfaceKind::Default => quote! {
            #features
            pub fn #name<#generics>(&self, #params) #return_type_tokens #where_clause {
                let this = self;
                unsafe {
                    #vcall
                }
            }
        },
        metadata::InterfaceKind::None
        | metadata::InterfaceKind::Base
        | metadata::InterfaceKind::Overridable => {
            quote! {
                #features
                pub fn #name<#generics>(&self, #params) #return_type_tokens #where_clause {
                    let this = &windows_core::Interface::cast::<#interface_name>(self)?;
                    unsafe {
                        #vcall
                    }
                }
            }
        }
        metadata::InterfaceKind::Static | metadata::InterfaceKind::Composable => {
            quote! {
                #features
                pub fn #name<#generics>(#params) #return_type_tokens #where_clause {
                    Self::#interface_name(|this| unsafe { #vcall })
                }
            }
        }
    }
}

fn gen_winrt_params(writer: &Writer, params: &[metadata::SignatureParam]) -> TokenStream {
    let mut result = quote! {};

    let mut generic_params = writer.generic_params(params);
    for param in params.iter() {
        let name = writer.param_name(param.def);
        let kind = writer.type_name(&param.ty);
        let default_type = writer.type_default_name(&param.ty);

        if param.def.flags().contains(metadata::ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                result.combine(&quote! { #name: &[#default_type], });
            } else if param.is_convertible() {
                let (position, _) = generic_params.next().unwrap();
                let kind: TokenStream = format!("P{position}").into();
                result.combine(&quote! { #name: #kind, });
            } else if metadata::type_is_blittable(&param.ty) {
                result.combine(&quote! { #name: #kind, });
            } else {
                result.combine(&quote! { #name: &#kind, });
            }
        } else if param.ty.is_winrt_array() {
            result.combine(&quote! { #name: &mut [#default_type], });
        } else if param.ty.is_winrt_array_ref() {
            result.combine(&quote! { #name: &mut windows_core::Array<#kind>, });
        } else {
            result.combine(&quote! { #name: &mut #default_type, });
        }
    }

    result
}

fn gen_winrt_abi_args(writer: &Writer, params: &[metadata::SignatureParam]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for param in params {
        let name = writer.param_name(param.def);

        let param = if param.def.flags().contains(metadata::ParamAttributes::In) {
            if param.ty.is_winrt_array() {
                if metadata::type_is_blittable(&param.ty) {
                    quote! { #name.len().try_into().unwrap(), #name.as_ptr(), }
                } else {
                    quote! { #name.len().try_into().unwrap(), core::mem::transmute(#name.as_ptr()), }
                }
            } else if metadata::type_is_borrowed(&param.ty) {
                quote! { #name.param().abi(), }
            } else if metadata::type_is_blittable(&param.ty) {
                if param.ty.is_const_ref() {
                    quote! { &#name, }
                } else {
                    quote! { #name, }
                }
            } else {
                quote! { core::mem::transmute_copy(#name), }
            }
        } else if param.ty.is_winrt_array() {
            if metadata::type_is_blittable(&param.ty) {
                quote! { #name.len().try_into().unwrap(), #name.as_mut_ptr(), }
            } else {
                quote! { #name.len().try_into().unwrap(), core::mem::transmute_copy(&#name), }
            }
        } else if param.ty.is_winrt_array_ref() {
            quote! { #name.set_abi_len(), #name as *mut _ as _, }
        } else if metadata::type_is_blittable(&param.ty) {
            quote! { #name, }
        } else {
            quote! { #name as *mut _ as _, }
        };
        tokens.combine(&param);
    }
    tokens
}

pub fn gen_upcall(
    writer: &Writer,
    sig: &metadata::Signature,
    inner: TokenStream,
    this: bool,
) -> TokenStream {
    let noexcept = metadata::method_def_is_noexcept(sig.def);

    let invoke_args = sig
        .params
        .iter()
        .map(|param| gen_winrt_invoke_arg(writer, param));

    let this = if this {
        quote! { this, }
    } else {
        quote! {}
    };

    match &sig.return_type {
        metadata::Type::Void => {
            if noexcept {
                quote! {
                    #inner(#this #(#invoke_args,)*);
                    windows_core::HRESULT(0)
                }
            } else {
                quote! {
                    #inner(#this #(#invoke_args,)*).into()
                }
            }
        }
        _ if sig.return_type.is_winrt_array() => {
            if noexcept {
                quote! {
                    let ok__ = #inner(#this #(#invoke_args,)*);
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    result__.write(ok_data__);
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
            } else {
                quote! {
                    match #inner(#this #(#invoke_args,)*) {
                        Ok(ok__) => {
                            let (ok_data__, ok_data_len__) = ok__.into_abi();
                            // use `ptr::write` since `result` could be uninitialized
                            result__.write(ok_data__);
                            result_size__.write(ok_data_len__);
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into()
                    }
                }
            }
        }
        _ => {
            let forget = if metadata::type_is_blittable(&sig.return_type) {
                quote! {}
            } else {
                quote! { core::mem::forget(ok__); }
            };

            if noexcept {
                quote! {
                    let ok__ = #inner(#this #(#invoke_args,)*);
                    // use `ptr::write` since `result` could be uninitialized
                    result__.write(core::mem::transmute_copy(&ok__));
                    #forget
                    windows_core::HRESULT(0)
                }
            } else {
                quote! {
                    match #inner(#this #(#invoke_args,)*) {
                        Ok(ok__) => {
                            // use `ptr::write` since `result` could be uninitialized
                            result__.write(core::mem::transmute_copy(&ok__));
                            #forget
                            windows_core::HRESULT(0)
                        }
                        Err(err) => err.into()
                    }
                }
            }
        }
    }
}

fn gen_winrt_invoke_arg(writer: &Writer, param: &metadata::SignatureParam) -> TokenStream {
    let name = writer.param_name(param.def);
    let abi_size_name: TokenStream = format!("{}_array_size", param.def.name()).into();

    if param.def.flags().contains(metadata::ParamAttributes::In) {
        if param.ty.is_winrt_array() {
            quote! { core::slice::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
        } else if metadata::type_is_primitive(&param.ty) {
            quote! { #name }
        } else if param.ty.is_const_ref() {
            quote! { core::mem::transmute_copy(&#name) }
        } else if metadata::type_is_nullable(&param.ty) {
            quote! { windows_core::from_raw_borrowed(&#name) }
        } else {
            quote! { core::mem::transmute(&#name) }
        }
    } else if param.ty.is_winrt_array() {
        quote! { core::slice::from_raw_parts_mut(core::mem::transmute_copy(&#name), #abi_size_name as usize) }
    } else if param.ty.is_winrt_array_ref() {
        quote! { windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&#name), #abi_size_name).as_array() }
    } else {
        quote! { core::mem::transmute_copy(&#name) }
    }
}
