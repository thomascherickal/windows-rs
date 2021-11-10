#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_WPD_NAMESPACE_EXTENSION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(897084732, 45173, 18873, [136, 221, 2, 152, 118, 225, 28, 1]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DELETE_OBJECT_OPTIONS(pub i32);
pub const PORTABLE_DEVICE_DELETE_NO_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(0i32);
pub const PORTABLE_DEVICE_DELETE_WITH_RECURSION: DELETE_OBJECT_OPTIONS = DELETE_OBJECT_OPTIONS(1i32);
impl ::core::convert::From<i32> for DELETE_OBJECT_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DELETE_OBJECT_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DEVICE_RADIO_STATE(pub i32);
pub const DRS_RADIO_ON: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(0i32);
pub const DRS_SW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(1i32);
pub const DRS_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(2i32);
pub const DRS_SW_HW_RADIO_OFF: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(3i32);
pub const DRS_HW_RADIO_ON_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(4i32);
pub const DRS_RADIO_INVALID: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(5i32);
pub const DRS_HW_RADIO_OFF_UNCONTROLLABLE: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
pub const DRS_RADIO_MAX: DEVICE_RADIO_STATE = DEVICE_RADIO_STATE(6i32);
impl ::core::convert::From<i32> for DEVICE_RADIO_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEVICE_RADIO_STATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_MTPBTH_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3927062522, 22685, 17522, [132, 228, 10, 190, 54, 253, 98, 239]),
    pid: 2u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const DEVSVCTYPE_ABSTRACT: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const DEVSVCTYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const DEVSVC_SERVICEINFO_VERSION: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DMProcessConfigXMLFiltered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszxmlin: Param0, rgszallowedcspnodes: *const super::super::Foundation::PWSTR, dwnumallowedcspnodes: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DMProcessConfigXMLFiltered(pszxmlin: super::super::Foundation::PWSTR, rgszallowedcspnodes: *const super::super::Foundation::PWSTR, dwnumallowedcspnodes: u32, pbstrxmlout: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        DMProcessConfigXMLFiltered(pszxmlin.into_param().abi(), ::core::mem::transmute(rgszallowedcspnodes), ::core::mem::transmute(dwnumallowedcspnodes), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_AnchorStateInvalid: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_AnchorStateNormal: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_AnchorStateOld: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_ItemStateChanged: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_ItemStateCreated: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_ItemStateDeleted: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_ItemStateInvalid: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_AnchorResults_ItemStateUpdated: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_CalendarObj_BusyStatusBusy: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_CalendarObj_BusyStatusFree: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_CalendarObj_BusyStatusOutOfOffice: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_CalendarObj_BusyStatusTentative: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABFalse: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_DeviceMetadataObj_DefaultCABTrue: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceFirst: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceFourth: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceLast: u32 = 5u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceNone: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceSecond: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternInstanceThird: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternTypeDaily: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternTypeMonthly: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternTypeWeekly: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PatternTypeYearly: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PriorityHighest: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PriorityLowest: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_PriorityNormal: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_ReadFalse: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_MessageObj_ReadTrue: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_ChargingActive: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_ChargingInactive: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_ChargingUnknown: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_RoamingActive: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_RoamingInactive: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_StatusSvc_RoamingUnknown: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesDisabled: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_SyncSvc_SyncObjectReferencesEnabled: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_TaskObj_CompleteFalse: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const ENUM_TaskObj_CompleteTrue: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_DEVICE_ALREADY_OPENED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731135i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_DEVICE_IS_HUNG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731130i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_DEVICE_NOT_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731134i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_DEVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731133i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_OBJECT_ALREADY_ATTACHED_TO_SERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144730934i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_DEVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731132i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_OBJECT_NOT_ATTACHED_TO_SERVICE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144730933i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_OBJECT_NOT_COMMITED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731131i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SERVICE_ALREADY_OPENED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144730936i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SERVICE_BAD_PARAMETER_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144730932i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SERVICE_NOT_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144730935i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SMS_INVALID_MESSAGE_BODY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731035i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SMS_INVALID_RECIPIENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731036i32 as _);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const E_WPD_SMS_SERVICE_UNAVAILABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144731034i32 as _);
