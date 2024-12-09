#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapturedMetadataExposureCompensation {
    pub Flags: u64,
    pub Value: i32,
}
impl Default for CapturedMetadataExposureCompensation {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CapturedMetadataExposureCompensation {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapturedMetadataISOGains {
    pub AnalogGain: f32,
    pub DigitalGain: f32,
}
impl Default for CapturedMetadataISOGains {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CapturedMetadataISOGains {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapturedMetadataWhiteBalanceGains {
    pub R: f32,
    pub G: f32,
    pub B: f32,
}
impl Default for CapturedMetadataWhiteBalanceGains {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CapturedMetadataWhiteBalanceGains {
    type TypeKind = windows_core::CopyType;
}
pub const DEVPKEY_Device_DLNACAP: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 16 };
pub const DEVPKEY_Device_DLNADOC: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 15 };
pub const DEVPKEY_Device_MaxVolume: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 19 };
pub const DEVPKEY_Device_PacketWakeSupported: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 0 };
pub const DEVPKEY_Device_SendPacketWakeSupported: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 1 };
pub const DEVPKEY_Device_SinkProtocolInfo: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 14 };
pub const DEVPKEY_Device_SupportsAudio: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 8 };
pub const DEVPKEY_Device_SupportsImages: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 10 };
pub const DEVPKEY_Device_SupportsMute: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 18 };
pub const DEVPKEY_Device_SupportsSearch: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 17 };
pub const DEVPKEY_Device_SupportsSetNextAVT: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 20 };
pub const DEVPKEY_Device_SupportsVideo: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 9 };
pub const DEVPKEY_Device_UDN: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x88ad39db_0d0c_4a38_8435_4043826b5c91), pid: 6 };
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceCharacterization {
    pub BlinkScoreLeft: u32,
    pub BlinkScoreRight: u32,
    pub FacialExpression: u32,
    pub FacialExpressionScore: u32,
}
impl Default for FaceCharacterization {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FaceCharacterization {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceCharacterizationBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl Default for FaceCharacterizationBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FaceCharacterizationBlobHeader {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceRectInfo {
    pub Region: super::super::Foundation::RECT,
    pub confidenceLevel: i32,
}
impl Default for FaceRectInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FaceRectInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceRectInfoBlobHeader {
    pub Size: u32,
    pub Count: u32,
}
impl Default for FaceRectInfoBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FaceRectInfoBlobHeader {
    type TypeKind = windows_core::CopyType;
}
pub const GUID_DEVINTERFACE_DMP: windows_core::GUID = windows_core::GUID::from_u128(0x25b4e268_2a05_496e_803b_266837fbda4b);
pub const GUID_DEVINTERFACE_DMR: windows_core::GUID = windows_core::GUID::from_u128(0xd0875fb4_2196_4c7a_a63d_e416addd60a1);
pub const GUID_DEVINTERFACE_DMS: windows_core::GUID = windows_core::GUID::from_u128(0xc96037ae_a558_4470_b432_115a31b85553);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistogramBlobHeader {
    pub Size: u32,
    pub Histograms: u32,
}
impl Default for HistogramBlobHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HistogramBlobHeader {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistogramDataHeader {
    pub Size: u32,
    pub ChannelMask: u32,
    pub Linear: u32,
}
impl Default for HistogramDataHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HistogramDataHeader {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistogramGrid {
    pub Width: u32,
    pub Height: u32,
    pub Region: super::super::Foundation::RECT,
}
impl Default for HistogramGrid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HistogramGrid {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HistogramHeader {
    pub Size: u32,
    pub Bins: u32,
    pub FourCC: u32,
    pub ChannelMasks: u32,
    pub Grid: HistogramGrid,
}
impl Default for HistogramHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for HistogramHeader {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_MEDIASOURCE_STATUS_INFO(pub i32);
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(0i32);
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = MF_MEDIASOURCE_STATUS_INFO(1i32);
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_TRANSFER_VIDEO_FRAME_FLAGS(pub i32);
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(2i32);
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = MF_TRANSFER_VIDEO_FRAME_FLAGS(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetadataTimeStamps {
    pub Flags: u32,
    pub Device: i64,
    pub Presentation: i64,
}
impl Default for MetadataTimeStamps {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MetadataTimeStamps {
    type TypeKind = windows_core::CopyType;
}
