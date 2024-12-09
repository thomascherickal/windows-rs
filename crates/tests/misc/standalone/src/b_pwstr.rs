#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut PWSTR,
}
pub type PWSTR = *mut u16;