pub const EnumBthMtpConnectors: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2706833737, 58949, 20291, [139, 13, 64, 155, 6, 29, 178, 252]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FACILITY_WPD: u32 = 42u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekFriday: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekMonday: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekNone: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekSaturday: u32 = 64u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekSunday: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekThursday: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekTuesday: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const FLAG_MessageObj_DayOfWeekWednesday: u32 = 8u32;
pub const GUID_DEVINTERFACE_WPD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1791129720, 42746, 16725, [186, 133, 249, 143, 73, 29, 79, 51]);
pub const GUID_DEVINTERFACE_WPD_PRIVATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121377679, 19949, 18871, [189, 211, 250, 190, 40, 102, 18, 17]);
pub const GUID_DEVINTERFACE_WPD_SERVICE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2666811264, 15716, 16966, [166, 170, 32, 111, 50, 141, 30, 220]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IConnectionRequestCallback(pub ::windows::runtime::IUnknown);
impl IConnectionRequestCallback {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConnectionRequestCallback {
    type Vtable = IConnectionRequestCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(657234656, 29025, 19168, [145, 189, 159, 68, 142, 233, 196, 39]);
}
impl ::core::convert::From<IConnectionRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: IConnectionRequestCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IConnectionRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IConnectionRequestCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConnectionRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConnectionRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionRequestCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumPortableDeviceConnectors(pub ::windows::runtime::IUnknown);
impl IEnumPortableDeviceConnectors {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Next(&self, crequested: u32, pconnectors: *mut ::core::option::Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(crequested), ::core::mem::transmute(pconnectors), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Skip(&self, cconnectors: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cconnectors)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumPortableDeviceConnectors> {
        let mut result__: <IEnumPortableDeviceConnectors as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumPortableDeviceConnectors>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumPortableDeviceConnectors {
    type Vtable = IEnumPortableDeviceConnectors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3219060041, 37447, 17743, [189, 130, 6, 254, 128, 133, 63, 170]);
}
impl ::core::convert::From<IEnumPortableDeviceConnectors> for ::windows::runtime::IUnknown {
    fn from(value: IEnumPortableDeviceConnectors) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumPortableDeviceConnectors> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumPortableDeviceConnectors) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumPortableDeviceConnectors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceConnectors_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, crequested: u32, pconnectors: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cconnectors: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IEnumPortableDeviceObjectIDs(pub ::windows::runtime::IUnknown);
impl IEnumPortableDeviceObjectIDs {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Next(&self, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(cobjects), ::core::mem::transmute(pobjids), ::core::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Skip(&self, cobjects: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(cobjects)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: <IEnumPortableDeviceObjectIDs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumPortableDeviceObjectIDs {
    type Vtable = IEnumPortableDeviceObjectIDs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(283961685, 53057, 18216, [191, 160, 65, 238, 223, 27, 191, 25]);
}
impl ::core::convert::From<IEnumPortableDeviceObjectIDs> for ::windows::runtime::IUnknown {
    fn from(value: IEnumPortableDeviceObjectIDs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IEnumPortableDeviceObjectIDs> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumPortableDeviceObjectIDs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumPortableDeviceObjectIDs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumPortableDeviceObjectIDs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cobjects: u32, pobjids: *mut super::super::Foundation::PWSTR, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cobjects: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaRadioManager(pub ::windows::runtime::IUnknown);
impl IMediaRadioManager {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetRadioInstances(&self) -> ::windows::runtime::Result<IRadioInstanceCollection> {
        let mut result__: <IRadioInstanceCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IRadioInstanceCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnSystemRadioStateChange(&self, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(sysradiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMediaRadioManager {
    type Vtable = IMediaRadioManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1828571829, 64583, 17061, [146, 65, 7, 75, 88, 131, 14, 115]);
}
impl ::core::convert::From<IMediaRadioManager> for ::windows::runtime::IUnknown {
    fn from(value: IMediaRadioManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaRadioManager> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaRadioManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaRadioManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaRadioManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IMediaRadioManagerNotifySink(pub ::windows::runtime::IUnknown);
impl IMediaRadioManagerNotifySink {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnInstanceAdd<'a, Param0: ::windows::runtime::IntoParam<'a, IRadioInstance>>(&self, pradioinstance: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pradioinstance.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn OnInstanceRemove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrradioinstanceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), bstrradioinstanceid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn OnInstanceRadioChange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrradioinstanceid: Param0, radiostate: DEVICE_RADIO_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), bstrradioinstanceid.into_param().abi(), ::core::mem::transmute(radiostate)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMediaRadioManagerNotifySink {
    type Vtable = IMediaRadioManagerNotifySink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2312642399, 49479, 18925, [161, 28, 119, 178, 12, 49, 231, 201]);
}
impl ::core::convert::From<IMediaRadioManagerNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: IMediaRadioManagerNotifySink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMediaRadioManagerNotifySink> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaRadioManagerNotifySink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMediaRadioManagerNotifySink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRadioManagerNotifySink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pradioinstance: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrradioinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const IOCTL_WPD_MESSAGE_READWRITE_ACCESS: u32 = 4243720u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const IOCTL_WPD_MESSAGE_READ_ACCESS: u32 = 4210952u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDevice(pub ::windows::runtime::IUnknown);
impl IPortableDevice {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpdeviceid: Param0, pclientinfo: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SendCommand<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Content(&self) -> ::windows::runtime::Result<IPortableDeviceContent> {
        let mut result__: <IPortableDeviceContent as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceContent>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Capabilities(&self) -> ::windows::runtime::Result<IPortableDeviceCapabilities> {
        let mut result__: <IPortableDeviceCapabilities as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceCapabilities>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Advise<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcookie: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszcookie.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetPnPDeviceID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDevice {
    type Vtable = IPortableDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1650339320, 25490, 19696, [154, 209, 60, 250, 95, 23, 119, 92]);
}
impl ::core::convert::From<IPortableDevice> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pparameters: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pcallback: ::windows::runtime::RawPtr, pparameters: ::windows::runtime::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcookie: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceCapabilities(pub ::windows::runtime::IUnknown);
impl IPortableDeviceCapabilities {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetFunctionalCategories(&self) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetFunctionalObjects(&self, category: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedContentTypes(&self, category: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(category), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedFormats(&self, contenttype: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(contenttype), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetFixedPropertyAttributes(&self, format: *const ::windows::runtime::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetEventOptions(&self, event: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceCapabilities {
    type Vtable = IPortableDeviceCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(747400639, 58332, 16481, [190, 204, 133, 66, 232, 16, 209, 38]);
}
impl ::core::convert::From<IPortableDeviceCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcommands: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcategories: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: *const ::windows::runtime::GUID, ppobjectids: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, category: *const ::windows::runtime::GUID, ppcontenttypes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contenttype: *const ::windows::runtime::GUID, ppformats: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, ppkeys: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppevents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: *const ::windows::runtime::GUID, ppoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceConnector(pub ::windows::runtime::IUnknown);
impl IPortableDeviceConnector {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Connect<'a, Param0: ::windows::runtime::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Disconnect<'a, Param0: ::windows::runtime::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel<'a, Param0: ::windows::runtime::IntoParam<'a, IConnectionRequestCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Devices_Properties")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Devices_Properties`*"]
    pub unsafe fn GetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(ppropertytype), ::core::mem::transmute(ppdata), ::core::mem::transmute(pcbdata)).ok()
    }
    #[cfg(feature = "Win32_Devices_Properties")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Devices_Properties`*"]
    pub unsafe fn SetProperty(&self, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppropertykey), ::core::mem::transmute(propertytype), ::core::mem::transmute(pdata), ::core::mem::transmute(cbdata)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetPnPID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceConnector {
    type Vtable = IPortableDeviceConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1650339320, 25490, 19696, [154, 209, 60, 250, 95, 23, 119, 92]);
}
impl ::core::convert::From<IPortableDeviceConnector> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceConnector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceConnector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Devices_Properties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut u32, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))] usize,
    #[cfg(feature = "Win32_Devices_Properties")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: u32, pdata: *const u8, cbdata: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Devices_Properties"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppwszpnpid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceContent(pub ::windows::runtime::IUnknown);
impl IPortableDeviceContent {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn EnumObjects<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows::runtime::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: <IEnumPortableDeviceObjectIDs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), &mut result__).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Properties(&self) -> ::windows::runtime::Result<IPortableDeviceProperties> {
        let mut result__: <IPortableDeviceProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Transfer(&self) -> ::windows::runtime::Result<IPortableDeviceResources> {
        let mut result__: <IPortableDeviceResources as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceResources>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ppersistentuniqueids.into_param().abi(), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Move<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Copy<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceContent {
    type Vtable = IPortableDeviceContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1788276100, 31859, 17536, [153, 56, 191, 90, 244, 119, 212, 38]);
}
impl ::core::convert::From<IPortableDeviceContent> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceContent> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: ::windows::runtime::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: ::windows::runtime::RawPtr, ppdata: *mut ::windows::runtime::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoptions: u32, pobjectids: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppersistentuniqueids: ::windows::runtime::RawPtr, ppobjectids: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectids: ::windows::runtime::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectids: ::windows::runtime::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceContent2(pub ::windows::runtime::IUnknown);
impl IPortableDeviceContent2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn EnumObjects<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pszparentobjectid: Param1, pfilter: Param2) -> ::windows::runtime::Result<IEnumPortableDeviceObjectIDs> {
        let mut result__: <IEnumPortableDeviceObjectIDs as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pszparentobjectid.into_param().abi(), pfilter.into_param().abi(), &mut result__).from_abi::<IEnumPortableDeviceObjectIDs>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Properties(&self) -> ::windows::runtime::Result<IPortableDeviceProperties> {
        let mut result__: <IPortableDeviceProperties as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceProperties>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Transfer(&self) -> ::windows::runtime::Result<IPortableDeviceResources> {
        let mut result__: <IPortableDeviceResources as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceResources>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn CreateObjectWithPropertiesOnly<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppszobjectid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateObjectWithPropertiesAndData<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pvalues.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Delete<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, dwoptions: u32, pobjectids: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoptions), pobjectids.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, ppersistentuniqueids: Param0) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ppersistentuniqueids.into_param().abi(), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Move<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Copy<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectids: Param0, pszdestinationfolderobjectid: Param1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pszdestinationfolderobjectid.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn UpdateObjectWithPropertiesAndData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pproperties: Param1, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pproperties.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceContent2 {
    type Vtable = IPortableDeviceContent2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2605374870, 63167, 16436, [135, 8, 236, 167, 43, 241, 5, 84]);
}
impl ::core::convert::From<IPortableDeviceContent2> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceContent2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceContent2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: IPortableDeviceContent2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPortableDeviceContent2> for IPortableDeviceContent {
    fn from(value: &IPortableDeviceContent2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPortableDeviceContent> for IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPortableDeviceContent> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPortableDeviceContent> for &IPortableDeviceContent2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPortableDeviceContent> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceContent2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pszparentobjectid: super::super::Foundation::PWSTR, pfilter: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppresources: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: ::windows::runtime::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: ::windows::runtime::RawPtr, ppdata: *mut ::windows::runtime::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoptions: u32, pobjectids: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppersistentuniqueids: ::windows::runtime::RawPtr, ppobjectids: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectids: ::windows::runtime::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectids: ::windows::runtime::RawPtr, pszdestinationfolderobjectid: super::super::Foundation::PWSTR, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, pproperties: ::windows::runtime::RawPtr, ppdata: *mut ::windows::runtime::RawPtr, pdwoptimalwritebuffersize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceDataStream(pub ::windows::runtime::IUnknown);
impl IPortableDeviceDataStream {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pv), ::core::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_System_Com`*"]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(libnewsize)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_System_Com`*"]
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pstm.into_param().abi(), ::core::mem::transmute(cb), ::core::mem::transmute(pcbread), ::core::mem::transmute(pcbwritten)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(grfcommitflags)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(liboffset), ::core::mem::transmute(cb), ::core::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstatstg), ::core::mem::transmute(grfstatflag)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_System_Com`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<super::super::System::Com::IStream> {
        let mut result__: <super::super::System::Com::IStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::IStream>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetObjectID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceDataStream {
    type Vtable = IPortableDeviceDataStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2296401331, 4114, 19812, [153, 150, 247, 3, 169, 80, 211, 244]);
}
impl ::core::convert::From<IPortableDeviceDataStream> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceDataStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceDataStream> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceDataStream> for super::super::System::Com::IStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceDataStream> for super::super::System::Com::IStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IStream> for &IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceDataStream> for super::super::System::Com::ISequentialStream {
    fn from(value: IPortableDeviceDataStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceDataStream> for super::super::System::Com::ISequentialStream {
    fn from(value: &IPortableDeviceDataStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::ISequentialStream> for &IPortableDeviceDataStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::ISequentialStream> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDataStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceDispatchFactory(pub ::windows::runtime::IUnknown);
impl IPortableDeviceDispatchFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetDeviceDispatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0) -> ::windows::runtime::Result<super::super::System::Com::IDispatch> {
        let mut result__: <super::super::System::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceDispatchFactory {
    type Vtable = IPortableDeviceDispatchFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1579069379, 58327, 16690, [150, 250, 117, 156, 15, 157, 30, 15]);
}
impl ::core::convert::From<IPortableDeviceDispatchFactory> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceDispatchFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceDispatchFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceDispatchFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceDispatchFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceDispatchFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, ppdevicedispatch: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceEventCallback(pub ::windows::runtime::IUnknown);
impl IPortableDeviceEventCallback {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnEvent<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, peventparameters: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), peventparameters.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceEventCallback {
    type Vtable = IPortableDeviceEventCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2826512945, 62341, 18748, [168, 147, 64, 246, 78, 180, 95, 110]);
}
impl ::core::convert::From<IPortableDeviceEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceEventCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceEventCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceEventCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peventparameters: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceKeyCollection(pub ::windows::runtime::IUnknown);
impl IPortableDeviceKeyCollection {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pkey)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceKeyCollection {
    type Vtable = IPortableDeviceKeyCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3671728983, 57517, 18734, [152, 219, 221, 97, 197, 59, 163, 83]);
}
impl ::core::convert::From<IPortableDeviceKeyCollection> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceKeyCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceKeyCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceKeyCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceKeyCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceKeyCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcelems: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceManager(pub ::windows::runtime::IUnknown);
impl IPortableDeviceManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDevices(&self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn RefreshDeviceList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicefriendlyname: Param1, pcchdevicefriendlyname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicefriendlyname.into_param().abi(), ::core::mem::transmute(pcchdevicefriendlyname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicedescription: Param1, pcchdevicedescription: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicedescription.into_param().abi(), ::core::mem::transmute(pcchdevicedescription)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceManufacturer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pdevicemanufacturer: Param1, pcchdevicemanufacturer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pdevicemanufacturer.into_param().abi(), ::core::mem::transmute(pcchdevicemanufacturer)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, pszdevicepropertyname: Param1, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), pszdevicepropertyname.into_param().abi(), ::core::mem::transmute(pdata), ::core::mem::transmute(pcbdata), ::core::mem::transmute(pdwtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetPrivateDevices(&self, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(ppnpdeviceids), ::core::mem::transmute(pcpnpdeviceids)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceManager {
    type Vtable = IPortableDeviceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2706797973, 19503, 17780, [166, 250, 236, 239, 145, 123, 154, 64]);
}
impl ::core::convert::From<IPortableDeviceManager> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicefriendlyname: super::super::Foundation::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicedescription: super::super::Foundation::PWSTR, pcchdevicedescription: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, pdevicemanufacturer: super::super::Foundation::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, pszdevicepropertyname: super::super::Foundation::PWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnpdeviceids: *mut super::super::Foundation::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDevicePropVariantCollection(pub ::windows::runtime::IUnknown);
impl IPortableDevicePropVariantCollection {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn GetAt(&self, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    pub unsafe fn Add(&self, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetType(&self) -> ::windows::runtime::Result<u16> {
        let mut result__: <u16 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u16>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn ChangeType(&self, vt: u16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(vt)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDevicePropVariantCollection {
    type Vtable = IPortableDevicePropVariantCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2310202402, 20251, 17174, [188, 239, 164, 74, 254, 168, 62, 179]);
}
impl ::core::convert::From<IPortableDevicePropVariantCollection> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDevicePropVariantCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDevicePropVariantCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDevicePropVariantCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDevicePropVariantCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropVariantCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcelems: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, pvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvt: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vt: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceProperties(pub ::windows::runtime::IUnknown);
impl IPortableDeviceProperties {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetPropertyAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetValues<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi(), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn SetValues<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pszobjectid: Param0, pvalues: Param1) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pvalues.into_param().abi(), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceProperties {
    type Vtable = IPortableDeviceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2137876828, 991, 17465, [168, 9, 89, 38, 107, 238, 227, 166]);
}
impl ::core::convert::From<IPortableDeviceProperties> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::runtime::RawPtr, ppvalues: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, pvalues: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDevicePropertiesBulk(pub ::windows::runtime::IUnknown);
impl IPortableDevicePropertiesBulk {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn QueueGetValuesByObjectList<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>, Param2: ::windows::runtime::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectids: Param0, pkeys: Param1, pcallback: Param2) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobjectids.into_param().abi(), pkeys.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn QueueGetValuesByObjectFormat<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>, Param4: ::windows::runtime::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pguidobjectformat: *const ::windows::runtime::GUID, pszparentobjectid: Param1, dwdepth: u32, pkeys: Param3, pcallback: Param4) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pguidobjectformat), pszparentobjectid.into_param().abi(), ::core::mem::transmute(dwdepth), pkeys.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn QueueSetValuesByObjectList<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValuesCollection>, Param1: ::windows::runtime::IntoParam<'a, IPortableDevicePropertiesBulkCallback>>(&self, pobjectvalues: Param0, pcallback: Param1) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pobjectvalues.into_param().abi(), pcallback.into_param().abi(), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Start(&self, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDevicePropertiesBulk {
    type Vtable = IPortableDevicePropertiesBulk_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1210779072, 16470, 17645, [158, 15, 94, 35, 176, 9, 218, 147]);
}
impl ::core::convert::From<IPortableDevicePropertiesBulk> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDevicePropertiesBulk) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulk> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulk) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDevicePropertiesBulk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulk_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectids: ::windows::runtime::RawPtr, pkeys: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pcontext: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidobjectformat: *const ::windows::runtime::GUID, pszparentobjectid: super::super::Foundation::PWSTR, dwdepth: u32, pkeys: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pcontext: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectvalues: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, pcontext: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDevicePropertiesBulkCallback(pub ::windows::runtime::IUnknown);
impl IPortableDevicePropertiesBulkCallback {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnStart(&self, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnProgress<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, pcontext: *const ::windows::runtime::GUID, presults: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext), presults.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnEnd(&self, pcontext: *const ::windows::runtime::GUID, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontext), ::core::mem::transmute(hrstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDevicePropertiesBulkCallback {
    type Vtable = IPortableDevicePropertiesBulkCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2649410432, 4584, 16611, [169, 243, 245, 87, 152, 106, 120, 69]);
}
impl ::core::convert::From<IPortableDevicePropertiesBulkCallback> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDevicePropertiesBulkCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDevicePropertiesBulkCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDevicePropertiesBulkCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDevicePropertiesBulkCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDevicePropertiesBulkCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *const ::windows::runtime::GUID, presults: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: *const ::windows::runtime::GUID, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceResources(pub ::windows::runtime::IUnknown);
impl IPortableDeviceResources {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetSupportedResources<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetResourceAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszobjectid: Param0, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), ::core::mem::transmute(key), ::core::mem::transmute(dwmode), ::core::mem::transmute(pdwoptimalbuffersize), ::core::mem::transmute(ppstream)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Delete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, pszobjectid: Param0, pkeys: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszobjectid.into_param().abi(), pkeys.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn CreateResource<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, presourceattributes: Param0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), presourceattributes.into_param().abi(), ::core::mem::transmute(ppdata), ::core::mem::transmute(pdwoptimalwritebuffersize), ::core::mem::transmute(ppszcookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceResources {
    type Vtable = IPortableDeviceResources_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4253579436, 55361, 19735, [137, 28, 230, 130, 156, 219, 105, 52]);
}
impl ::core::convert::From<IPortableDeviceResources> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceResources) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceResources> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceResources) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceResources {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceResources {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceResources_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, ppkeys: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszobjectid: super::super::Foundation::PWSTR, pkeys: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourceattributes: ::windows::runtime::RawPtr, ppdata: *mut ::windows::runtime::RawPtr, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceService(pub ::windows::runtime::IUnknown);
impl IPortableDeviceService {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Open<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Capabilities(&self) -> ::windows::runtime::Result<IPortableDeviceServiceCapabilities> {
        let mut result__: <IPortableDeviceServiceCapabilities as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceServiceCapabilities>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Content(&self) -> ::windows::runtime::Result<IPortableDeviceContent2> {
        let mut result__: <IPortableDeviceContent2 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceContent2>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Methods(&self) -> ::windows::runtime::Result<IPortableDeviceServiceMethods> {
        let mut result__: <IPortableDeviceServiceMethods as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceServiceMethods>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetServiceObjectID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetPnPServiceID(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Advise<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceEventCallback>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pcallback: Param1, pparameters: Param2) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pcallback.into_param().abi(), pparameters.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcookie: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszcookie.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SendCommand<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwflags: u32, pparameters: Param1) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwflags), pparameters.into_param().abi(), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceService {
    type Vtable = IPortableDeviceService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3552393796, 55221, 16553, [152, 183, 47, 164, 208, 29, 236, 8]);
}
impl ::core::convert::From<IPortableDeviceService> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceService> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcapabilities: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontent: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmethods: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszserviceobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppszpnpserviceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pcallback: ::windows::runtime::RawPtr, pparameters: ::windows::runtime::RawPtr, ppszcookie: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcookie: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32, pparameters: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceActivation(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceActivation {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn OpenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceServiceOpenCallback>>(&self, pszpnpserviceid: Param0, pclientinfo: Param1, pcallback: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), pclientinfo.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn CancelOpenAsync(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceActivation {
    type Vtable = IPortableDeviceServiceActivation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3848996148, 55737, 16988, [155, 153, 117, 249, 124, 179, 215, 200]);
}
impl ::core::convert::From<IPortableDeviceServiceActivation> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceActivation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceActivation> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceActivation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceActivation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceActivation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpserviceid: super::super::Foundation::PWSTR, pclientinfo: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceCapabilities(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceCapabilities {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedMethods(&self) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedMethodsByFormat(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetMethodAttributes(&self, method: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetMethodParameterAttributes(&self, method: *const ::windows::runtime::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), ::core::mem::transmute(parameter), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedFormats(&self) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetFormatAttributes(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedFormatProperties(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetFormatPropertyAttributes(&self, format: *const ::windows::runtime::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), ::core::mem::transmute(property), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedEvents(&self) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetEventAttributes(&self, event: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetEventParameterAttributes(&self, event: *const ::windows::runtime::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(event), ::core::mem::transmute(parameter), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetInheritedServices(&self, dwinheritancetype: u32) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwinheritancetype), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetFormatRenderingProfiles(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IPortableDeviceValuesCollection> {
        let mut result__: <IPortableDeviceValuesCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(format), &mut result__).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSupportedCommands(&self) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetCommandOptions(&self, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(command), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceCapabilities {
    type Vtable = IPortableDeviceServiceCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(618387613, 16702, 17376, [189, 91, 25, 127, 60, 86, 200, 134]);
}
impl ::core::convert::From<IPortableDeviceServiceCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmethods: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, ppmethods: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, method: *const ::windows::runtime::GUID, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, method: *const ::windows::runtime::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppformats: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, ppkeys: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppevents: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: *const ::windows::runtime::GUID, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, event: *const ::windows::runtime::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwinheritancetype: u32, ppservices: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID, pprenderingprofiles: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcommands: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceManager(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceServices<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpdeviceid: Param0, guidservicecategory: *const ::windows::runtime::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpnpdeviceid.into_param().abi(), ::core::mem::transmute(guidservicecategory), ::core::mem::transmute(pservices), ::core::mem::transmute(pcservices)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetDeviceForService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpnpserviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpnpserviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceManager {
    type Vtable = IPortableDeviceServiceManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2829829353, 43082, 18345, [128, 179, 197, 217, 177, 114, 169, 97]);
}
impl ::core::convert::From<IPortableDeviceServiceManager> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpdeviceid: super::super::Foundation::PWSTR, guidservicecategory: *const ::windows::runtime::GUID, pservices: *mut super::super::Foundation::PWSTR, pcservices: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpnpserviceid: super::super::Foundation::PWSTR, ppszpnpdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceMethodCallback(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceMethodCallback {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnComplete<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, hrstatus: ::windows::runtime::HRESULT, presults: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus), presults.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceMethodCallback {
    type Vtable = IPortableDeviceServiceMethodCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3290702652, 45006, 18472, [167, 86, 126, 215, 162, 53, 0, 131]);
}
impl ::core::convert::From<IPortableDeviceServiceMethodCallback> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceMethodCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethodCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceMethodCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceMethodCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethodCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrstatus: ::windows::runtime::HRESULT, presults: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceMethods(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceMethods {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Invoke<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, method: *const ::windows::runtime::GUID, pparameters: Param1, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), pparameters.into_param().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn InvokeAsync<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>, Param2: ::windows::runtime::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, method: *const ::windows::runtime::GUID, pparameters: Param1, pcallback: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(method), pparameters.into_param().abi(), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceServiceMethodCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceMethods {
    type Vtable = IPortableDeviceServiceMethods_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3791860681, 64820, 16685, [163, 129, 204, 111, 45, 130, 13, 247]);
}
impl ::core::convert::From<IPortableDeviceServiceMethods> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceMethods) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceMethods> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceMethods) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceMethods {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceMethods_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, method: *const ::windows::runtime::GUID, pparameters: ::windows::runtime::RawPtr, ppresults: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, method: *const ::windows::runtime::GUID, pparameters: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceServiceOpenCallback(pub ::windows::runtime::IUnknown);
impl IPortableDeviceServiceOpenCallback {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn OnComplete(&self, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(hrstatus)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceServiceOpenCallback {
    type Vtable = IPortableDeviceServiceOpenCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3169667528, 36606, 16877, [150, 11, 97, 49, 58, 189, 71, 169]);
}
impl ::core::convert::From<IPortableDeviceServiceOpenCallback> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceServiceOpenCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceServiceOpenCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceServiceOpenCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceServiceOpenCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceServiceOpenCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrstatus: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceUnitsStream(pub ::windows::runtime::IUnknown);
impl IPortableDeviceUnitsStream {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SeekInUnits(&self, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(dlibmove), ::core::mem::transmute(units), ::core::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceUnitsStream {
    type Vtable = IPortableDeviceUnitsStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1587020383, 49092, 18338, [154, 95, 188, 144, 10, 80, 124, 103]);
}
impl ::core::convert::From<IPortableDeviceUnitsStream> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceUnitsStream) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceUnitsStream> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceUnitsStream) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceUnitsStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceUnitsStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceValues(pub ::windows::runtime::IUnknown);
impl IPortableDeviceValues {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetCount(&self, pcelt: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(pkey), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(pvalue)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::System::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::System::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetStringValue<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetStringValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetUnsignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSignedIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetUnsignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetSignedLargeIntegerValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<i64>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetFloatValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<f32>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetErrorValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetKeyValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__: <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetBoolValue<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), value.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetBoolValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetIUnknownValue<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetIUnknownValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetGuidValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(pvalue), ::core::mem::transmute(cbvalue)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetBufferValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), ::core::mem::transmute(ppvalue), ::core::mem::transmute(pcbvalue)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetIPortableDeviceValuesValue<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetIPortableDeviceValuesValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetIPortableDevicePropVariantCollectionValue<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDevicePropVariantCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetIPortableDevicePropVariantCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDevicePropVariantCollection> {
        let mut result__: <IPortableDevicePropVariantCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDevicePropVariantCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetIPortableDeviceKeyCollectionValue<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceKeyCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetIPortableDeviceKeyCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceKeyCollection> {
        let mut result__: <IPortableDeviceKeyCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceKeyCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn SetIPortableDeviceValuesCollectionValue<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValuesCollection>>(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), pvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn GetIPortableDeviceValuesCollectionValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<IPortableDeviceValuesCollection> {
        let mut result__: <IPortableDeviceValuesCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::core::mem::transmute_copy(self), ::core::mem::transmute(key), &mut result__).from_abi::<IPortableDeviceValuesCollection>(result__)
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn RemoveValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::core::mem::transmute_copy(self), ::core::mem::transmute(key)).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn CopyValuesFromPropertyStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::core::mem::transmute_copy(self), pstore.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
    pub unsafe fn CopyValuesToPropertyStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>(&self, pstore: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::core::mem::transmute_copy(self), pstore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceValues {
    type Vtable = IPortableDeviceValues_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1749612274, 12629, 20358, [182, 245, 38, 62, 238, 171, 49, 67]);
}
impl ::core::convert::From<IPortableDeviceValues> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceValues) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceValues> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceValues) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceValues {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceValues {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValues_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcelt: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceValuesCollection(pub ::windows::runtime::IUnknown);
impl IPortableDeviceValuesCollection {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetCount(&self, pcelems: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelems)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetAt(&self, dwindex: u32) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, pvalues: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pvalues.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn RemoveAt(&self, dwindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwindex)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceValuesCollection {
    type Vtable = IPortableDeviceValuesCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1849634169, 19975, 18628, [130, 8, 216, 194, 229, 175, 74, 153]);
}
impl ::core::convert::From<IPortableDeviceValuesCollection> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceValuesCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceValuesCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceValuesCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceValuesCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceValuesCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcelems: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32, ppvalues: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalues: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwindex: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPortableDeviceWebControl(pub ::windows::runtime::IUnknown);
impl IPortableDeviceWebControl {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetDeviceFromId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, deviceid: Param0) -> ::windows::runtime::Result<super::super::System::Com::IDispatch> {
        let mut result__: <super::super::System::Com::IDispatch as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetDeviceFromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, deviceid: Param0, pcompletionhandler: Param1, perrorhandler: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), deviceid.into_param().abi(), pcompletionhandler.into_param().abi(), perrorhandler.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPortableDeviceWebControl {
    type Vtable = IPortableDeviceWebControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2499574099, 23713, 18490, [138, 238, 223, 82, 231, 116, 125, 0]);
}
impl ::core::convert::From<IPortableDeviceWebControl> for ::windows::runtime::IUnknown {
    fn from(value: IPortableDeviceWebControl) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPortableDeviceWebControl> for ::windows::runtime::IUnknown {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<IPortableDeviceWebControl> for super::super::System::Com::IDispatch {
    fn from(value: IPortableDeviceWebControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&IPortableDeviceWebControl> for super::super::System::Com::IDispatch {
    fn from(value: &IPortableDeviceWebControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Com::IDispatch> for &IPortableDeviceWebControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPortableDeviceWebControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Com::DISPPARAMS, pvarresult: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::core::mem::ManuallyDrop<super::super::System::Com::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcompletionhandler: ::windows::runtime::RawPtr, perrorhandler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRadioInstance(pub ::windows::runtime::IUnknown);
impl IRadioInstance {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetRadioManagerSignature(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetInstanceSignature(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn GetFriendlyName(&self, lcid: u32) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(lcid), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetRadioState(&self) -> ::windows::runtime::Result<DEVICE_RADIO_STATE> {
        let mut result__: <DEVICE_RADIO_STATE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DEVICE_RADIO_STATE>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn SetRadioState(&self, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(radiostate), ::core::mem::transmute(utimeoutsec)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn IsMultiComm(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_Foundation`*"]
    pub unsafe fn IsAssociatingDevice(&self) -> super::super::Foundation::BOOL {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IRadioInstance {
    type Vtable = IRadioInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1890196638, 62132, 19553, [134, 211, 107, 159, 183, 95, 209, 162]);
}
impl ::core::convert::From<IRadioInstance> for ::windows::runtime::IUnknown {
    fn from(value: IRadioInstance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRadioInstance> for ::windows::runtime::IUnknown {
    fn from(value: &IRadioInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRadioInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRadioInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidsignature: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcid: u32, pbstrname: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IRadioInstanceCollection(pub ::windows::runtime::IUnknown);
impl IRadioInstanceCollection {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetAt(&self, uindex: u32) -> ::windows::runtime::Result<IRadioInstance> {
        let mut result__: <IRadioInstance as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(uindex), &mut result__).from_abi::<IRadioInstance>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRadioInstanceCollection {
    type Vtable = IRadioInstanceCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3849920430, 22117, 19980, [149, 190, 95, 222, 49, 100, 65, 133]);
}
impl ::core::convert::From<IRadioInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: IRadioInstanceCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IRadioInstanceCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IRadioInstanceCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRadioInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRadioInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadioInstanceCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcinstance: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, ppradioinstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWpdSerializer(pub ::windows::runtime::IUnknown);
impl IWpdSerializer {
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetIPortableDeviceValuesFromBuffer(&self, pbuffer: *const u8, dwinputbufferlength: u32) -> ::windows::runtime::Result<IPortableDeviceValues> {
        let mut result__: <IPortableDeviceValues as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pbuffer), ::core::mem::transmute(dwinputbufferlength), &mut result__).from_abi::<IPortableDeviceValues>(result__)
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn WriteIPortableDeviceValuesToBuffer<'a, Param1: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, dwoutputbufferlength: u32, presults: Param1, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwoutputbufferlength), presults.into_param().abi(), ::core::mem::transmute(pbuffer), ::core::mem::transmute(pdwbyteswritten)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetBufferFromIPortableDeviceValues<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psource.into_param().abi(), ::core::mem::transmute(ppbuffer), ::core::mem::transmute(pdwbuffersize)).ok()
    }
    #[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
    pub unsafe fn GetSerializedSize<'a, Param0: ::windows::runtime::IntoParam<'a, IPortableDeviceValues>>(&self, psource: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), psource.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWpdSerializer {
    type Vtable = IWpdSerializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3006218242, 47911, 17919, [175, 79, 6, 99, 28, 30, 141, 173]);
}
impl ::core::convert::From<IWpdSerializer> for ::windows::runtime::IUnknown {
    fn from(value: IWpdSerializer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWpdSerializer> for ::windows::runtime::IUnknown {
    fn from(value: &IWpdSerializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWpdSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWpdSerializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWpdSerializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwoutputbufferlength: u32, presults: ::windows::runtime::RawPtr, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psource: ::windows::runtime::RawPtr, pdwsize: *mut u32) -> ::windows::runtime::HRESULT,
);
pub const PortableDevice: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1921655237, 15774, 18647, [152, 16, 134, 72, 72, 240, 244, 4]);
pub const PortableDeviceDispatchFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1126375987, 33592, 18008, [174, 1, 11, 74, 232, 48, 182, 176]);
pub const PortableDeviceFTM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4156556186, 18274, 18570, [180, 179, 118, 14, 249, 161, 186, 155]);
pub const PortableDeviceKeyCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3727491629, 9344, 17342, [151, 240, 209, 250, 44, 249, 143, 79]);
pub const PortableDeviceManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(183569644, 11981, 19346, [149, 129, 52, 246, 174, 6, 55, 243]);
pub const PortableDevicePropVariantCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(145333807, 28013, 19328, [175, 90, 186, 242, 188, 190, 76, 185]);
pub const PortableDeviceService: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4015895746, 37650, 16940, [145, 82, 65, 28, 217, 196, 221, 132]);
pub const PortableDeviceServiceFTM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(373928276, 51092, 18810, [155, 3, 243, 240, 18, 19, 2, 243]);
pub const PortableDeviceValues: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(202757379, 53271, 18382, [144, 22, 123, 63, 151, 135, 33, 204]);
pub const PortableDeviceValuesCollection: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(948048717, 5327, 16928, [156, 180, 67, 95, 134, 216, 63, 96]);
pub const PortableDeviceWebControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(409849900, 11756, 16821, [167, 212, 181, 144, 86, 250, 222, 81]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_MessageObj_PatternDayOfMonth: u32 = 31u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_MessageObj_PatternMonthOfYear: u32 = 12u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_BatteryLife: u32 = 100u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_MissedCalls: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_NewPictures: u32 = 65535u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_SignalStrength: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_TextMessages: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMAX_StatusSvc_VoiceMail: u32 = 255u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMIN_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMIN_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMIN_StatusSvc_BatteryLife: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGEMIN_StatusSvc_SignalStrength: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGESTEP_MessageObj_PatternDayOfMonth: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGESTEP_MessageObj_PatternMonthOfYear: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGESTEP_StatusSvc_BatteryLife: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const RANGESTEP_StatusSvc_SignalStrength: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SMS_MESSAGE_TYPES(pub i32);
pub const SMS_TEXT_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(0i32);
pub const SMS_BINARY_MESSAGE: SMS_MESSAGE_TYPES = SMS_MESSAGE_TYPES(1i32);
impl ::core::convert::From<i32> for SMS_MESSAGE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SMS_MESSAGE_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const SYNCSVC_FILTER_CALENDAR_WINDOW_WITH_RECURRENCE: u32 = 3u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const SYNCSVC_FILTER_CONTACTS_WITH_PHONE: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const SYNCSVC_FILTER_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const SYNCSVC_FILTER_TASK_ACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYSTEM_RADIO_STATE(pub i32);
pub const SRS_RADIO_ENABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(0i32);
pub const SRS_RADIO_DISABLED: SYSTEM_RADIO_STATE = SYSTEM_RADIO_STATE(1i32);
impl ::core::convert::From<i32> for SYSTEM_RADIO_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_RADIO_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_AnchorSyncSvc: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_CalendarSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_ContactsSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_DeviceMetadataSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_FullEnumSyncSvc: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_HintsSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_MessageSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_NotesSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_RingtonesSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_StatusSvc: u32 = 0u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const TYPE_TasksSvc: u32 = 0u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_HAS_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_HAS_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_HAS_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_HAS_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_HAS_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPDNSE_OBJECT_OPTIMAL_READ_BLOCK_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]),
    pid: 7u32,
};
pub const WPDNSE_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(886510601, 19271, 19840, [170, 172, 58, 40, 164, 163, 179, 230]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_CONTENT_DETAILS: u32 = 32u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_CONTENT_GENERAL: u32 = 4u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_CONTENT_REFERENCES: u32 = 8u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_CONTENT_RESOURCES: u32 = 16u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_DEVICE_GENERAL: u32 = 1u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPDNSE_PROPSHEET_STORAGE_GENERAL: u32 = 2u32;
pub const WPD_API_OPTIONS_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(283462206, 1325, 18295, [161, 60, 222, 118, 20, 190, 43, 196]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_API_OPTION_IOCTL_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(283462206, 1325, 18295, [161, 60, 222, 118, 20, 190, 43, 196]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_API_OPTION_USE_CLEAR_DATA_STREAM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(283462206, 1325, 18295, [161, 60, 222, 118, 20, 190, 43, 196]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_ACCEPTED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_DECLINED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 3u32,
};
pub const WPD_APPOINTMENT_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_OPTIONAL_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_REQUIRED_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_TENTATIVE_ATTENDEES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_APPOINTMENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4187946243, 17181, 16600, [161, 201, 78, 34, 13, 156, 136, 211]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_AUDIO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_AUDIO_BIT_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_AUDIO_BLOCK_ALIGNMENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_AUDIO_CHANNEL_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_AUDIO_FORMAT_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 11u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_BITRATE_TYPES(pub i32);
pub const WPD_BITRATE_TYPE_UNUSED: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(0i32);
pub const WPD_BITRATE_TYPE_DISCRETE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(1i32);
pub const WPD_BITRATE_TYPE_VARIABLE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(2i32);
pub const WPD_BITRATE_TYPE_FREE: WPD_BITRATE_TYPES = WPD_BITRATE_TYPES(3i32);
impl ::core::convert::From<i32> for WPD_BITRATE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_BITRATE_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_CAPTURE_MODES(pub i32);
pub const WPD_CAPTURE_MODE_UNDEFINED: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(0i32);
pub const WPD_CAPTURE_MODE_NORMAL: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(1i32);
pub const WPD_CAPTURE_MODE_BURST: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(2i32);
pub const WPD_CAPTURE_MODE_TIMELAPSE: WPD_CAPTURE_MODES = WPD_CAPTURE_MODES(3i32);
impl ::core::convert::From<i32> for WPD_CAPTURE_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_CAPTURE_MODES {
    type Abi = Self;
}
pub const WPD_CATEGORY_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]);
pub const WPD_CATEGORY_COMMON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]);
pub const WPD_CATEGORY_DEVICE_HINTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(224377131, 52038, 19535, [131, 67, 11, 195, 211, 241, 124, 132]);
pub const WPD_CATEGORY_MEDIA_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1504981946, 65092, 19853, [128, 140, 107, 203, 155, 15, 21, 232]);
pub const WPD_CATEGORY_MTP_EXT_VENDOR_OPERATIONS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]);
pub const WPD_CATEGORY_NETWORK_CONFIGURATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2029635324, 31160, 18236, [144, 96, 107, 210, 61, 208, 114, 196]);
pub const WPD_CATEGORY_NULL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
pub const WPD_CATEGORY_OBJECT_ENUMERATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]);
pub const WPD_CATEGORY_OBJECT_MANAGEMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]);
pub const WPD_CATEGORY_OBJECT_PROPERTIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]);
pub const WPD_CATEGORY_OBJECT_PROPERTIES_BULK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]);
pub const WPD_CATEGORY_OBJECT_RESOURCES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]);
pub const WPD_CATEGORY_SERVICE_CAPABILITIES: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]);
pub const WPD_CATEGORY_SERVICE_COMMON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(841942813, 14063, 18303, [180, 181, 111, 82, 215, 52, 186, 238]);
pub const WPD_CATEGORY_SERVICE_METHODS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]);
pub const WPD_CATEGORY_SMS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]);
pub const WPD_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1338861954, 8866, 19205, [164, 139, 98, 211, 139, 242, 123, 50]);
pub const WPD_CATEGORY_STORAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3640199078, 13516, 17914, [151, 251, 208, 7, 250, 71, 236, 148]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_DEVICE_IDENTIFICATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1043699162, 19825, 18942, [160, 180, 212, 64, 108, 58, 233, 63]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_DONT_REGISTER_WPD_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1661599727, 43132, 19623, [132, 52, 121, 117, 118, 228, 10, 150]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_MULTITRANSPORT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1043699162, 19825, 18942, [160, 180, 212, 64, 108, 58, 233, 63]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_REGISTER_WPD_PRIVATE_DEVICE_INTERFACE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1661599727, 43132, 19623, [132, 52, 121, 117, 118, 228, 10, 150]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_SILENCE_AUTOPLAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1707172088, 4967, 19682, [147, 157, 131, 16, 131, 159, 13, 48]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1661599727, 43132, 19623, [132, 52, 121, 117, 118, 228, 10, 150]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLASS_EXTENSION_OPTIONS_TRANSPORT_BANDWIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1043699162, 19825, 18942, [160, 180, 212, 64, 108, 58, 233, 63]),
    pid: 4u32,
};
pub const WPD_CLASS_EXTENSION_OPTIONS_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1661599727, 43132, 19623, [132, 52, 121, 117, 118, 228, 10, 150]);
pub const WPD_CLASS_EXTENSION_OPTIONS_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1043699162, 19825, 18942, [160, 180, 212, 64, 108, 58, 233, 63]);
pub const WPD_CLASS_EXTENSION_OPTIONS_V3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1707172088, 4967, 19682, [147, 157, 131, 16, 131, 159, 13, 48]);
pub const WPD_CLASS_EXTENSION_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(872090897, 25763, 20396, [180, 199, 61, 254, 170, 153, 176, 81]);
pub const WPD_CLASS_EXTENSION_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_DESIRED_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_EVENT_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 11u32,
};
pub const WPD_CLIENT_INFORMATION_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_MAJOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_MANUAL_CLOSE_ON_DISCONNECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_MINIMUM_RESULTS_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_MINOR_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_REVISION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_SECURITY_QUALITY_OF_SERVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_SHARE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_WMDRM_APPLICATION_CERTIFICATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CLIENT_WMDRM_APPLICATION_PRIVATE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(541957900, 8850, 16512, [159, 66, 64, 102, 78, 112, 248, 89]),
    pid: 6u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_COLOR_CORRECTED_STATUS_VALUES(pub i32);
