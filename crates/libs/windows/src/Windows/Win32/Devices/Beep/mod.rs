pub const BEEP_FREQUENCY_MAXIMUM: u32 = 32767u32;
pub const BEEP_FREQUENCY_MINIMUM: u32 = 37u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BEEP_SET_PARAMETERS {
    pub Frequency: u32,
    pub Duration: u32,
}
impl Default for BEEP_SET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BEEP_SET_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
pub const DD_BEEP_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\Beep");
pub const DD_BEEP_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\Beep");
pub const IOCTL_BEEP_SET: u32 = 65536u32;