pub const WPD_COLOR_CORRECTED_STATUS_NOT_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(0i32);
pub const WPD_COLOR_CORRECTED_STATUS_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(1i32);
pub const WPD_COLOR_CORRECTED_STATUS_SHOULD_NOT_BE_CORRECTED: WPD_COLOR_CORRECTED_STATUS_VALUES = WPD_COLOR_CORRECTED_STATUS_VALUES(2i32);
impl ::core::convert::From<i32> for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_COLOR_CORRECTED_STATUS_VALUES {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub struct WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    pub Command: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
    pub AccessType: u32,
    pub AccessProperty: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY,
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WPD_COMMAND_ACCESS_LOOKUP_ENTRY").field("Command", &self.Command).field("AccessType", &self.AccessType).field("AccessProperty", &self.AccessProperty).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.AccessType == other.AccessType && self.AccessProperty == other.AccessProperty
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::runtime::Abi for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_COMMAND_ACCESS_TYPES(pub i32);
pub const WPD_COMMAND_ACCESS_READ: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(1i32);
pub const WPD_COMMAND_ACCESS_READWRITE: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(3i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_STGM_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(4i32);
pub const WPD_COMMAND_ACCESS_FROM_PROPERTY_WITH_FILE_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(8i32);
pub const WPD_COMMAND_ACCESS_FROM_ATTRIBUTE_WITH_METHOD_ACCESS: WPD_COMMAND_ACCESS_TYPES = WPD_COMMAND_ACCESS_TYPES(16i32);
impl ::core::convert::From<i32> for WPD_COMMAND_ACCESS_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_COMMAND_ACCESS_TYPES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_FIXED_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CAPABILITIES_GET_SUPPORTED_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CLASS_EXTENSION_REGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CLASS_EXTENSION_UNREGISTER_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_CLASS_EXTENSION_WRITE_DEVICE_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(872090897, 25763, 20396, [180, 199, 61, 254, 170, 153, 176, 81]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_COMMIT_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2029635324, 31160, 18236, [144, 96, 107, 210, 61, 208, 114, 196]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_COMMON_GET_OBJECT_IDS_FROM_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_COMMON_RESET_DEVICE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_COMMON_SAVE_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_DEVICE_HINTS_GET_CONTENT_LOCATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(224377131, 52038, 19535, [131, 67, 11, 195, 211, 241, 124, 132]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_GENERATE_KEYPAIR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2029635324, 31160, 18236, [144, 96, 107, 210, 61, 208, 114, 196]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MEDIA_CAPTURE_PAUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1504981946, 65092, 19853, [128, 140, 107, 203, 155, 15, 21, 232]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MEDIA_CAPTURE_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1504981946, 65092, 19853, [128, 140, 107, 203, 155, 15, 21, 232]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MEDIA_CAPTURE_STOP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1504981946, 65092, 19853, [128, 140, 107, 203, 155, 15, 21, 232]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_END_DATA_TRANSFER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITHOUT_DATA_PHASE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_EXECUTE_COMMAND_WITH_DATA_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_GET_SUPPORTED_VENDOR_OPCODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_GET_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_READ_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_MTP_EXT_WRITE_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_ENUMERATION_END_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_ENUMERATION_FIND_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_ENUMERATION_START_FIND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COMMIT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_COPY_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_CREATE_OBJECT_WITH_PROPERTIES_ONLY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_DELETE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_MOVE_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_REVERT_OBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_UPDATE_OBJECT_WITH_PROPERTIES_AND_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_MANAGEMENT_WRITE_OBJECT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_FORMAT_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_GET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_END: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_NEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_BULK_SET_VALUES_BY_OBJECT_LIST_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ALL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_PROPERTIES_SET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_CLOSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_COMMIT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_CREATE_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_GET_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_OPEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_REVERT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_SEEK_IN_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_OBJECT_RESOURCES_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_PROCESS_WIRELESS_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2029635324, 31160, 18236, [144, 96, 107, 210, 61, 208, 114, 196]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_EVENT_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_FORMAT_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_METHOD_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_FORMAT_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_CAPABILITIES_GET_SUPPORTED_METHODS_BY_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_COMMON_GET_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(841942813, 14063, 18303, [180, 181, 111, 82, 215, 52, 186, 238]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_METHODS_CANCEL_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_METHODS_END_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SERVICE_METHODS_START_INVOKE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_SMS_SEND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_STILL_IMAGE_CAPTURE_INITIATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1338861954, 8866, 19205, [164, 139, 98, 211, 139, 242, 123, 50]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_STORAGE_EJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3640199078, 13516, 17914, [151, 251, 208, 7, 250, 71, 236, 148]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMAND_STORAGE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3640199078, 13516, 17914, [151, 251, 208, 7, 250, 71, 236, 148]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_BODY_TEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_END_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_NOTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 7u32,
};
pub const WPD_COMMON_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_PRIORITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_START_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_COMMON_INFORMATION_SUBJECT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2995448139, 1444, 20110, [190, 1, 114, 204, 126, 9, 157, 143]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_ANNIVERSARY_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 62u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_ASSISTANT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 61u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BIRTHDATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 57u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 45u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 40u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 41u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_BUSINESS_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 50u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_CHILDREN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 60u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 54u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_INSTANT_MESSENGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 51u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_INSTANT_MESSENGER2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 52u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_INSTANT_MESSENGER3: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 53u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_MIDDLE_NAMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_MOBILE_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 42u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_MOBILE_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 43u32,
};
pub const WPD_CONTACT_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_EMAILS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_PHONES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 47u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_POSTAL_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_OTHER_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PAGER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 46u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_EMAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_EMAIL2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 44u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_FULL_POSTAL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_PHONE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_CITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_COUNTRY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE1: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_LINE2: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_POSTAL_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_POSTAL_ADDRESS_REGION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PERSONAL_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PHONETIC_COMPANY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 55u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PHONETIC_FIRST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PHONETIC_LAST_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PREFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PRIMARY_EMAIL_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PRIMARY_FAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 58u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PRIMARY_PHONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_PRIMARY_WEB_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 48u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_RINGTONE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 63u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_ROLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 56u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_SPOUSE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 59u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_CONTACT_SUFFIX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4225039787, 39037, 18295, [179, 249, 114, 97, 133, 169, 49, 43]),
    pid: 7u32,
};
pub const WPD_CONTENT_TYPE_ALL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2162258130, 4181, 19006, [185, 82, 130, 204, 79, 138, 134, 137]);
pub const WPD_CONTENT_TYPE_APPOINTMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(267191822, 34707, 19230, [144, 201, 72, 172, 56, 154, 198, 49]);
pub const WPD_CONTENT_TYPE_AUDIO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1255327838, 24109, 17893, [136, 100, 79, 34, 158, 60, 108, 240]);
pub const WPD_CONTENT_TYPE_AUDIO_ALBUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2853729150, 20489, 18682, [174, 33, 133, 242, 67, 131, 180, 230]);
pub const WPD_CONTENT_TYPE_CALENDAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2717735271, 24611, 18848, [157, 241, 248, 6, 11, 231, 81, 176]);
pub const WPD_CONTENT_TYPE_CERTIFICATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3694687976, 43336, 16480, [144, 80, 203, 215, 126, 138, 61, 135]);
pub const WPD_CONTENT_TYPE_CONTACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3938091795, 17701, 18183, [159, 14, 135, 198, 128, 142, 148, 53]);
pub const WPD_CONTENT_TYPE_CONTACT_GROUP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(879462706, 19510, 16600, [148, 21, 24, 40, 41, 31, 157, 233]);
pub const WPD_CONTENT_TYPE_DOCUMENT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1745542994, 38154, 16449, [155, 65, 101, 227, 147, 100, 129, 85]);
pub const WPD_CONTENT_TYPE_EMAIL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2151154762, 32337, 20367, [136, 61, 29, 6, 35, 209, 69, 51]);
pub const WPD_CONTENT_TYPE_FOLDER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(669180818, 41233, 18656, [171, 12, 225, 119, 5, 160, 95, 133]);
pub const WPD_CONTENT_TYPE_FUNCTIONAL_OBJECT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2582446432, 6143, 19524, [157, 152, 29, 122, 111, 148, 25, 33]);
pub const WPD_CONTENT_TYPE_GENERIC_FILE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(8773798, 36148, 17879, [188, 92, 68, 126, 89, 199, 61, 72]);
pub const WPD_CONTENT_TYPE_GENERIC_MESSAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3893275384, 45787, 16691, [182, 126, 27, 239, 75, 74, 110, 95]);
pub const WPD_CONTENT_TYPE_IMAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4011919317, 42282, 16963, [162, 107, 98, 212, 23, 109, 118, 3]);
pub const WPD_CONTENT_TYPE_IMAGE_ALBUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1970876744, 5621, 18992, [168, 19, 84, 237, 138, 55, 226, 38]);
pub const WPD_CONTENT_TYPE_MEDIA_CAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1586017228, 15973, 20066, [191, 255, 34, 148, 149, 37, 58, 176]);
pub const WPD_CONTENT_TYPE_MEMO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2631012047, 15184, 16719, [166, 65, 228, 115, 255, 228, 87, 81]);
pub const WPD_CONTENT_TYPE_MIXED_CONTENT_ALBUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(15778732, 42387, 18860, [146, 25, 36, 171, 202, 90, 37, 99]);
pub const WPD_CONTENT_TYPE_NETWORK_ASSOCIATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(52275182, 6344, 16901, [132, 126, 137, 161, 18, 97, 208, 243]);
pub const WPD_CONTENT_TYPE_PLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(439613412, 44819, 18677, [153, 78, 119, 54, 157, 254, 4, 163]);
pub const WPD_CONTENT_TYPE_PROGRAM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3530160490, 9340, 19455, [152, 251, 151, 243, 196, 146, 32, 230]);
pub const WPD_CONTENT_TYPE_SECTION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2182121973, 7569, 19913, [190, 60, 187, 177, 179, 91, 24, 206]);
pub const WPD_CONTENT_TYPE_TASK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1663381292, 34943, 19638, [177, 172, 210, 152, 85, 220, 239, 108]);
pub const WPD_CONTENT_TYPE_TELEVISION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1621191119, 62126, 20001, [147, 117, 150, 119, 241, 28, 28, 110]);
pub const WPD_CONTENT_TYPE_UNSPECIFIED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(685298462, 9372, 17742, [170, 188, 52, 136, 49, 104, 230, 52]);
pub const WPD_CONTENT_TYPE_VIDEO: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2455875644, 15736, 17689, [133, 227, 2, 197, 225, 245, 11, 185]);
pub const WPD_CONTENT_TYPE_VIDEO_ALBUM: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(19598775, 54465, 17878, [176, 129, 148, 184, 119, 121, 97, 79]);
pub const WPD_CONTENT_TYPE_WIRELESS_PROFILE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(195823370, 40799, 19876, [168, 246, 61, 228, 77, 104, 253, 108]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
pub const WPD_CONTROL_FUNCTION_GENERIC_MESSAGE: u32 = 66u32;
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_CROPPED_STATUS_VALUES(pub i32);
pub const WPD_CROPPED_STATUS_NOT_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(0i32);
pub const WPD_CROPPED_STATUS_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(1i32);
pub const WPD_CROPPED_STATUS_SHOULD_NOT_BE_CROPPED: WPD_CROPPED_STATUS_VALUES = WPD_CROPPED_STATUS_VALUES(2i32);
impl ::core::convert::From<i32> for WPD_CROPPED_STATUS_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_CROPPED_STATUS_VALUES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_DATETIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_EDP_IDENTITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1814792076, 49900, 18701, [180, 37, 215, 167, 94, 35, 229, 237]),
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_FIRMWARE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_FRIENDLY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_FUNCTIONAL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1178457698, 32708, 17041, [145, 28, 127, 76, 156, 202, 151, 153]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_MODEL_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1178457698, 32708, 17041, [145, 28, 127, 76, 156, 202, 151, 153]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_NETWORK_IDENTIFIER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_POWER_LEVEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_POWER_SOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 5u32,
};
pub const WPD_DEVICE_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]);
pub const WPD_DEVICE_PROPERTIES_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1178457698, 32708, 17041, [145, 28, 127, 76, 156, 202, 151, 153]);
pub const WPD_DEVICE_PROPERTIES_V3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1814792076, 49900, 18701, [180, 37, 215, 167, 94, 35, 229, 237]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_PROTOCOL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_SUPPORTED_DRM_SCHEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_SUPPORTED_FORMATS_ARE_ORDERED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_SUPPORTS_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_SYNC_PARTNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_TRANSPORT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1178457698, 32708, 17041, [145, 28, 127, 76, 156, 202, 151, 153]),
    pid: 4u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_DEVICE_TRANSPORTS(pub i32);
pub const WPD_DEVICE_TRANSPORT_UNSPECIFIED: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(0i32);
pub const WPD_DEVICE_TRANSPORT_USB: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(1i32);
pub const WPD_DEVICE_TRANSPORT_IP: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(2i32);
pub const WPD_DEVICE_TRANSPORT_BLUETOOTH: WPD_DEVICE_TRANSPORTS = WPD_DEVICE_TRANSPORTS(3i32);
impl ::core::convert::From<i32> for WPD_DEVICE_TRANSPORTS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_DEVICE_TRANSPORTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(651466650, 58947, 17958, [158, 43, 115, 109, 192, 201, 47, 220]),
    pid: 15u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_DEVICE_TYPES(pub i32);
pub const WPD_DEVICE_TYPE_GENERIC: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(0i32);
pub const WPD_DEVICE_TYPE_CAMERA: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(1i32);
pub const WPD_DEVICE_TYPE_MEDIA_PLAYER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(2i32);
pub const WPD_DEVICE_TYPE_PHONE: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(3i32);
pub const WPD_DEVICE_TYPE_VIDEO: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(4i32);
pub const WPD_DEVICE_TYPE_PERSONAL_INFORMATION_MANAGER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(5i32);
pub const WPD_DEVICE_TYPE_AUDIO_RECORDER: WPD_DEVICE_TYPES = WPD_DEVICE_TYPES(6i32);
impl ::core::convert::From<i32> for WPD_DEVICE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_DEVICE_TYPES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_DEVICE_USE_DEVICE_STAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1178457698, 32708, 17041, [145, 28, 127, 76, 156, 202, 151, 153]),
    pid: 5u32,
};
pub const WPD_DOCUMENT_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(185664003, 60309, 20226, [147, 224, 151, 198, 49, 73, 58, 213]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_EFFECT_MODES(pub i32);
pub const WPD_EFFECT_MODE_UNDEFINED: WPD_EFFECT_MODES = WPD_EFFECT_MODES(0i32);
pub const WPD_EFFECT_MODE_COLOR: WPD_EFFECT_MODES = WPD_EFFECT_MODES(1i32);
pub const WPD_EFFECT_MODE_BLACK_AND_WHITE: WPD_EFFECT_MODES = WPD_EFFECT_MODES(2i32);
pub const WPD_EFFECT_MODE_SEPIA: WPD_EFFECT_MODES = WPD_EFFECT_MODES(3i32);
impl ::core::convert::From<i32> for WPD_EFFECT_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_EFFECT_MODES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_BCC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_CC_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_HAS_ATTACHMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_HAS_BEEN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 7u32,
};
pub const WPD_EMAIL_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_RECEIVED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_SENDER_ADDRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EMAIL_TO_LINE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1106835034, 21636, 18306, [177, 61, 71, 64, 221, 124, 55, 197]),
    pid: 2u32,
};
pub const WPD_EVENT_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(281634168, 11905, 16657, [173, 222, 224, 140, 166, 19, 143, 109]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(281634168, 11905, 16657, [173, 222, 224, 140, 166, 19, 143, 109]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_ATTRIBUTE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(281634168, 11905, 16657, [173, 222, 224, 140, 166, 19, 143, 109]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(281634168, 11905, 16657, [173, 222, 224, 140, 166, 19, 143, 109]),
    pid: 3u32,
};
pub const WPD_EVENT_DEVICE_CAPABILITIES_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(914905761, 52564, 19882, [179, 208, 175, 179, 224, 63, 89, 153]);
pub const WPD_EVENT_DEVICE_REMOVED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3838560795, 26904, 18617, [133, 238, 2, 190, 124, 133, 10, 249]);
pub const WPD_EVENT_DEVICE_RESET: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2002112339, 49645, 17651, [181, 162, 69, 30, 44, 55, 107, 39]);
pub const WPD_EVENT_MTP_VENDOR_EXTENDED_EVENTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(0, 22328, 20466, [132, 69, 190, 49, 38, 105, 16, 89]);
pub const WPD_EVENT_NOTIFICATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(732095498, 27468, 17045, [187, 67, 38, 50, 43, 153, 174, 178]);
pub const WPD_EVENT_OBJECT_ADDED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2804341397, 57863, 19202, [141, 68, 190, 242, 232, 108, 191, 252]);
pub const WPD_EVENT_OBJECT_REMOVED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3196234632, 42284, 18467, [150, 229, 208, 39, 38, 113, 252, 56]);
pub const WPD_EVENT_OBJECT_TRANSFER_REQUESTED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2367070369, 62150, 16858, [143, 25, 94, 83, 114, 26, 219, 242]);
pub const WPD_EVENT_OBJECT_UPDATED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(340109145, 11777, 18525, [159, 39, 255, 7, 218, 230, 151, 171]);
pub const WPD_EVENT_OPTIONS_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3017333463, 41825, 19331, [138, 72, 91, 2, 206, 16, 113, 59]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_OPTION_IS_AUTOPLAY_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3017333463, 41825, 19331, [138, 72, 91, 2, 206, 16, 113, 59]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_OPTION_IS_BROADCAST_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3017333463, 41825, 19331, [138, 72, 91, 2, 206, 16, 113, 59]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_CHILD_HIERARCHY_CHANGED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_EVENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_OBJECT_CREATION_COOKIE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_OBJECT_PARENT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_OPERATION_PROGRESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_OPERATION_STATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_PNP_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_EVENT_PARAMETER_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1384151946, 18708, 17187, [155, 154, 116, 246, 84, 178, 184, 70]),
    pid: 2u32,
};
pub const WPD_EVENT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(363534675, 63511, 20463, [169, 33, 86, 118, 232, 56, 246, 224]);
pub const WPD_EVENT_PROPERTIES_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1384151946, 18708, 17187, [155, 154, 116, 246, 84, 178, 184, 70]);
pub const WPD_EVENT_SERVICE_METHOD_COMPLETE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2318661112, 2764, 19867, [156, 196, 17, 45, 53, 59, 134, 202]);
pub const WPD_EVENT_STORAGE_FORMAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931291499, 8892, 17524, [162, 81, 48, 112, 248, 211, 136, 87]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_EXPOSURE_METERING_MODES(pub i32);
pub const WPD_EXPOSURE_METERING_MODE_UNDEFINED: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(0i32);
pub const WPD_EXPOSURE_METERING_MODE_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(1i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_WEIGHTED_AVERAGE: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(2i32);
pub const WPD_EXPOSURE_METERING_MODE_MULTI_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(3i32);
pub const WPD_EXPOSURE_METERING_MODE_CENTER_SPOT: WPD_EXPOSURE_METERING_MODES = WPD_EXPOSURE_METERING_MODES(4i32);
impl ::core::convert::From<i32> for WPD_EXPOSURE_METERING_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_EXPOSURE_METERING_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_EXPOSURE_PROGRAM_MODES(pub i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_UNDEFINED: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(0i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_MANUAL: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(1i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_AUTO: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(2i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_APERTURE_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(3i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_SHUTTER_PRIORITY: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(4i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_CREATIVE: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(5i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_ACTION: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(6i32);
pub const WPD_EXPOSURE_PROGRAM_MODE_PORTRAIT: WPD_EXPOSURE_PROGRAM_MODES = WPD_EXPOSURE_PROGRAM_MODES(7i32);
impl ::core::convert::From<i32> for WPD_EXPOSURE_PROGRAM_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_EXPOSURE_PROGRAM_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_FLASH_MODES(pub i32);
pub const WPD_FLASH_MODE_UNDEFINED: WPD_FLASH_MODES = WPD_FLASH_MODES(0i32);
pub const WPD_FLASH_MODE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(1i32);
pub const WPD_FLASH_MODE_OFF: WPD_FLASH_MODES = WPD_FLASH_MODES(2i32);
pub const WPD_FLASH_MODE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(3i32);
pub const WPD_FLASH_MODE_RED_EYE_AUTO: WPD_FLASH_MODES = WPD_FLASH_MODES(4i32);
pub const WPD_FLASH_MODE_RED_EYE_FILL: WPD_FLASH_MODES = WPD_FLASH_MODES(5i32);
pub const WPD_FLASH_MODE_EXTERNAL_SYNC: WPD_FLASH_MODES = WPD_FLASH_MODES(6i32);
impl ::core::convert::From<i32> for WPD_FLASH_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_FLASH_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_FOCUS_METERING_MODES(pub i32);
pub const WPD_FOCUS_METERING_MODE_UNDEFINED: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(0i32);
pub const WPD_FOCUS_METERING_MODE_CENTER_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(1i32);
pub const WPD_FOCUS_METERING_MODE_MULTI_SPOT: WPD_FOCUS_METERING_MODES = WPD_FOCUS_METERING_MODES(2i32);
impl ::core::convert::From<i32> for WPD_FOCUS_METERING_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_FOCUS_METERING_MODES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_FOCUS_MODES(pub i32);
pub const WPD_FOCUS_UNDEFINED: WPD_FOCUS_MODES = WPD_FOCUS_MODES(0i32);
pub const WPD_FOCUS_MANUAL: WPD_FOCUS_MODES = WPD_FOCUS_MODES(1i32);
pub const WPD_FOCUS_AUTOMATIC: WPD_FOCUS_MODES = WPD_FOCUS_MODES(2i32);
pub const WPD_FOCUS_AUTOMATIC_MACRO: WPD_FOCUS_MODES = WPD_FOCUS_MODES(3i32);
impl ::core::convert::From<i32> for WPD_FOCUS_MODES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_FOCUS_MODES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_FOLDER_CONTENT_TYPES_ALLOWED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2124053183, 58728, 19252, [170, 47, 19, 187, 18, 171, 23, 125]),
    pid: 2u32,
};
pub const WPD_FOLDER_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2124053183, 58728, 19252, [170, 47, 19, 187, 18, 171, 23, 125]);
pub const WPD_FORMAT_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2694848512, 48303, 19432, [179, 245, 35, 63, 35, 28, 245, 143]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_FORMAT_ATTRIBUTE_MIMETYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2694848512, 48303, 19432, [179, 245, 35, 63, 35, 28, 245, 143]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_FORMAT_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2694848512, 48303, 19432, [179, 245, 35, 63, 35, 28, 245, 143]),
    pid: 2u32,
};
pub const WPD_FUNCTIONAL_CATEGORY_ALL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(764044562, 42828, 17550, [186, 138, 244, 172, 7, 196, 147, 153]);
pub const WPD_FUNCTIONAL_CATEGORY_AUDIO_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1059723545, 51138, 18944, [133, 93, 245, 124, 240, 109, 235, 187]);
pub const WPD_FUNCTIONAL_CATEGORY_DEVICE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(149571179, 58276, 17206, [161, 243, 164, 77, 43, 92, 67, 140]);
pub const WPD_FUNCTIONAL_CATEGORY_NETWORK_CONFIGURATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1224006514, 31850, 19120, [158, 26, 71, 14, 60, 219, 242, 106]);
pub const WPD_FUNCTIONAL_CATEGORY_RENDERING_INFORMATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(140512164, 42938, 18945, [171, 14, 0, 101, 208, 163, 86, 211]);
pub const WPD_FUNCTIONAL_CATEGORY_SMS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4497585, 49641, 19197, [179, 88, 166, 44, 97, 23, 201, 207]);
pub const WPD_FUNCTIONAL_CATEGORY_STILL_IMAGE_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1631363879, 43923, 18688, [180, 250, 137, 91, 181, 135, 75, 121]);
pub const WPD_FUNCTIONAL_CATEGORY_STORAGE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(602954684, 5598, 19498, [165, 91, 169, 175, 92, 228, 18, 239]);
pub const WPD_FUNCTIONAL_CATEGORY_VIDEO_CAPTURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3795738475, 29251, 17322, [141, 241, 14, 179, 217, 104, 169, 24]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_FUNCTIONAL_OBJECT_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2399481235, 43978, 20421, [165, 172, 176, 29, 244, 219, 229, 152]),
    pid: 2u32,
};
pub const WPD_FUNCTIONAL_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2399481235, 43978, 20421, [165, 172, 176, 29, 244, 219, 229, 152]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_BITDEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_COLOR_CORRECTED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_CROPPED_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_HORIZONTAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 9u32,
};
pub const WPD_IMAGE_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_IMAGE_VERTICAL_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1674987784, 40865, 18335, [133, 186, 153, 82, 33, 100, 71, 219]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_ALBUM_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_AUDIO_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 49u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_BITRATE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_BUY_NOW: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_BYTE_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 36u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_COMPOSER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_COPYRIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_DESTINATION_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_DURATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_ENCODING_PROFILE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_GUID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 38u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_LAST_ACCESSED_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_LAST_BUILD_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 35u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_MANAGING_EDITOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_META_GENRE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_OBJECT_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 34u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_PARENTAL_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 9u32,
};
pub const WPD_MEDIA_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_RELEASE_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SAMPLE_RATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SKIP_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SOURCE_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_STAR_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SUBSCRIPTION_CONTENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SUB_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 39u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_SUB_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_TIME_BOOKMARK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 33u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_TIME_TO_LIVE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 37u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_TITLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_TOTAL_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_USER_EFFECTIVE_RATING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_USE_COUNT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_WEBMASTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MEDIA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(785955333, 2771, 17116, [176, 208, 188, 149, 172, 57, 106, 200]),
    pid: 22u32,
};
pub const WPD_MEMO_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1610349691, 29827, 16813, [175, 185, 218, 63, 78, 89, 43, 141]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_META_GENRES(pub i32);
pub const WPD_META_GENRE_UNUSED: WPD_META_GENRES = WPD_META_GENRES(0i32);
pub const WPD_META_GENRE_GENERIC_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(1i32);
pub const WPD_META_GENRE_GENERIC_NON_MUSIC_AUDIO_FILE: WPD_META_GENRES = WPD_META_GENRES(17i32);
pub const WPD_META_GENRE_SPOKEN_WORD_AUDIO_BOOK_FILES: WPD_META_GENRES = WPD_META_GENRES(18i32);
pub const WPD_META_GENRE_SPOKEN_WORD_FILES_NON_AUDIO_BOOK: WPD_META_GENRES = WPD_META_GENRES(19i32);
pub const WPD_META_GENRE_SPOKEN_WORD_NEWS: WPD_META_GENRES = WPD_META_GENRES(20i32);
pub const WPD_META_GENRE_SPOKEN_WORD_TALK_SHOWS: WPD_META_GENRES = WPD_META_GENRES(21i32);
pub const WPD_META_GENRE_GENERIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(33i32);
pub const WPD_META_GENRE_NEWS_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(34i32);
pub const WPD_META_GENRE_MUSIC_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(35i32);
pub const WPD_META_GENRE_HOME_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(36i32);
pub const WPD_META_GENRE_FEATURE_FILM_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(37i32);
pub const WPD_META_GENRE_TELEVISION_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(38i32);
pub const WPD_META_GENRE_TRAINING_EDUCATIONAL_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(39i32);
pub const WPD_META_GENRE_PHOTO_MONTAGE_VIDEO_FILE: WPD_META_GENRES = WPD_META_GENRES(40i32);
pub const WPD_META_GENRE_GENERIC_NON_AUDIO_NON_VIDEO: WPD_META_GENRES = WPD_META_GENRES(48i32);
pub const WPD_META_GENRE_AUDIO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(64i32);
pub const WPD_META_GENRE_VIDEO_PODCAST: WPD_META_GENRES = WPD_META_GENRES(65i32);
pub const WPD_META_GENRE_MIXED_PODCAST: WPD_META_GENRES = WPD_META_GENRES(66i32);
impl ::core::convert::From<i32> for WPD_META_GENRES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_META_GENRES {
    type Abi = Self;
}
pub const WPD_METHOD_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4051325041, 61497, 17583, [142, 254, 67, 44, 243, 46, 67, 42]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_METHOD_ATTRIBUTE_ACCESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4051325041, 61497, 17583, [142, 254, 67, 44, 243, 46, 67, 42]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_METHOD_ATTRIBUTE_ASSOCIATED_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4051325041, 61497, 17583, [142, 254, 67, 44, 243, 46, 67, 42]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_METHOD_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4051325041, 61497, 17583, [142, 254, 67, 44, 243, 46, 67, 42]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_METHOD_ATTRIBUTE_PARAMETERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4051325041, 61497, 17583, [142, 254, 67, 44, 243, 46, 67, 42]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MUSIC_ALBUM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MUSIC_LYRICS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MUSIC_MOOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 8u32,
};
pub const WPD_MUSIC_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_MUSIC_TRACK: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3005543786, 56413, 18149, [182, 223, 210, 234, 65, 72, 136, 198]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_NETWORK_ASSOCIATION_HOST_NETWORK_IDENTIFIERS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3838393375, 45571, 17393, [161, 0, 90, 7, 209, 27, 2, 116]),
    pid: 2u32,
};
pub const WPD_NETWORK_ASSOCIATION_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3838393375, 45571, 17393, [161, 0, 90, 7, 209, 27, 2, 116]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_NETWORK_ASSOCIATION_X509V3SEQUENCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3838393375, 45571, 17393, [161, 0, 90, 7, 209, 27, 2, 116]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_BACK_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_CONTAINER_FUNCTIONAL_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_DATE_AUTHORED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_DATE_CREATED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_DATE_MODIFIED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 6u32,
};
pub const WPD_OBJECT_FORMAT_3G2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112501248, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_3G2A: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(437329965, 34649, 20020, [186, 94, 177, 33, 16, 135, 238, 228]);
pub const WPD_OBJECT_FORMAT_3GP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112435712, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_3GPA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3843499824, 63857, 16879, [161, 11, 34, 113, 160, 1, 157, 122]);
pub const WPD_OBJECT_FORMAT_AAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3103981568, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3145793536, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ABSTRACT_CONTACT_GROUP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3120955392, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ABSTRACT_MEDIA_CAST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121283072, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_AIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805765120, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ALL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3254136498, 19379, 18332, [156, 250, 5, 181, 243, 165, 123, 34]);
pub const WPD_OBJECT_FORMAT_AMR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3104309248, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ASF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(806092800, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ASXPLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121807360, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ATSCTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112632320, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_AUDIBLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3104047104, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_AVCHD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112566784, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_AVI: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805961728, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_BMP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939786240, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_CIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939851776, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_DPOF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805699584, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_DVBTS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112697856, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_EXECUTABLE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805502976, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_EXIF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939589632, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_FLAC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3104178176, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_FLASHPIX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939720704, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_GIF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939982848, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_HTML: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805634048, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ICALENDAR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3187867648, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_ICON: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(124924653, 4140, 17976, [156, 34, 131, 241, 66, 191, 200, 34]);
pub const WPD_OBJECT_FORMAT_JFIF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940048384, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_JP2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940507136, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_JPEGXR: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3087269888, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_JPX: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940572672, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_M3UPLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121676288, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_M4A: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(816555948, 28669, 19491, [163, 89, 62, 155, 82, 243, 241, 200]);
pub const WPD_OBJECT_FORMAT_MHT_COMPILED_HTML: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129212928, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MICROSOFT_EXCEL: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129278464, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MICROSOFT_POWERPOINT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129344000, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WFC: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2969829376, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MICROSOFT_WORD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129147392, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MKV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3113222144, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MP2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112370176, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MP3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805896192, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MP4: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112304640, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MPEG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(806027264, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_MPLPLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121741824, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_NETWORK_ASSOCIATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2969698304, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_OGG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3103916032, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_PCD: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940113920, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_PICT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940179456, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_PLSPLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121872896, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_PNG: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940244992, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_PROPERTIES_ONLY: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805371904, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_QCELP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3104243712, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_SCRIPT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805437440, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_TEXT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805568512, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_TIFF: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940376064, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_TIFFEP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(939655168, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_TIFFIT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(940441600, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_UNSPECIFIED: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805306368, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_VCALENDAR1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3187802112, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_VCARD2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3145859072, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_VCARD3: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3145924608, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WAVE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(805830656, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WBMP: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3087204352, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WINDOWSIMAGEFORMAT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3095461888, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WMA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3103850496, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WMV: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3112239104, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_WPLPLAYLIST: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3121610752, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_X509V3CERTIFICATE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2969763840, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
pub const WPD_OBJECT_FORMAT_XML: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3129081856, 44652, 18436, [152, 186, 197, 123, 70, 150, 95, 231]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_GENERATE_THUMBNAIL_FROM_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_HINT_LOCATION_DISPLAY_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_ISHIDDEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_ISSYSTEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_IS_DRM_PROTECTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_KEYWORDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_LANGUAGE_LOCALE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_NON_CONSUMABLE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_ORIGINAL_FILE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_PERSISTENT_UNIQUE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 5u32,
};
pub const WPD_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]);
pub const WPD_OBJECT_PROPERTIES_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(57920829, 19014, 16599, [180, 216, 115, 232, 218, 116, 231, 117]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_REFERENCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_SUPPORTED_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(57920829, 19014, 16599, [180, 216, 115, 232, 218, 116, 231, 117]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OBJECT_SYNC_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4016785677, 23768, 17274, [175, 252, 218, 139, 96, 238, 74, 60]),
    pid: 16u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_OPERATION_STATES(pub i32);
pub const WPD_OPERATION_STATE_UNSPECIFIED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(0i32);
pub const WPD_OPERATION_STATE_STARTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(1i32);
pub const WPD_OPERATION_STATE_RUNNING: WPD_OPERATION_STATES = WPD_OPERATION_STATES(2i32);
pub const WPD_OPERATION_STATE_PAUSED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(3i32);
pub const WPD_OPERATION_STATE_CANCELLED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(4i32);
pub const WPD_OPERATION_STATE_FINISHED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(5i32);
pub const WPD_OPERATION_STATE_ABORTED: WPD_OPERATION_STATES = WPD_OPERATION_STATES(6i32);
impl ::core::convert::From<i32> for WPD_OPERATION_STATES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_OPERATION_STATES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_OBJECT_MANAGEMENT_RECURSIVE_DELETE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_OBJECT_RESOURCES_NO_INPUT_BUFFER_ON_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 5003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_READ_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_OBJECT_RESOURCES_SEEK_ON_WRITE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 5002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_SMS_BINARY_MESSAGE_SUPPORTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 5001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_OPTION_VALID_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 5001u32,
};
pub const WPD_PARAMETER_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_ORDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_USAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PARAMETER_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3867561431, 62245, 17898, [161, 213, 151, 207, 115, 182, 202, 88]),
    pid: 12u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_PARAMETER_USAGE_TYPES(pub i32);
pub const WPD_PARAMETER_USAGE_RETURN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(0i32);
pub const WPD_PARAMETER_USAGE_IN: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(1i32);
pub const WPD_PARAMETER_USAGE_OUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(2i32);
pub const WPD_PARAMETER_USAGE_INOUT: WPD_PARAMETER_USAGE_TYPES = WPD_PARAMETER_USAGE_TYPES(3i32);
impl ::core::convert::From<i32> for WPD_PARAMETER_USAGE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_PARAMETER_USAGE_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_POWER_SOURCES(pub i32);
pub const WPD_POWER_SOURCE_BATTERY: WPD_POWER_SOURCES = WPD_POWER_SOURCES(0i32);
pub const WPD_POWER_SOURCE_EXTERNAL: WPD_POWER_SOURCES = WPD_POWER_SOURCES(1i32);
impl ::core::convert::From<i32> for WPD_POWER_SOURCES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_POWER_SOURCES {
    type Abi = Self;
}
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_DEVICE_PROPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297371224, 35072, 16563, [143, 29, 220, 36, 110, 30, 131, 112]);
pub const WPD_PROPERTIES_MTP_VENDOR_EXTENDED_OBJECT_PROPS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297371224, 20430, 17784, [149, 200, 134, 152, 169, 188, 15, 73]);
pub const WPD_PROPERTY_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]);
pub const WPD_PROPERTY_ATTRIBUTES_V2: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1570611552, 29870, 17356, [133, 169, 254, 85, 90, 128, 121, 142]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_DEFAULT_VALUE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_ENUMERATION_ELEMENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FAST_PROPERTY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_FORM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_MAX_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1570611552, 29870, 17356, [133, 169, 254, 85, 90, 128, 121, 142]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MAX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_MIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_RANGE_STEP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_REGULAR_EXPRESSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2876851160, 25394, 17503, [160, 13, 141, 94, 241, 233, 111, 55]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_ATTRIBUTE_VARTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1570611552, 29870, 17356, [133, 169, 254, 85, 90, 128, 121, 142]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_CONTENT_TYPES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_EVENT_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_FUNCTIONAL_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(212593784, 27508, 16838, [146, 22, 38, 57, 209, 252, 227, 86]),
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(872090897, 25763, 20396, [180, 199, 61, 254, 170, 153, 176, 81]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CLASS_EXTENSION_DEVICE_INFORMATION_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(872090897, 25763, 20396, [180, 199, 61, 254, 170, 153, 176, 81]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_INTERFACES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_CLASS_EXTENSION_SERVICE_REGISTRATION_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2131196341, 64043, 18278, [156, 178, 247, 59, 163, 11, 103, 88]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_ACTIVITY_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_CLIENT_INFORMATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_COMMAND_CATEGORY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_COMMAND_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_COMMAND_TARGET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_DRIVER_ERROR_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_COMMON_PERSISTENT_UNIQUE_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4030868124, 24008, 17472, [181, 189, 93, 242, 136, 53, 101, 138]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_LOCATIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(224377131, 52038, 19535, [131, 67, 11, 195, 211, 241, 124, 132]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_DEVICE_HINTS_CONTENT_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(224377131, 52038, 19535, [131, 67, 11, 195, 211, 241, 124, 132]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_EVENT_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 61320, 20045, [149, 195, 79, 50, 127, 114, 138, 150]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_OPERATION_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_RESPONSE_PARAMS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_TRANSFER_TOTAL_DATA_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_EXTENSION_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_MTP_EXT_VENDOR_OPERATION_CODES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1297371224, 6702, 16646, [163, 87, 119, 30, 8, 25, 252, 86]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_NULL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]), pid: 0u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_FILTER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_NUM_OBJECTS_REQUESTED: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_ENUMERATION_PARENT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3074903697, 59384, 19161, [180, 0, 173, 26, 75, 88, 238, 236]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_COPY_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_CREATION_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_DESTINATION_FOLDER_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_MOVE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_MANAGEMENT_UPDATE_PROPERTIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4011738077, 43501, 17217, [139, 204, 24, 97, 146, 174, 160, 137]),
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_OBJECT_IDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PARENT_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_BULK_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(298329309, 1229, 20046, [140, 123, 246, 239, 183, 148, 216, 78]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_DELETE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_PROPERTIES_PROPERTY_WRITE_RESULTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2656404196, 2068, 17638, [152, 26, 178, 153, 141, 88, 56, 4]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_ACCESS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_DATA: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_TO_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_NUM_BYTES_WRITTEN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_OPTIMAL_TRANSFER_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_POSITION_FROM_START: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_RESOURCE_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SEEK_ORIGIN_FLAG: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_STREAM_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_OBJECT_RESOURCES_SUPPORTS_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3013784109, 42389, 16648, [190, 10, 252, 60, 150, 95, 61, 74]),
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_PUBLIC_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2029635324, 31160, 18236, [144, 96, 107, 210, 61, 208, 114, 196]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1018u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_COMMAND_OPTIONS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1019u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1012u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_EVENT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1013u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1007u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_FORMAT_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1008u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITANCE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1014u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_INHERITED_SERVICES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1015u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_METHOD_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PARAMETER_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1006u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_ATTRIBUTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1010u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_PROPERTY_KEYS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1009u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_RENDERING_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1016u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_COMMANDS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1017u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_EVENTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1011u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_CAPABILITIES_SUPPORTED_METHODS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(608534132, 11935, 17657, [140, 87, 29, 27, 203, 23, 11, 137]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_METHOD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_METHOD_CONTEXT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_METHOD_HRESULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 1005u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_METHOD_PARAMETER_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_METHOD_RESULT_VALUES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(760356008, 49584, 17000, [163, 66, 207, 25, 50, 21, 105, 188]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SERVICE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(841942813, 14063, 18303, [180, 181, 111, 82, 215, 52, 186, 238]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SMS_BINARY_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 1004u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SMS_MESSAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SMS_RECIPIENT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 1001u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_SMS_TEXT_MESSAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2948750694, 65037, 16660, [144, 151, 151, 12, 147, 233, 32, 209]),
    pid: 1003u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_STORAGE_DESTINATION_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3640199078, 13516, 17914, [151, 251, 208, 7, 250, 71, 236, 148]),
    pid: 1002u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_PROPERTY_STORAGE_OBJECT_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3640199078, 13516, 17914, [151, 251, 208, 7, 250, 71, 236, 148]),
    pid: 1001u32,
};
pub const WPD_RENDERING_INFORMATION_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309110175, 60963, 18993, [133, 144, 118, 57, 135, 152, 112, 180]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RENDERING_INFORMATION_PROFILES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3309110175, 60963, 18993, [133, 144, 118, 57, 135, 152, 112, 180]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_CREATABLE_RESOURCES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3309110175, 60963, 18993, [133, 144, 118, 57, 135, 152, 112, 180]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3309110175, 60963, 18993, [133, 144, 118, 57, 135, 152, 112, 180]),
    pid: 3u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(pub i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_OBJECT: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(0i32);
pub const WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPE_RESOURCE: WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES = WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES(1i32);
impl ::core::convert::From<i32> for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ALBUM_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4029326164, 8960, 20013, [161, 185, 59, 103, 48, 247, 250, 33]),
    pid: 0u32,
};
pub const WPD_RESOURCE_ATTRIBUTES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_DELETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_READ: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_CAN_WRITE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_READ_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_OPTIMAL_WRITE_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_RESOURCE_KEY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ATTRIBUTE_TOTAL_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(515307012, 37496, 17055, [147, 204, 91, 184, 192, 102, 86, 182]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_AUDIO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1002518914, 34225, 18656, [149, 166, 141, 58, 208, 107, 225, 23]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_BRANDING_ART: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3056841134, 27823, 19079, [149, 137, 34, 222, 214, 221, 88, 153]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_CONTACT_PHOTO: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(743270403, 33002, 17792, [175, 154, 91, 225, 162, 62, 221, 203]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_DEFAULT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3894311358, 13552, 16831, [181, 63, 241, 160, 106, 232, 120, 66]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_GENERIC: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3115971861, 47728, 17991, [148, 220, 250, 73, 37, 233, 90, 7]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(4053139160, 43560, 20195, [177, 83, 225, 130, 221, 94, 220, 57]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_THUMBNAIL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3351513018, 39162, 18101, [153, 96, 35, 254, 193, 36, 207, 222]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_RESOURCE_VIDEO_CLIP: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3043421762, 25448, 17040, [134, 98, 112, 24, 47, 183, 159, 32]),
    pid: 0u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SECTION_DATA_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1365966123, 50766, 17648, [152, 220, 190, 225, 200, 143, 125, 102]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SECTION_DATA_OFFSET: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1365966123, 50766, 17648, [152, 220, 190, 225, 200, 143, 125, 102]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SECTION_DATA_REFERENCED_OBJECT_RESOURCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1365966123, 50766, 17648, [152, 220, 190, 225, 200, 143, 125, 102]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SECTION_DATA_UNITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1365966123, 50766, 17648, [152, 220, 190, 225, 200, 143, 125, 102]),
    pid: 4u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_SECTION_DATA_UNITS_VALUES(pub i32);
pub const WPD_SECTION_DATA_UNITS_BYTES: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(0i32);
pub const WPD_SECTION_DATA_UNITS_MILLISECONDS: WPD_SECTION_DATA_UNITS_VALUES = WPD_SECTION_DATA_UNITS_VALUES(1i32);
impl ::core::convert::From<i32> for WPD_SECTION_DATA_UNITS_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_SECTION_DATA_UNITS_VALUES {
    type Abi = Self;
}
pub const WPD_SECTION_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1365966123, 50766, 17648, [152, 220, 190, 225, 200, 143, 125, 102]);
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_SERVICE_INHERITANCE_TYPES(pub i32);
pub const WPD_SERVICE_INHERITANCE_IMPLEMENTATION: WPD_SERVICE_INHERITANCE_TYPES = WPD_SERVICE_INHERITANCE_TYPES(0i32);
impl ::core::convert::From<i32> for WPD_SERVICE_INHERITANCE_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_SERVICE_INHERITANCE_TYPES {
    type Abi = Self;
}
pub const WPD_SERVICE_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1964009866, 52052, 18460, [184, 219, 13, 117, 201, 63, 28, 6]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SERVICE_VERSION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1964009866, 52052, 18460, [184, 219, 13, 117, 201, 63, 28, 6]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SMS_ENCODING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2115007692, 20735, 19921, [167, 66, 83, 190, 111, 9, 58, 13]),
    pid: 5u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_SMS_ENCODING_TYPES(pub i32);
pub const SMS_ENCODING_7_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(0i32);
pub const SMS_ENCODING_8_BIT: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(1i32);
pub const SMS_ENCODING_UTF_16: WPD_SMS_ENCODING_TYPES = WPD_SMS_ENCODING_TYPES(2i32);
impl ::core::convert::From<i32> for WPD_SMS_ENCODING_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_SMS_ENCODING_TYPES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SMS_MAX_PAYLOAD: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2115007692, 20735, 19921, [167, 66, 83, 190, 111, 9, 58, 13]),
    pid: 4u32,
};
pub const WPD_SMS_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2115007692, 20735, 19921, [167, 66, 83, 190, 111, 9, 58, 13]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SMS_PROVIDER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2115007692, 20735, 19921, [167, 66, 83, 190, 111, 9, 58, 13]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_SMS_TIMEOUT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(2115007692, 20735, 19921, [167, 66, 83, 190, 111, 9, 58, 13]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_ARTIST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_BURST_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_BURST_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAMERA_MANUFACTURER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAMERA_MODEL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAPTURE_DELAY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAPTURE_FORMAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAPTURE_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 18u32,
};
pub const WPD_STILL_IMAGE_CAPTURE_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CAPTURE_RESOLUTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_COMPRESSION_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_CONTRAST: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_DIGITAL_ZOOM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EFFECT_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EXPOSURE_BIAS_COMPENSATION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EXPOSURE_INDEX: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EXPOSURE_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EXPOSURE_PROGRAM_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_EXPOSURE_TIME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FLASH_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FNUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FOCAL_LENGTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FOCUS_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FOCUS_METERING_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_FOCUS_MODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_RGB_GAIN: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_SHARPNESS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_TIMELAPSE_INTERVAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_TIMELAPSE_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_UPLOAD_URL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STILL_IMAGE_WHITE_BALANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(1489334764, 7115, 17063, [138, 197, 187, 41, 21, 115, 162, 96]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_ACCESS_CAPABILITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 11u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_STORAGE_ACCESS_CAPABILITY_VALUES(pub i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READWRITE: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(0i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITHOUT_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(1i32);
pub const WPD_STORAGE_ACCESS_CAPABILITY_READ_ONLY_WITH_OBJECT_DELETION: WPD_STORAGE_ACCESS_CAPABILITY_VALUES = WPD_STORAGE_ACCESS_CAPABILITY_VALUES(2i32);
impl ::core::convert::From<i32> for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    type Abi = Self;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_CAPACITY: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_CAPACITY_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_FILE_SYSTEM_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_FREE_SPACE_IN_BYTES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_FREE_SPACE_IN_OBJECTS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_MAX_OBJECT_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 9u32,
};
pub const WPD_STORAGE_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_SERIAL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_STORAGE_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(27460986, 29910, 20096, [190, 167, 220, 76, 33, 44, 229, 10]),
    pid: 2u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_STORAGE_TYPE_VALUES(pub i32);
pub const WPD_STORAGE_TYPE_UNDEFINED: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(0i32);
pub const WPD_STORAGE_TYPE_FIXED_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(1i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_ROM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(2i32);
pub const WPD_STORAGE_TYPE_FIXED_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(3i32);
pub const WPD_STORAGE_TYPE_REMOVABLE_RAM: WPD_STORAGE_TYPE_VALUES = WPD_STORAGE_TYPE_VALUES(4i32);
impl ::core::convert::From<i32> for WPD_STORAGE_TYPE_VALUES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_STORAGE_TYPE_VALUES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_STREAM_UNITS(pub i32);
pub const WPD_STREAM_UNITS_BYTES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(0i32);
pub const WPD_STREAM_UNITS_FRAMES: WPD_STREAM_UNITS = WPD_STREAM_UNITS(1i32);
pub const WPD_STREAM_UNITS_ROWS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(2i32);
pub const WPD_STREAM_UNITS_MILLISECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(4i32);
pub const WPD_STREAM_UNITS_MICROSECONDS: WPD_STREAM_UNITS = WPD_STREAM_UNITS(8i32);
impl ::core::convert::From<i32> for WPD_STREAM_UNITS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_STREAM_UNITS {
    type Abi = Self;
}
pub const WPD_TASK_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3813992798, 55456, 17975, [160, 58, 12, 178, 104, 56, 219, 199]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_TASK_OWNER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3813992798, 55456, 17975, [160, 58, 12, 178, 104, 56, 219, 199]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_TASK_PERCENT_COMPLETE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3813992798, 55456, 17975, [160, 58, 12, 178, 104, 56, 219, 199]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_TASK_REMINDER_DATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3813992798, 55456, 17975, [160, 58, 12, 178, 104, 56, 219, 199]),
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_TASK_STATUS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(3813992798, 55456, 17975, [160, 58, 12, 178, 104, 56, 219, 199]),
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_AUTHOR: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_BITRATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_BUFFER_SIZE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_CREDITS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_FOURCC_CODE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_FRAMERATE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_KEY_FRAME_DISTANCE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 10u32,
};
pub const WPD_VIDEO_OBJECT_PROPERTIES_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_QUALITY_SETTING: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_RECORDEDTV_CHANNEL_NUMBER: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_RECORDEDTV_REPEAT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_RECORDEDTV_STATION_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_Devices_PortableDevices`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const WPD_VIDEO_SCAN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows::runtime::GUID::from_values(879698275, 63896, 16710, [139, 1, 209, 155, 76, 0, 222, 154]),
    pid: 12u32,
};
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_VIDEO_SCAN_TYPES(pub i32);
pub const WPD_VIDEO_SCAN_TYPE_UNUSED: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(0i32);
pub const WPD_VIDEO_SCAN_TYPE_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(1i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(2i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_INTERLEAVED_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(3i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_UPPER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(4i32);
pub const WPD_VIDEO_SCAN_TYPE_FIELD_SINGLE_LOWER_FIRST: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(5i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(6i32);
pub const WPD_VIDEO_SCAN_TYPE_MIXED_INTERLACE_AND_PROGRESSIVE: WPD_VIDEO_SCAN_TYPES = WPD_VIDEO_SCAN_TYPES(7i32);
impl ::core::convert::From<i32> for WPD_VIDEO_SCAN_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_VIDEO_SCAN_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WPD_WHITE_BALANCE_SETTINGS(pub i32);
pub const WPD_WHITE_BALANCE_UNDEFINED: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(0i32);
pub const WPD_WHITE_BALANCE_MANUAL: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(1i32);
pub const WPD_WHITE_BALANCE_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(2i32);
pub const WPD_WHITE_BALANCE_ONE_PUSH_AUTOMATIC: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(3i32);
pub const WPD_WHITE_BALANCE_DAYLIGHT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(4i32);
pub const WPD_WHITE_BALANCE_FLORESCENT: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(5i32);
pub const WPD_WHITE_BALANCE_TUNGSTEN: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(6i32);
pub const WPD_WHITE_BALANCE_FLASH: WPD_WHITE_BALANCE_SETTINGS = WPD_WHITE_BALANCE_SETTINGS(7i32);
impl ::core::convert::From<i32> for WPD_WHITE_BALANCE_SETTINGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WPD_WHITE_BALANCE_SETTINGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WpdAttributeForm(pub i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_UNSPECIFIED: WpdAttributeForm = WpdAttributeForm(0i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_RANGE: WpdAttributeForm = WpdAttributeForm(1i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_ENUMERATION: WpdAttributeForm = WpdAttributeForm(2i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdAttributeForm = WpdAttributeForm(3i32);
pub const WPD_PROPERTY_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdAttributeForm = WpdAttributeForm(4i32);
impl ::core::convert::From<i32> for WpdAttributeForm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WpdAttributeForm {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Devices_PortableDevices`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WpdParameterAttributeForm(pub i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_UNSPECIFIED: WpdParameterAttributeForm = WpdParameterAttributeForm(0i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_RANGE: WpdParameterAttributeForm = WpdParameterAttributeForm(1i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_ENUMERATION: WpdParameterAttributeForm = WpdParameterAttributeForm(2i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_REGULAR_EXPRESSION: WpdParameterAttributeForm = WpdParameterAttributeForm(3i32);
pub const WPD_PARAMETER_ATTRIBUTE_FORM_OBJECT_IDENTIFIER: WpdParameterAttributeForm = WpdParameterAttributeForm(4i32);
impl ::core::convert::From<i32> for WpdParameterAttributeForm {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WpdParameterAttributeForm {
    type Abi = Self;
}
pub const WpdSerializer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(194094923, 44412, 19101, [181, 99, 41, 238, 249, 22, 113, 114]);
