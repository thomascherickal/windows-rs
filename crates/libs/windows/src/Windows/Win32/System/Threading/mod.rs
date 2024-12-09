#[inline]
pub unsafe fn AcquireSRWLockExclusive(srwlock: *mut SRWLOCK) {
    windows_targets::link!("kernel32.dll" "system" fn AcquireSRWLockExclusive(srwlock : *mut SRWLOCK));
    AcquireSRWLockExclusive(core::mem::transmute(srwlock))
}
#[inline]
pub unsafe fn AcquireSRWLockShared(srwlock: *mut SRWLOCK) {
    windows_targets::link!("kernel32.dll" "system" fn AcquireSRWLockShared(srwlock : *mut SRWLOCK));
    AcquireSRWLockShared(core::mem::transmute(srwlock))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn AddIntegrityLabelToBoundaryDescriptor<P1>(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Security::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor : *mut super::super::Foundation:: HANDLE, integritylabel : super::super::Security:: PSID) -> super::super::Foundation:: BOOL);
    AddIntegrityLabelToBoundaryDescriptor(core::mem::transmute(boundarydescriptor), integritylabel.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn AddSIDToBoundaryDescriptor<P1>(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Security::PSID>,
{
    windows_targets::link!("kernel32.dll" "system" fn AddSIDToBoundaryDescriptor(boundarydescriptor : *mut super::super::Foundation:: HANDLE, requiredsid : super::super::Security:: PSID) -> super::super::Foundation:: BOOL);
    AddSIDToBoundaryDescriptor(core::mem::transmute(boundarydescriptor), requiredsid.param().abi()).ok()
}
#[inline]
pub unsafe fn AttachThreadInput<P2>(idattach: u32, idattachto: u32, fattach: P2) -> super::super::Foundation::BOOL
where
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn AttachThreadInput(idattach : u32, idattachto : u32, fattach : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    AttachThreadInput(core::mem::transmute(idattach), core::mem::transmute(idattachto), fattach.param().abi())
}
#[inline]
pub unsafe fn AvQuerySystemResponsiveness<P0>(avrthandle: P0, systemresponsivenessvalue: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvQuerySystemResponsiveness(avrthandle : super::super::Foundation:: HANDLE, systemresponsivenessvalue : *mut u32) -> super::super::Foundation:: BOOL);
    AvQuerySystemResponsiveness(avrthandle.param().abi(), core::mem::transmute(systemresponsivenessvalue)).ok()
}
#[inline]
pub unsafe fn AvRevertMmThreadCharacteristics<P0>(avrthandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRevertMmThreadCharacteristics(avrthandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AvRevertMmThreadCharacteristics(avrthandle.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroup(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut windows_core::GUID, timeout: Option<*const i64>) -> windows_core::Result<()> {
    windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroup(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_core::GUID, timeout : *const i64) -> super::super::Foundation:: BOOL);
    AvRtCreateThreadOrderingGroup(core::mem::transmute(context), core::mem::transmute(period), core::mem::transmute(threadorderingguid), core::mem::transmute(timeout.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExA<P4>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut windows_core::GUID, timeout: Option<*const i64>, taskname: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExA(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_core::GUID, timeout : *const i64, taskname : windows_core::PCSTR) -> super::super::Foundation:: BOOL);
    AvRtCreateThreadOrderingGroupExA(core::mem::transmute(context), core::mem::transmute(period), core::mem::transmute(threadorderingguid), core::mem::transmute(timeout.unwrap_or(core::ptr::null())), taskname.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExW<P4>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut windows_core::GUID, timeout: Option<*const i64>, taskname: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtCreateThreadOrderingGroupExW(context : *mut super::super::Foundation:: HANDLE, period : *const i64, threadorderingguid : *mut windows_core::GUID, timeout : *const i64, taskname : windows_core::PCWSTR) -> super::super::Foundation:: BOOL);
    AvRtCreateThreadOrderingGroupExW(core::mem::transmute(context), core::mem::transmute(period), core::mem::transmute(threadorderingguid), core::mem::transmute(timeout.unwrap_or(core::ptr::null())), taskname.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtDeleteThreadOrderingGroup<P0>(context: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtDeleteThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AvRtDeleteThreadOrderingGroup(context.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtJoinThreadOrderingGroup<P2>(context: *mut super::super::Foundation::HANDLE, threadorderingguid: *const windows_core::GUID, before: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtJoinThreadOrderingGroup(context : *mut super::super::Foundation:: HANDLE, threadorderingguid : *const windows_core::GUID, before : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    AvRtJoinThreadOrderingGroup(core::mem::transmute(context), core::mem::transmute(threadorderingguid), before.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtLeaveThreadOrderingGroup<P0>(context: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtLeaveThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AvRtLeaveThreadOrderingGroup(context.param().abi()).ok()
}
#[inline]
pub unsafe fn AvRtWaitOnThreadOrderingGroup<P0>(context: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvRtWaitOnThreadOrderingGroup(context : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AvRtWaitOnThreadOrderingGroup(context.param().abi()).ok()
}
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsA<P0, P1>(firsttask: P0, secondtask: P1, taskindex: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsA(firsttask : windows_core::PCSTR, secondtask : windows_core::PCSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = AvSetMmMaxThreadCharacteristicsA(firsttask.param().abi(), secondtask.param().abi(), core::mem::transmute(taskindex));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsW<P0, P1>(firsttask: P0, secondtask: P1, taskindex: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvSetMmMaxThreadCharacteristicsW(firsttask : windows_core::PCWSTR, secondtask : windows_core::PCWSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = AvSetMmMaxThreadCharacteristicsW(firsttask.param().abi(), secondtask.param().abi(), core::mem::transmute(taskindex));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsA<P0>(taskname: P0, taskindex: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsA(taskname : windows_core::PCSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = AvSetMmThreadCharacteristicsA(taskname.param().abi(), core::mem::transmute(taskindex));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsW<P0>(taskname: P0, taskindex: *mut u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadCharacteristicsW(taskname : windows_core::PCWSTR, taskindex : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = AvSetMmThreadCharacteristicsW(taskname.param().abi(), core::mem::transmute(taskindex));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn AvSetMmThreadPriority<P0>(avrthandle: P0, priority: AVRT_PRIORITY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("avrt.dll" "system" fn AvSetMmThreadPriority(avrthandle : super::super::Foundation:: HANDLE, priority : AVRT_PRIORITY) -> super::super::Foundation:: BOOL);
    AvSetMmThreadPriority(avrthandle.param().abi(), core::mem::transmute(priority)).ok()
}
#[inline]
pub unsafe fn CallbackMayRunLong<P0>(pci: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CallbackMayRunLong(pci : PTP_CALLBACK_INSTANCE) -> super::super::Foundation:: BOOL);
    CallbackMayRunLong(pci.param().abi())
}
#[inline]
pub unsafe fn CancelThreadpoolIo<P0>(pio: P0)
where
    P0: windows_core::Param<PTP_IO>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelThreadpoolIo(pio : PTP_IO));
    CancelThreadpoolIo(pio.param().abi())
}
#[inline]
pub unsafe fn CancelTimerQueueTimer<P0, P1>(timerqueue: P0, timer: P1) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelTimerQueueTimer(timerqueue.param().abi(), timer.param().abi())
}
#[inline]
pub unsafe fn CancelWaitableTimer<P0>(htimer: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CancelWaitableTimer(htimer : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelWaitableTimer(htimer.param().abi()).ok()
}
#[inline]
pub unsafe fn ChangeTimerQueueTimer<P0>(timerqueue: P0, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ChangeTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE, duetime : u32, period : u32) -> super::super::Foundation:: BOOL);
    ChangeTimerQueueTimer(timerqueue.param().abi(), core::mem::transmute(timer), core::mem::transmute(duetime), core::mem::transmute(period)).ok()
}
#[inline]
pub unsafe fn ClosePrivateNamespace<P0>(handle: P0, flags: u32) -> super::super::Foundation::BOOLEAN
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ClosePrivateNamespace(handle : super::super::Foundation:: HANDLE, flags : u32) -> super::super::Foundation:: BOOLEAN);
    ClosePrivateNamespace(handle.param().abi(), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn CloseThreadpool<P0>(ptpp: P0)
where
    P0: windows_core::Param<PTP_POOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : PTP_POOL));
    CloseThreadpool(ptpp.param().abi())
}
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroup<P0>(ptpcg: P0)
where
    P0: windows_core::Param<PTP_CLEANUP_GROUP>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : PTP_CLEANUP_GROUP));
    CloseThreadpoolCleanupGroup(ptpcg.param().abi())
}
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroupMembers<P0, P1>(ptpcg: P0, fcancelpendingcallbacks: P1, pvcleanupcontext: Option<*mut core::ffi::c_void>)
where
    P0: windows_core::Param<PTP_CLEANUP_GROUP>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroupMembers(ptpcg : PTP_CLEANUP_GROUP, fcancelpendingcallbacks : super::super::Foundation:: BOOL, pvcleanupcontext : *mut core::ffi::c_void));
    CloseThreadpoolCleanupGroupMembers(ptpcg.param().abi(), fcancelpendingcallbacks.param().abi(), core::mem::transmute(pvcleanupcontext.unwrap_or(core::ptr::null_mut())))
}
#[inline]
pub unsafe fn CloseThreadpoolIo<P0>(pio: P0)
where
    P0: windows_core::Param<PTP_IO>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : PTP_IO));
    CloseThreadpoolIo(pio.param().abi())
}
#[inline]
pub unsafe fn CloseThreadpoolTimer<P0>(pti: P0)
where
    P0: windows_core::Param<PTP_TIMER>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : PTP_TIMER));
    CloseThreadpoolTimer(pti.param().abi())
}
#[inline]
pub unsafe fn CloseThreadpoolWait<P0>(pwa: P0)
where
    P0: windows_core::Param<PTP_WAIT>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : PTP_WAIT));
    CloseThreadpoolWait(pwa.param().abi())
}
#[inline]
pub unsafe fn CloseThreadpoolWork<P0>(pwk: P0)
where
    P0: windows_core::Param<PTP_WORK>,
{
    windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : PTP_WORK));
    CloseThreadpoolWork(pwk.param().abi())
}
#[inline]
pub unsafe fn ConvertFiberToThread() -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn ConvertFiberToThread() -> super::super::Foundation:: BOOL);
    ConvertFiberToThread().ok()
}
#[inline]
pub unsafe fn ConvertThreadToFiber(lpparameter: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn ConvertThreadToFiber(lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    ConvertThreadToFiber(core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())))
}
#[inline]
pub unsafe fn ConvertThreadToFiberEx(lpparameter: Option<*const core::ffi::c_void>, dwflags: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn ConvertThreadToFiberEx(lpparameter : *const core::ffi::c_void, dwflags : u32) -> *mut core::ffi::c_void);
    ConvertThreadToFiberEx(core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn CreateBoundaryDescriptorA<P0>(name: P0, flags: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorA(name : windows_core::PCSTR, flags : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateBoundaryDescriptorA(name.param().abi(), core::mem::transmute(flags));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateBoundaryDescriptorW<P0>(name: P0, flags: u32) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorW(name : windows_core::PCWSTR, flags : u32) -> super::super::Foundation:: HANDLE);
    CreateBoundaryDescriptorW(name.param().abi(), core::mem::transmute(flags))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateEventA<P1, P2, P3>(lpeventattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P1, binitialstate: P2, lpname: P3) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateEventA(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, binitialstate : super::super::Foundation:: BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateEventA(core::mem::transmute(lpeventattributes.unwrap_or(core::ptr::null())), bmanualreset.param().abi(), binitialstate.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateEventExA<P1>(lpeventattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateEventExA(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : CREATE_EVENT, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateEventExA(core::mem::transmute(lpeventattributes.unwrap_or(core::ptr::null())), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateEventExW<P1>(lpeventattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateEventExW(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : CREATE_EVENT, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateEventExW(core::mem::transmute(lpeventattributes.unwrap_or(core::ptr::null())), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateEventW<P1, P2, P3>(lpeventattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P1, binitialstate: P2, lpname: P3) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, binitialstate : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateEventW(core::mem::transmute(lpeventattributes.unwrap_or(core::ptr::null())), bmanualreset.param().abi(), binitialstate.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn CreateFiber(dwstacksize : usize, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    CreateFiber(core::mem::transmute(dwstacksize), core::mem::transmute(lpstartaddress), core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())))
}
#[inline]
pub unsafe fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn CreateFiberEx(dwstackcommitsize : usize, dwstackreservesize : usize, dwflags : u32, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    CreateFiberEx(core::mem::transmute(dwstackcommitsize), core::mem::transmute(dwstackreservesize), core::mem::transmute(dwflags), core::mem::transmute(lpstartaddress), core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMutexA<P1, P2>(lpmutexattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binitialowner: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binitialowner : super::super::Foundation:: BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMutexA(core::mem::transmute(lpmutexattributes.unwrap_or(core::ptr::null())), binitialowner.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMutexExA<P1>(lpmutexattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMutexExA(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMutexExA(core::mem::transmute(lpmutexattributes.unwrap_or(core::ptr::null())), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMutexExW<P1>(lpmutexattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P1, dwflags: u32, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMutexExW(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMutexExW(core::mem::transmute(lpmutexattributes.unwrap_or(core::ptr::null())), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateMutexW<P1, P2>(lpmutexattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binitialowner: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binitialowner : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMutexW(core::mem::transmute(lpmutexattributes.unwrap_or(core::ptr::null())), binitialowner.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreatePrivateNamespaceA<P2>(lpprivatenamespaceattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreatePrivateNamespaceA(lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreatePrivateNamespaceA(core::mem::transmute(lpprivatenamespaceattributes.unwrap_or(core::ptr::null())), core::mem::transmute(lpboundarydescriptor), lpaliasprefix.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreatePrivateNamespaceW<P2>(lpprivatenamespaceattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P2) -> super::super::Foundation::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreatePrivateNamespaceW(lpprivatenamespaceattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    CreatePrivateNamespaceW(core::mem::transmute(lpprivatenamespaceattributes.unwrap_or(core::ptr::null())), core::mem::transmute(lpboundarydescriptor), lpaliasprefix.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateProcessA<P0, P4, P7>(lpapplicationname: P0, lpcommandline: windows_core::PSTR, lpprocessattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P4, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P7, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<super::super::Foundation::BOOL>,
    P7: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateProcessA(lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessA(lpapplicationname.param().abi(), core::mem::transmute(lpcommandline), core::mem::transmute(lpprocessattributes.unwrap_or(core::ptr::null())), core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())), binherithandles.param().abi(), core::mem::transmute(dwcreationflags), core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())), lpcurrentdirectory.param().abi(), core::mem::transmute(lpstartupinfo), core::mem::transmute(lpprocessinformation)).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateProcessAsUserA<P0, P1, P5, P8>(htoken: P0, lpapplicationname: P1, lpcommandline: windows_core::PSTR, lpprocessattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P5, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P8, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<super::super::Foundation::BOOL>,
    P8: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateProcessAsUserA(htoken : super::super::Foundation:: HANDLE, lpapplicationname : windows_core::PCSTR, lpcommandline : windows_core::PSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCSTR, lpstartupinfo : *const STARTUPINFOA, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessAsUserA(
        htoken.param().abi(),
        lpapplicationname.param().abi(),
        core::mem::transmute(lpcommandline),
        core::mem::transmute(lpprocessattributes.unwrap_or(core::ptr::null())),
        core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())),
        binherithandles.param().abi(),
        core::mem::transmute(dwcreationflags),
        core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())),
        lpcurrentdirectory.param().abi(),
        core::mem::transmute(lpstartupinfo),
        core::mem::transmute(lpprocessinformation),
    )
    .ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateProcessAsUserW<P0, P1, P5, P8>(htoken: P0, lpapplicationname: P1, lpcommandline: windows_core::PWSTR, lpprocessattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P5, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P8, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<super::super::Foundation::BOOL>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateProcessAsUserW(htoken : super::super::Foundation:: HANDLE, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessAsUserW(
        htoken.param().abi(),
        lpapplicationname.param().abi(),
        core::mem::transmute(lpcommandline),
        core::mem::transmute(lpprocessattributes.unwrap_or(core::ptr::null())),
        core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())),
        binherithandles.param().abi(),
        core::mem::transmute(dwcreationflags),
        core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())),
        lpcurrentdirectory.param().abi(),
        core::mem::transmute(lpstartupinfo),
        core::mem::transmute(lpprocessinformation),
    )
    .ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateProcessW<P0, P4, P7>(lpapplicationname: P0, lpcommandline: windows_core::PWSTR, lpprocessattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, binherithandles: P4, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P7, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<super::super::Foundation::BOOL>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateProcessW(lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, lpprocessattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, binherithandles : super::super::Foundation:: BOOL, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessW(lpapplicationname.param().abi(), core::mem::transmute(lpcommandline), core::mem::transmute(lpprocessattributes.unwrap_or(core::ptr::null())), core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())), binherithandles.param().abi(), core::mem::transmute(dwcreationflags), core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())), lpcurrentdirectory.param().abi(), core::mem::transmute(lpstartupinfo), core::mem::transmute(lpprocessinformation)).ok()
}
#[inline]
pub unsafe fn CreateProcessWithLogonW<P0, P1, P2, P4, P8>(lpusername: P0, lpdomain: P1, lppassword: P2, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: P4, lpcommandline: windows_core::PWSTR, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P8, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateProcessWithLogonW(lpusername : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, dwlogonflags : CREATE_PROCESS_LOGON_FLAGS, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessWithLogonW(lpusername.param().abi(), lpdomain.param().abi(), lppassword.param().abi(), core::mem::transmute(dwlogonflags), lpapplicationname.param().abi(), core::mem::transmute(lpcommandline), core::mem::transmute(dwcreationflags), core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())), lpcurrentdirectory.param().abi(), core::mem::transmute(lpstartupinfo), core::mem::transmute(lpprocessinformation)).ok()
}
#[inline]
pub unsafe fn CreateProcessWithTokenW<P0, P2, P6>(htoken: P0, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: P2, lpcommandline: windows_core::PWSTR, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: Option<*const core::ffi::c_void>, lpcurrentdirectory: P6, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("advapi32.dll" "system" fn CreateProcessWithTokenW(htoken : super::super::Foundation:: HANDLE, dwlogonflags : CREATE_PROCESS_LOGON_FLAGS, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, dwcreationflags : PROCESS_CREATION_FLAGS, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : *const STARTUPINFOW, lpprocessinformation : *mut PROCESS_INFORMATION) -> super::super::Foundation:: BOOL);
    CreateProcessWithTokenW(htoken.param().abi(), core::mem::transmute(dwlogonflags), lpapplicationname.param().abi(), core::mem::transmute(lpcommandline), core::mem::transmute(dwcreationflags), core::mem::transmute(lpenvironment.unwrap_or(core::ptr::null())), lpcurrentdirectory.param().abi(), core::mem::transmute(lpstartupinfo), core::mem::transmute(lpprocessinformation)).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateRemoteThread<P0>(hprocess: P0, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: u32, lpthreadid: Option<*mut u32>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateRemoteThread(hprocess : super::super::Foundation:: HANDLE, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateRemoteThread(hprocess.param().abi(), core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())), core::mem::transmute(dwstacksize), core::mem::transmute(lpstartaddress), core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())), core::mem::transmute(dwcreationflags), core::mem::transmute(lpthreadid.unwrap_or(core::ptr::null_mut())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateRemoteThreadEx<P0, P6>(hprocess: P0, lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: u32, lpattributelist: P6, lpthreadid: Option<*mut u32>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P6: windows_core::Param<LPPROC_THREAD_ATTRIBUTE_LIST>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateRemoteThreadEx(hprocess : super::super::Foundation:: HANDLE, lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : u32, lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateRemoteThreadEx(hprocess.param().abi(), core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())), core::mem::transmute(dwstacksize), core::mem::transmute(lpstartaddress), core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())), core::mem::transmute(dwcreationflags), lpattributelist.param().abi(), core::mem::transmute(lpthreadid.unwrap_or(core::ptr::null_mut())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateSemaphoreA<P3>(lpsemaphoreattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreA(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateSemaphoreA(core::mem::transmute(lpsemaphoreattributes.unwrap_or(core::ptr::null())), core::mem::transmute(linitialcount), core::mem::transmute(lmaximumcount), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateSemaphoreExA<P3>(lpsemaphoreattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3, dwflags: u32, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreExA(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateSemaphoreExA(core::mem::transmute(lpsemaphoreattributes.unwrap_or(core::ptr::null())), core::mem::transmute(linitialcount), core::mem::transmute(lmaximumcount), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateSemaphoreExW<P3>(lpsemaphoreattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3, dwflags: u32, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreExW(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateSemaphoreExW(core::mem::transmute(lpsemaphoreattributes.unwrap_or(core::ptr::null())), core::mem::transmute(linitialcount), core::mem::transmute(lmaximumcount), lpname.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateSemaphoreW<P3>(lpsemaphoreattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, linitialcount: i32, lmaximumcount: i32, lpname: P3) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateSemaphoreW(lpsemaphoreattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateSemaphoreW(core::mem::transmute(lpsemaphoreattributes.unwrap_or(core::ptr::null())), core::mem::transmute(linitialcount), core::mem::transmute(lmaximumcount), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateThread(lpthreadattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: Option<*const core::ffi::c_void>, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: Option<*mut u32>) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThread(lpthreadattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, dwstacksize : usize, lpstartaddress : LPTHREAD_START_ROUTINE, lpparameter : *const core::ffi::c_void, dwcreationflags : THREAD_CREATION_FLAGS, lpthreadid : *mut u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateThread(core::mem::transmute(lpthreadattributes.unwrap_or(core::ptr::null())), core::mem::transmute(dwstacksize), core::mem::transmute(lpstartaddress), core::mem::transmute(lpparameter.unwrap_or(core::ptr::null())), core::mem::transmute(dwcreationflags), core::mem::transmute(lpthreadid.unwrap_or(core::ptr::null_mut())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpool(reserved: Option<*const core::ffi::c_void>) -> windows_core::Result<PTP_POOL> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpool(reserved : *const core::ffi::c_void) -> PTP_POOL);
    let result__ = CreateThreadpool(core::mem::transmute(reserved.unwrap_or(core::ptr::null())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpoolCleanupGroup() -> windows_core::Result<PTP_CLEANUP_GROUP> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolCleanupGroup() -> PTP_CLEANUP_GROUP);
    let result__ = CreateThreadpoolCleanupGroup();
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpoolIo<P0>(fl: P0, pfnio: PTP_WIN32_IO_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const TP_CALLBACK_ENVIRON_V3>) -> windows_core::Result<PTP_IO>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolIo(fl : super::super::Foundation:: HANDLE, pfnio : PTP_WIN32_IO_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_IO);
    let result__ = CreateThreadpoolIo(fl.param().abi(), core::mem::transmute(pfnio), core::mem::transmute(pv.unwrap_or(core::ptr::null_mut())), core::mem::transmute(pcbe.unwrap_or(core::ptr::null())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const TP_CALLBACK_ENVIRON_V3>) -> windows_core::Result<PTP_TIMER> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolTimer(pfnti : PTP_TIMER_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_TIMER);
    let result__ = CreateThreadpoolTimer(core::mem::transmute(pfnti), core::mem::transmute(pv.unwrap_or(core::ptr::null_mut())), core::mem::transmute(pcbe.unwrap_or(core::ptr::null())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const TP_CALLBACK_ENVIRON_V3>) -> windows_core::Result<PTP_WAIT> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolWait(pfnwa : PTP_WAIT_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_WAIT);
    let result__ = CreateThreadpoolWait(core::mem::transmute(pfnwa), core::mem::transmute(pv.unwrap_or(core::ptr::null_mut())), core::mem::transmute(pcbe.unwrap_or(core::ptr::null())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const TP_CALLBACK_ENVIRON_V3>) -> windows_core::Result<PTP_WORK> {
    windows_targets::link!("kernel32.dll" "system" fn CreateThreadpoolWork(pfnwk : PTP_WORK_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> PTP_WORK);
    let result__ = CreateThreadpoolWork(core::mem::transmute(pfnwk), core::mem::transmute(pv.unwrap_or(core::ptr::null_mut())), core::mem::transmute(pcbe.unwrap_or(core::ptr::null())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateTimerQueue() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateTimerQueue() -> super::super::Foundation:: HANDLE);
    let result__ = CreateTimerQueue();
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateTimerQueueTimer<P1>(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: P1, callback: WAITORTIMERCALLBACK, parameter: Option<*const core::ffi::c_void>, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateTimerQueueTimer(phnewtimer : *mut super::super::Foundation:: HANDLE, timerqueue : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, flags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
    CreateTimerQueueTimer(core::mem::transmute(phnewtimer), timerqueue.param().abi(), core::mem::transmute(callback), core::mem::transmute(parameter.unwrap_or(core::ptr::null())), core::mem::transmute(duetime), core::mem::transmute(period), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn CreateUmsCompletionList(umscompletionlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CreateUmsCompletionList(umscompletionlist : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreateUmsCompletionList(core::mem::transmute(umscompletionlist)).ok()
}
#[inline]
pub unsafe fn CreateUmsThreadContext(lpumsthread: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CreateUmsThreadContext(lpumsthread : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    CreateUmsThreadContext(core::mem::transmute(lpumsthread)).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWaitableTimerA<P1, P2>(lptimerattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P1, lptimername: P2) -> super::super::Foundation::HANDLE
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerA(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, lptimername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    CreateWaitableTimerA(core::mem::transmute(lptimerattributes.unwrap_or(core::ptr::null())), bmanualreset.param().abi(), lptimername.param().abi())
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWaitableTimerExA<P1>(lptimerattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lptimername: P1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerExA(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lptimername : windows_core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    CreateWaitableTimerExA(core::mem::transmute(lptimerattributes.unwrap_or(core::ptr::null())), lptimername.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWaitableTimerExW<P1>(lptimerattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lptimername: P1, dwflags: u32, dwdesiredaccess: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lptimername : windows_core::PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateWaitableTimerExW(core::mem::transmute(lptimerattributes.unwrap_or(core::ptr::null())), lptimername.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(dwdesiredaccess));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateWaitableTimerW<P1, P2>(lptimerattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, bmanualreset: P1, lptimername: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, bmanualreset : super::super::Foundation:: BOOL, lptimername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateWaitableTimerW(core::mem::transmute(lptimerattributes.unwrap_or(core::ptr::null())), bmanualreset.param().abi(), lptimername.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DeleteBoundaryDescriptor<P0>(boundarydescriptor: P0)
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteBoundaryDescriptor(boundarydescriptor : super::super::Foundation:: HANDLE));
    DeleteBoundaryDescriptor(boundarydescriptor.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn DeleteCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION) {
    windows_targets::link!("kernel32.dll" "system" fn DeleteCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
    DeleteCriticalSection(core::mem::transmute(lpcriticalsection))
}
#[inline]
pub unsafe fn DeleteFiber(lpfiber: *const core::ffi::c_void) {
    windows_targets::link!("kernel32.dll" "system" fn DeleteFiber(lpfiber : *const core::ffi::c_void));
    DeleteFiber(core::mem::transmute(lpfiber))
}
#[inline]
pub unsafe fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST) {
    windows_targets::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST));
    DeleteProcThreadAttributeList(core::mem::transmute(lpattributelist))
}
#[inline]
pub unsafe fn DeleteSynchronizationBarrier(lpbarrier: *mut SYNCHRONIZATION_BARRIER) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn DeleteSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER) -> super::super::Foundation:: BOOL);
    DeleteSynchronizationBarrier(core::mem::transmute(lpbarrier))
}
#[inline]
pub unsafe fn DeleteTimerQueue<P0>(timerqueue: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueue(timerqueue : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteTimerQueue(timerqueue.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteTimerQueueEx<P0, P1>(timerqueue: P0, completionevent: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueueEx(timerqueue : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteTimerQueueEx(timerqueue.param().abi(), completionevent.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteTimerQueueTimer<P0, P1, P2>(timerqueue: P0, timer: P1, completionevent: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DeleteTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, timer : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DeleteTimerQueueTimer(timerqueue.param().abi(), timer.param().abi(), completionevent.param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteUmsCompletionList(umscompletionlist: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn DeleteUmsCompletionList(umscompletionlist : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeleteUmsCompletionList(core::mem::transmute(umscompletionlist)).ok()
}
#[inline]
pub unsafe fn DeleteUmsThreadContext(umsthread: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn DeleteUmsThreadContext(umsthread : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DeleteUmsThreadContext(core::mem::transmute(umsthread)).ok()
}
#[inline]
pub unsafe fn DequeueUmsCompletionListItems(umscompletionlist: *const core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn DequeueUmsCompletionListItems(umscompletionlist : *const core::ffi::c_void, waittimeout : u32, umsthreadlist : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    DequeueUmsCompletionListItems(core::mem::transmute(umscompletionlist), core::mem::transmute(waittimeout), core::mem::transmute(umsthreadlist)).ok()
}
#[inline]
pub unsafe fn DisassociateCurrentThreadFromCallback<P0>(pci: P0)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
{
    windows_targets::link!("kernel32.dll" "system" fn DisassociateCurrentThreadFromCallback(pci : PTP_CALLBACK_INSTANCE));
    DisassociateCurrentThreadFromCallback(pci.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn EnterCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION) {
    windows_targets::link!("kernel32.dll" "system" fn EnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
    EnterCriticalSection(core::mem::transmute(lpcriticalsection))
}
#[inline]
pub unsafe fn EnterSynchronizationBarrier(lpbarrier: *mut SYNCHRONIZATION_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn EnterSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, dwflags : u32) -> super::super::Foundation:: BOOL);
    EnterSynchronizationBarrier(core::mem::transmute(lpbarrier), core::mem::transmute(dwflags))
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn EnterUmsSchedulingMode(schedulerstartupinfo : *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation:: BOOL);
    EnterUmsSchedulingMode(core::mem::transmute(schedulerstartupinfo)).ok()
}
#[inline]
pub unsafe fn ExecuteUmsThread(umsthread: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn ExecuteUmsThread(umsthread : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    ExecuteUmsThread(core::mem::transmute(umsthread)).ok()
}
#[inline]
pub unsafe fn ExitProcess(uexitcode: u32) -> ! {
    windows_targets::link!("kernel32.dll" "system" fn ExitProcess(uexitcode : u32) -> !);
    ExitProcess(core::mem::transmute(uexitcode))
}
#[inline]
pub unsafe fn ExitThread(dwexitcode: u32) -> ! {
    windows_targets::link!("kernel32.dll" "system" fn ExitThread(dwexitcode : u32) -> !);
    ExitThread(core::mem::transmute(dwexitcode))
}
#[inline]
pub unsafe fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : PFLS_CALLBACK_FUNCTION) -> u32);
    FlsAlloc(core::mem::transmute(lpcallback))
}
#[inline]
pub unsafe fn FlsFree(dwflsindex: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : u32) -> super::super::Foundation:: BOOL);
    FlsFree(core::mem::transmute(dwflsindex)).ok()
}
#[inline]
pub unsafe fn FlsGetValue(dwflsindex: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : u32) -> *mut core::ffi::c_void);
    FlsGetValue(core::mem::transmute(dwflsindex))
}
#[inline]
pub unsafe fn FlsSetValue(dwflsindex: u32, lpflsdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : u32, lpflsdata : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    FlsSetValue(core::mem::transmute(dwflsindex), core::mem::transmute(lpflsdata.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn FlushProcessWriteBuffers() {
    windows_targets::link!("kernel32.dll" "system" fn FlushProcessWriteBuffers());
    FlushProcessWriteBuffers()
}
#[inline]
pub unsafe fn FreeLibraryWhenCallbackReturns<P0, P1>(pci: P0, r#mod: P1)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HMODULE>,
{
    windows_targets::link!("kernel32.dll" "system" fn FreeLibraryWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, r#mod : super::super::Foundation:: HMODULE));
    FreeLibraryWhenCallbackReturns(pci.param().abi(), r#mod.param().abi())
}
#[inline]
pub unsafe fn GetActiveProcessorCount(groupnumber: u16) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetActiveProcessorCount(groupnumber : u16) -> u32);
    GetActiveProcessorCount(core::mem::transmute(groupnumber))
}
#[inline]
pub unsafe fn GetActiveProcessorGroupCount() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetActiveProcessorGroupCount() -> u16);
    GetActiveProcessorGroupCount()
}
#[inline]
pub unsafe fn GetCurrentProcess() -> super::super::Foundation::HANDLE {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcess() -> super::super::Foundation:: HANDLE);
    GetCurrentProcess()
}
#[inline]
pub unsafe fn GetCurrentProcessId() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
    GetCurrentProcessId()
}
#[inline]
pub unsafe fn GetCurrentProcessorNumber() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessorNumber() -> u32);
    GetCurrentProcessorNumber()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn GetCurrentProcessorNumberEx() -> super::Kernel::PROCESSOR_NUMBER {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessorNumberEx(procnumber : *mut super::Kernel:: PROCESSOR_NUMBER));
    let mut result__ = core::mem::zeroed();
    GetCurrentProcessorNumberEx(&mut result__);
    result__
}
#[inline]
pub unsafe fn GetCurrentThread() -> super::super::Foundation::HANDLE {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentThread() -> super::super::Foundation:: HANDLE);
    GetCurrentThread()
}
#[inline]
pub unsafe fn GetCurrentThreadId() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentThreadId() -> u32);
    GetCurrentThreadId()
}
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize) {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentThreadStackLimits(lowlimit : *mut usize, highlimit : *mut usize));
    GetCurrentThreadStackLimits(core::mem::transmute(lowlimit), core::mem::transmute(highlimit))
}
#[inline]
pub unsafe fn GetCurrentUmsThread() -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn GetCurrentUmsThread() -> *mut core::ffi::c_void);
    GetCurrentUmsThread()
}
#[inline]
pub unsafe fn GetExitCodeProcess<P0>(hprocess: P0, lpexitcode: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : super::super::Foundation:: HANDLE, lpexitcode : *mut u32) -> super::super::Foundation:: BOOL);
    GetExitCodeProcess(hprocess.param().abi(), core::mem::transmute(lpexitcode)).ok()
}
#[inline]
pub unsafe fn GetExitCodeThread<P0>(hthread: P0, lpexitcode: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetExitCodeThread(hthread : super::super::Foundation:: HANDLE, lpexitcode : *mut u32) -> super::super::Foundation:: BOOL);
    GetExitCodeThread(hthread.param().abi(), core::mem::transmute(lpexitcode)).ok()
}
#[inline]
pub unsafe fn GetGuiResources<P0>(hprocess: P0, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn GetGuiResources(hprocess : super::super::Foundation:: HANDLE, uiflags : GET_GUI_RESOURCES_FLAGS) -> u32);
    GetGuiResources(hprocess.param().abi(), core::mem::transmute(uiflags))
}
#[inline]
pub unsafe fn GetMachineTypeAttributes(machine: u16) -> windows_core::Result<MACHINE_ATTRIBUTES> {
    windows_targets::link!("kernel32.dll" "system" fn GetMachineTypeAttributes(machine : u16, machinetypeattributes : *mut MACHINE_ATTRIBUTES) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetMachineTypeAttributes(core::mem::transmute(machine), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn GetMaximumProcessorCount(groupnumber: u16) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetMaximumProcessorCount(groupnumber : u16) -> u32);
    GetMaximumProcessorCount(core::mem::transmute(groupnumber))
}
#[inline]
pub unsafe fn GetMaximumProcessorGroupCount() -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn GetMaximumProcessorGroupCount() -> u16);
    GetMaximumProcessorGroupCount()
}
#[inline]
pub unsafe fn GetNextUmsListItem(umscontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn GetNextUmsListItem(umscontext : *mut core::ffi::c_void) -> *mut core::ffi::c_void);
    GetNextUmsListItem(core::mem::transmute(umscontext))
}
#[inline]
pub unsafe fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNode(node : u8, availablebytes : *mut u64) -> super::super::Foundation:: BOOL);
    GetNumaAvailableMemoryNode(core::mem::transmute(node), core::mem::transmute(availablebytes)).ok()
}
#[inline]
pub unsafe fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNodeEx(node : u16, availablebytes : *mut u64) -> super::super::Foundation:: BOOL);
    GetNumaAvailableMemoryNodeEx(core::mem::transmute(node), core::mem::transmute(availablebytes)).ok()
}
#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : *mut u32) -> super::super::Foundation:: BOOL);
    GetNumaHighestNodeNumber(core::mem::transmute(highestnodenumber)).ok()
}
#[inline]
pub unsafe fn GetNumaNodeNumberFromHandle<P0>(hfile: P0, nodenumber: *mut u16) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeNumberFromHandle(hfile : super::super::Foundation:: HANDLE, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
    GetNumaNodeNumberFromHandle(hfile.param().abi(), core::mem::transmute(nodenumber)).ok()
}
#[inline]
pub unsafe fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask(node : u8, processormask : *mut u64) -> super::super::Foundation:: BOOL);
    GetNumaNodeProcessorMask(core::mem::transmute(node), core::mem::transmute(processormask)).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : u16, processormasks : *mut super::SystemInformation:: GROUP_AFFINITY, processormaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
    GetNumaNodeProcessorMask2(core::mem::transmute(nodenumber), core::mem::transmute(processormasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), processormasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(requiredmaskcount))
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : u16, processormask : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
    GetNumaNodeProcessorMaskEx(core::mem::transmute(node), core::mem::transmute(processormask))
}
#[inline]
pub unsafe fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaProcessorNode(processor : u8, nodenumber : *mut u8) -> super::super::Foundation:: BOOL);
    GetNumaProcessorNode(core::mem::transmute(processor), core::mem::transmute(nodenumber)).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaProcessorNodeEx(processor : *const super::Kernel:: PROCESSOR_NUMBER, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
    GetNumaProcessorNodeEx(core::mem::transmute(processor), core::mem::transmute(nodenumber)).ok()
}
#[inline]
pub unsafe fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaProximityNode(proximityid : u32, nodenumber : *mut u8) -> super::super::Foundation:: BOOL);
    GetNumaProximityNode(core::mem::transmute(proximityid), core::mem::transmute(nodenumber)).ok()
}
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : u32, nodenumber : *mut u16) -> super::super::Foundation:: BOOL);
    GetNumaProximityNodeEx(core::mem::transmute(proximityid), core::mem::transmute(nodenumber))
}
#[inline]
pub unsafe fn GetPriorityClass<P0>(hprocess: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetPriorityClass(hprocess : super::super::Foundation:: HANDLE) -> u32);
    GetPriorityClass(hprocess.param().abi())
}
#[inline]
pub unsafe fn GetProcessAffinityMask<P0>(hprocess: P0, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessAffinityMask(hprocess : super::super::Foundation:: HANDLE, lpprocessaffinitymask : *mut usize, lpsystemaffinitymask : *mut usize) -> super::super::Foundation:: BOOL);
    GetProcessAffinityMask(hprocess.param().abi(), core::mem::transmute(lpprocessaffinitymask), core::mem::transmute(lpsystemaffinitymask)).ok()
}
#[inline]
pub unsafe fn GetProcessDEPPolicy<P0>(hprocess: P0, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessDEPPolicy(hprocess : super::super::Foundation:: HANDLE, lpflags : *mut u32, lppermanent : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetProcessDEPPolicy(hprocess.param().abi(), core::mem::transmute(lpflags), core::mem::transmute(lppermanent)).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn GetProcessDefaultCpuSetMasks<P0>(process: P0, cpusetmasks: Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSetMasks(process : super::super::Foundation:: HANDLE, cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
    GetProcessDefaultCpuSetMasks(process.param().abi(), core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(requiredmaskcount))
}
#[inline]
pub unsafe fn GetProcessDefaultCpuSets<P0>(process: P0, cpusetids: Option<&mut [u32]>, requiredidcount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessDefaultCpuSets(process : super::super::Foundation:: HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> super::super::Foundation:: BOOL);
    GetProcessDefaultCpuSets(process.param().abi(), core::mem::transmute(cpusetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(requiredidcount))
}
#[inline]
pub unsafe fn GetProcessGroupAffinity<P0>(hprocess: P0, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::super::Foundation:: HANDLE, groupcount : *mut u16, grouparray : *mut u16) -> super::super::Foundation:: BOOL);
    GetProcessGroupAffinity(hprocess.param().abi(), core::mem::transmute(groupcount), core::mem::transmute(grouparray))
}
#[inline]
pub unsafe fn GetProcessHandleCount<P0>(hprocess: P0, pdwhandlecount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : super::super::Foundation:: HANDLE, pdwhandlecount : *mut u32) -> super::super::Foundation:: BOOL);
    GetProcessHandleCount(hprocess.param().abi(), core::mem::transmute(pdwhandlecount)).ok()
}
#[inline]
pub unsafe fn GetProcessHandleFromHwnd<P0>(hwnd: P0) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("oleacc.dll" "system" fn GetProcessHandleFromHwnd(hwnd : super::super::Foundation:: HWND) -> super::super::Foundation:: HANDLE);
    GetProcessHandleFromHwnd(hwnd.param().abi())
}
#[inline]
pub unsafe fn GetProcessId<P0>(process: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessId(process : super::super::Foundation:: HANDLE) -> u32);
    GetProcessId(process.param().abi())
}
#[inline]
pub unsafe fn GetProcessIdOfThread<P0>(thread: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessIdOfThread(thread : super::super::Foundation:: HANDLE) -> u32);
    GetProcessIdOfThread(thread.param().abi())
}
#[inline]
pub unsafe fn GetProcessInformation<P0>(hprocess: P0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut core::ffi::c_void, processinformationsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessInformation(hprocess : super::super::Foundation:: HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *mut core::ffi::c_void, processinformationsize : u32) -> super::super::Foundation:: BOOL);
    GetProcessInformation(hprocess.param().abi(), core::mem::transmute(processinformationclass), core::mem::transmute(processinformation), core::mem::transmute(processinformationsize)).ok()
}
#[inline]
pub unsafe fn GetProcessIoCounters<P0>(hprocess: P0, lpiocounters: *mut IO_COUNTERS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessIoCounters(hprocess : super::super::Foundation:: HANDLE, lpiocounters : *mut IO_COUNTERS) -> super::super::Foundation:: BOOL);
    GetProcessIoCounters(hprocess.param().abi(), core::mem::transmute(lpiocounters)).ok()
}
#[inline]
pub unsafe fn GetProcessMitigationPolicy<P0>(hprocess: P0, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut core::ffi::c_void, dwlength: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessMitigationPolicy(hprocess : super::super::Foundation:: HANDLE, mitigationpolicy : PROCESS_MITIGATION_POLICY, lpbuffer : *mut core::ffi::c_void, dwlength : usize) -> super::super::Foundation:: BOOL);
    GetProcessMitigationPolicy(hprocess.param().abi(), core::mem::transmute(mitigationpolicy), core::mem::transmute(lpbuffer), core::mem::transmute(dwlength)).ok()
}
#[inline]
pub unsafe fn GetProcessPriorityBoost<P0>(hprocess: P0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessPriorityBoost(hprocess : super::super::Foundation:: HANDLE, pdisablepriorityboost : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetProcessPriorityBoost(hprocess.param().abi(), core::mem::transmute(pdisablepriorityboost)).ok()
}
#[inline]
pub unsafe fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessShutdownParameters(lpdwlevel : *mut u32, lpdwflags : *mut u32) -> super::super::Foundation:: BOOL);
    GetProcessShutdownParameters(core::mem::transmute(lpdwlevel), core::mem::transmute(lpdwflags)).ok()
}
#[inline]
pub unsafe fn GetProcessTimes<P0>(hprocess: P0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : super::super::Foundation:: HANDLE, lpcreationtime : *mut super::super::Foundation:: FILETIME, lpexittime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    GetProcessTimes(hprocess.param().abi(), core::mem::transmute(lpcreationtime), core::mem::transmute(lpexittime), core::mem::transmute(lpkerneltime), core::mem::transmute(lpusertime)).ok()
}
#[inline]
pub unsafe fn GetProcessVersion(processid: u32) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessVersion(processid : u32) -> u32);
    GetProcessVersion(core::mem::transmute(processid))
}
#[inline]
pub unsafe fn GetProcessWorkingSetSize<P0>(hprocess: P0, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetProcessWorkingSetSize(hprocess : super::super::Foundation:: HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize) -> super::super::Foundation:: BOOL);
    GetProcessWorkingSetSize(hprocess.param().abi(), core::mem::transmute(lpminimumworkingsetsize), core::mem::transmute(lpmaximumworkingsetsize)).ok()
}
#[inline]
pub unsafe fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA) {
    windows_targets::link!("kernel32.dll" "system" fn GetStartupInfoA(lpstartupinfo : *mut STARTUPINFOA));
    GetStartupInfoA(core::mem::transmute(lpstartupinfo))
}
#[inline]
pub unsafe fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW) {
    windows_targets::link!("kernel32.dll" "system" fn GetStartupInfoW(lpstartupinfo : *mut STARTUPINFOW));
    GetStartupInfoW(core::mem::transmute(lpstartupinfo))
}
#[inline]
pub unsafe fn GetSystemTimes(lpidletime: Option<*mut super::super::Foundation::FILETIME>, lpkerneltime: Option<*mut super::super::Foundation::FILETIME>, lpusertime: Option<*mut super::super::Foundation::FILETIME>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemTimes(lpidletime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    GetSystemTimes(core::mem::transmute(lpidletime.unwrap_or(core::ptr::null_mut())), core::mem::transmute(lpkerneltime.unwrap_or(core::ptr::null_mut())), core::mem::transmute(lpusertime.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn GetThreadDescription<P0>(hthread: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadDescription(hthread : super::super::Foundation:: HANDLE, ppszthreaddescription : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    GetThreadDescription(hthread.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn GetThreadGroupAffinity<P0>(hthread: P0, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::super::Foundation:: HANDLE, groupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
    GetThreadGroupAffinity(hthread.param().abi(), core::mem::transmute(groupaffinity))
}
#[inline]
pub unsafe fn GetThreadIOPendingFlag<P0>(hthread: P0, lpioispending: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadIOPendingFlag(hthread : super::super::Foundation:: HANDLE, lpioispending : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetThreadIOPendingFlag(hthread.param().abi(), core::mem::transmute(lpioispending)).ok()
}
#[inline]
pub unsafe fn GetThreadId<P0>(thread: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadId(thread : super::super::Foundation:: HANDLE) -> u32);
    GetThreadId(thread.param().abi())
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn GetThreadIdealProcessorEx<P0>(hthread: P0, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadIdealProcessorEx(hthread : super::super::Foundation:: HANDLE, lpidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER) -> super::super::Foundation:: BOOL);
    GetThreadIdealProcessorEx(hthread.param().abi(), core::mem::transmute(lpidealprocessor)).ok()
}
#[inline]
pub unsafe fn GetThreadInformation<P0>(hthread: P0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut core::ffi::c_void, threadinformationsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadInformation(hthread : super::super::Foundation:: HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *mut core::ffi::c_void, threadinformationsize : u32) -> super::super::Foundation:: BOOL);
    GetThreadInformation(hthread.param().abi(), core::mem::transmute(threadinformationclass), core::mem::transmute(threadinformation), core::mem::transmute(threadinformationsize)).ok()
}
#[inline]
pub unsafe fn GetThreadPriority<P0>(hthread: P0) -> i32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadPriority(hthread : super::super::Foundation:: HANDLE) -> i32);
    GetThreadPriority(hthread.param().abi())
}
#[inline]
pub unsafe fn GetThreadPriorityBoost<P0>(hthread: P0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadPriorityBoost(hthread : super::super::Foundation:: HANDLE, pdisablepriorityboost : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetThreadPriorityBoost(hthread.param().abi(), core::mem::transmute(pdisablepriorityboost)).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn GetThreadSelectedCpuSetMasks<P0>(thread: P0, cpusetmasks: Option<&mut [super::SystemInformation::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSetMasks(thread : super::super::Foundation:: HANDLE, cpusetmasks : *mut super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16, requiredmaskcount : *mut u16) -> super::super::Foundation:: BOOL);
    GetThreadSelectedCpuSetMasks(thread.param().abi(), core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(requiredmaskcount))
}
#[inline]
pub unsafe fn GetThreadSelectedCpuSets<P0>(thread: P0, cpusetids: Option<&mut [u32]>, requiredidcount: *mut u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadSelectedCpuSets(thread : super::super::Foundation:: HANDLE, cpusetids : *mut u32, cpusetidcount : u32, requiredidcount : *mut u32) -> super::super::Foundation:: BOOL);
    GetThreadSelectedCpuSets(thread.param().abi(), core::mem::transmute(cpusetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(requiredidcount))
}
#[inline]
pub unsafe fn GetThreadTimes<P0>(hthread: P0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetThreadTimes(hthread : super::super::Foundation:: HANDLE, lpcreationtime : *mut super::super::Foundation:: FILETIME, lpexittime : *mut super::super::Foundation:: FILETIME, lpkerneltime : *mut super::super::Foundation:: FILETIME, lpusertime : *mut super::super::Foundation:: FILETIME) -> super::super::Foundation:: BOOL);
    GetThreadTimes(hthread.param().abi(), core::mem::transmute(lpcreationtime), core::mem::transmute(lpexittime), core::mem::transmute(lpkerneltime), core::mem::transmute(lpusertime)).ok()
}
#[inline]
pub unsafe fn GetUmsCompletionListEvent(umscompletionlist: *const core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetUmsCompletionListEvent(umscompletionlist : *const core::ffi::c_void, umscompletionevent : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    GetUmsCompletionListEvent(core::mem::transmute(umscompletionlist), core::mem::transmute(umscompletionevent)).ok()
}
#[inline]
pub unsafe fn GetUmsSystemThreadInformation<P0>(threadhandle: P0, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetUmsSystemThreadInformation(threadhandle : super::super::Foundation:: HANDLE, systemthreadinfo : *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation:: BOOL);
    GetUmsSystemThreadInformation(threadhandle.param().abi(), core::mem::transmute(systemthreadinfo))
}
#[inline]
pub unsafe fn InitOnceBeginInitialize(lpinitonce: *mut INIT_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitOnceBeginInitialize(lpinitonce : *mut INIT_ONCE, dwflags : u32, fpending : *mut super::super::Foundation:: BOOL, lpcontext : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    InitOnceBeginInitialize(core::mem::transmute(lpinitonce), core::mem::transmute(dwflags), core::mem::transmute(fpending), core::mem::transmute(lpcontext.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn InitOnceComplete(lpinitonce: *mut INIT_ONCE, dwflags: u32, lpcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitOnceComplete(lpinitonce : *mut INIT_ONCE, dwflags : u32, lpcontext : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    InitOnceComplete(core::mem::transmute(lpinitonce), core::mem::transmute(dwflags), core::mem::transmute(lpcontext.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn InitOnceExecuteOnce(initonce: *mut INIT_ONCE, initfn: PINIT_ONCE_FN, parameter: Option<*mut core::ffi::c_void>, context: Option<*mut *mut core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitOnceExecuteOnce(initonce : *mut INIT_ONCE, initfn : PINIT_ONCE_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    InitOnceExecuteOnce(core::mem::transmute(initonce), core::mem::transmute(initfn), core::mem::transmute(parameter.unwrap_or(core::ptr::null_mut())), core::mem::transmute(context.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn InitOnceInitialize() -> INIT_ONCE {
    windows_targets::link!("kernel32.dll" "system" fn InitOnceInitialize(initonce : *mut INIT_ONCE));
    let mut result__ = core::mem::zeroed();
    InitOnceInitialize(&mut result__);
    result__
}
#[inline]
pub unsafe fn InitializeConditionVariable() -> CONDITION_VARIABLE {
    windows_targets::link!("kernel32.dll" "system" fn InitializeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
    let mut result__ = core::mem::zeroed();
    InitializeConditionVariable(&mut result__);
    result__
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION) {
    windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
    InitializeCriticalSection(core::mem::transmute(lpcriticalsection))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut CRITICAL_SECTION, dwspincount: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSectionAndSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> super::super::Foundation:: BOOL);
    InitializeCriticalSectionAndSpinCount(core::mem::transmute(lpcriticalsection), core::mem::transmute(dwspincount)).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeCriticalSectionEx(lpcriticalsection: *mut CRITICAL_SECTION, dwspincount: u32, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitializeCriticalSectionEx(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32, flags : u32) -> super::super::Foundation:: BOOL);
    InitializeCriticalSectionEx(core::mem::transmute(lpcriticalsection), core::mem::transmute(dwspincount), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitializeProcThreadAttributeList(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount : u32, dwflags : u32, lpsize : *mut usize) -> super::super::Foundation:: BOOL);
    InitializeProcThreadAttributeList(core::mem::transmute(lpattributelist), core::mem::transmute(dwattributecount), core::mem::transmute(dwflags), core::mem::transmute(lpsize)).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeSListHead() -> super::Kernel::SLIST_HEADER {
    windows_targets::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : *mut super::Kernel:: SLIST_HEADER));
    let mut result__ = core::mem::zeroed();
    InitializeSListHead(&mut result__);
    result__
}
#[inline]
pub unsafe fn InitializeSRWLock() -> SRWLOCK {
    windows_targets::link!("kernel32.dll" "system" fn InitializeSRWLock(srwlock : *mut SRWLOCK));
    let mut result__ = core::mem::zeroed();
    InitializeSRWLock(&mut result__);
    result__
}
#[inline]
pub unsafe fn InitializeSynchronizationBarrier(lpbarrier: *mut SYNCHRONIZATION_BARRIER, ltotalthreads: i32, lspincount: i32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn InitializeSynchronizationBarrier(lpbarrier : *mut SYNCHRONIZATION_BARRIER, ltotalthreads : i32, lspincount : i32) -> super::super::Foundation:: BOOL);
    InitializeSynchronizationBarrier(core::mem::transmute(lpbarrier), core::mem::transmute(ltotalthreads), core::mem::transmute(lspincount)).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    windows_targets::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : *mut super::Kernel:: SLIST_HEADER) -> *mut super::Kernel:: SLIST_ENTRY);
    InterlockedFlushSList(core::mem::transmute(listhead))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    windows_targets::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : *mut super::Kernel:: SLIST_HEADER) -> *mut super::Kernel:: SLIST_ENTRY);
    InterlockedPopEntrySList(core::mem::transmute(listhead))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY {
    windows_targets::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::Kernel:: SLIST_HEADER, listentry : *mut super::Kernel:: SLIST_ENTRY) -> *mut super::Kernel:: SLIST_ENTRY);
    InterlockedPushEntrySList(core::mem::transmute(listhead), core::mem::transmute(listentry))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY {
    windows_targets::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::Kernel:: SLIST_HEADER, list : *mut super::Kernel:: SLIST_ENTRY, listend : *mut super::Kernel:: SLIST_ENTRY, count : u32) -> *mut super::Kernel:: SLIST_ENTRY);
    InterlockedPushListSListEx(core::mem::transmute(listhead), core::mem::transmute(list), core::mem::transmute(listend), core::mem::transmute(count))
}
#[inline]
pub unsafe fn IsImmersiveProcess<P0>(hprocess: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn IsImmersiveProcess(hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    IsImmersiveProcess(hprocess.param().abi()).ok()
}
#[inline]
pub unsafe fn IsProcessCritical<P0>(hprocess: P0, critical: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsProcessCritical(hprocess : super::super::Foundation:: HANDLE, critical : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsProcessCritical(hprocess.param().abi(), core::mem::transmute(critical)).ok()
}
#[inline]
pub unsafe fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsProcessorFeaturePresent(processorfeature : PROCESSOR_FEATURE_ID) -> super::super::Foundation:: BOOL);
    IsProcessorFeaturePresent(core::mem::transmute(processorfeature))
}
#[inline]
pub unsafe fn IsThreadAFiber() -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsThreadAFiber() -> super::super::Foundation:: BOOL);
    IsThreadAFiber()
}
#[inline]
pub unsafe fn IsThreadpoolTimerSet<P0>(pti: P0) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<PTP_TIMER>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsThreadpoolTimerSet(pti : PTP_TIMER) -> super::super::Foundation:: BOOL);
    IsThreadpoolTimerSet(pti.param().abi())
}
#[inline]
pub unsafe fn IsWow64Process<P0>(hprocess: P0, wow64process: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsWow64Process(hprocess : super::super::Foundation:: HANDLE, wow64process : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsWow64Process(hprocess.param().abi(), core::mem::transmute(wow64process)).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn IsWow64Process2<P0>(hprocess: P0, pprocessmachine: *mut super::SystemInformation::IMAGE_FILE_MACHINE, pnativemachine: Option<*mut super::SystemInformation::IMAGE_FILE_MACHINE>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsWow64Process2(hprocess : super::super::Foundation:: HANDLE, pprocessmachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE, pnativemachine : *mut super::SystemInformation:: IMAGE_FILE_MACHINE) -> super::super::Foundation:: BOOL);
    IsWow64Process2(hprocess.param().abi(), core::mem::transmute(pprocessmachine), core::mem::transmute(pnativemachine.unwrap_or(core::ptr::null_mut()))).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn LeaveCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION) {
    windows_targets::link!("kernel32.dll" "system" fn LeaveCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION));
    LeaveCriticalSection(core::mem::transmute(lpcriticalsection))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns<P0>(pci: P0, pcs: *mut CRITICAL_SECTION)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
{
    windows_targets::link!("kernel32.dll" "system" fn LeaveCriticalSectionWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, pcs : *mut CRITICAL_SECTION));
    LeaveCriticalSectionWhenCallbackReturns(pci.param().abi(), core::mem::transmute(pcs))
}
#[inline]
pub unsafe fn OpenEventA<P1, P2>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenEventA(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenEventA(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenEventW<P1, P2>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenEventW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenEventW(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenMutexW<P1, P2>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenMutexW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenMutexW(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenPrivateNamespaceA<P1>(lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P1) -> super::super::Foundation::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenPrivateNamespaceA(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    OpenPrivateNamespaceA(core::mem::transmute(lpboundarydescriptor), lpaliasprefix.param().abi())
}
#[inline]
pub unsafe fn OpenPrivateNamespaceW<P1>(lpboundarydescriptor: *const core::ffi::c_void, lpaliasprefix: P1) -> super::super::Foundation::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenPrivateNamespaceW(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    OpenPrivateNamespaceW(core::mem::transmute(lpboundarydescriptor), lpaliasprefix.param().abi())
}
#[inline]
pub unsafe fn OpenProcess<P1>(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: P1, dwprocessid: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : PROCESS_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, dwprocessid : u32) -> super::super::Foundation:: HANDLE);
    let result__ = OpenProcess(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), core::mem::transmute(dwprocessid));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenProcessToken<P0>(processhandle: P0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : super::super::Foundation:: HANDLE, desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK, tokenhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    OpenProcessToken(processhandle.param().abi(), core::mem::transmute(desiredaccess), core::mem::transmute(tokenhandle)).ok()
}
#[inline]
pub unsafe fn OpenSemaphoreW<P1, P2>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P1, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenSemaphoreW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenSemaphoreW(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenThread<P1>(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: P1, dwthreadid: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenThread(dwdesiredaccess : THREAD_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, dwthreadid : u32) -> super::super::Foundation:: HANDLE);
    let result__ = OpenThread(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), core::mem::transmute(dwthreadid));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenThreadToken<P0, P2>(threadhandle: P0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: P2, tokenhandle: *mut super::super::Foundation::HANDLE) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("advapi32.dll" "system" fn OpenThreadToken(threadhandle : super::super::Foundation:: HANDLE, desiredaccess : super::super::Security:: TOKEN_ACCESS_MASK, openasself : super::super::Foundation:: BOOL, tokenhandle : *mut super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    OpenThreadToken(threadhandle.param().abi(), core::mem::transmute(desiredaccess), openasself.param().abi(), core::mem::transmute(tokenhandle)).ok()
}
#[inline]
pub unsafe fn OpenWaitableTimerA<P1, P2>(dwdesiredaccess: u32, binherithandle: P1, lptimername: P2) -> super::super::Foundation::HANDLE
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenWaitableTimerA(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lptimername : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    OpenWaitableTimerA(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lptimername.param().abi())
}
#[inline]
pub unsafe fn OpenWaitableTimerW<P1, P2>(dwdesiredaccess: SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle: P1, lptimername: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenWaitableTimerW(dwdesiredaccess : SYNCHRONIZATION_ACCESS_RIGHTS, binherithandle : super::super::Foundation:: BOOL, lptimername : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenWaitableTimerW(core::mem::transmute(dwdesiredaccess), binherithandle.param().abi(), lptimername.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn PulseEvent<P0>(hevent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn PulseEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    PulseEvent(hevent.param().abi()).ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16 {
    windows_targets::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : *const super::Kernel:: SLIST_HEADER) -> u16);
    QueryDepthSList(core::mem::transmute(listhead))
}
#[inline]
pub unsafe fn QueryFullProcessImageNameA<P0>(hprocess: P0, dwflags: PROCESS_NAME_FORMAT, lpexename: windows_core::PSTR, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryFullProcessImageNameA(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_NAME_FORMAT, lpexename : windows_core::PSTR, lpdwsize : *mut u32) -> super::super::Foundation:: BOOL);
    QueryFullProcessImageNameA(hprocess.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(lpexename), core::mem::transmute(lpdwsize)).ok()
}
#[inline]
pub unsafe fn QueryFullProcessImageNameW<P0>(hprocess: P0, dwflags: PROCESS_NAME_FORMAT, lpexename: windows_core::PWSTR, lpdwsize: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_NAME_FORMAT, lpexename : windows_core::PWSTR, lpdwsize : *mut u32) -> super::super::Foundation:: BOOL);
    QueryFullProcessImageNameW(hprocess.param().abi(), core::mem::transmute(dwflags), core::mem::transmute(lpexename), core::mem::transmute(lpdwsize)).ok()
}
#[inline]
pub unsafe fn QueryProcessAffinityUpdateMode<P0>(hprocess: P0, lpdwflags: Option<*mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryProcessAffinityUpdateMode(hprocess : super::super::Foundation:: HANDLE, lpdwflags : *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
    QueryProcessAffinityUpdateMode(hprocess.param().abi(), core::mem::transmute(lpdwflags.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn QueryProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : *mut usize) -> super::super::Foundation:: BOOL);
    QueryProtectedPolicy(core::mem::transmute(policyguid), core::mem::transmute(policyvalue))
}
#[inline]
pub unsafe fn QueryThreadpoolStackInformation<P0>(ptpp: P0, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<PTP_POOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryThreadpoolStackInformation(ptpp : PTP_POOL, ptpsi : *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation:: BOOL);
    QueryThreadpoolStackInformation(ptpp.param().abi(), core::mem::transmute(ptpsi)).ok()
}
#[inline]
pub unsafe fn QueryUmsThreadInformation(umsthread: *const core::ffi::c_void, umsthreadinfoclass: UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut core::ffi::c_void, umsthreadinformationlength: u32, returnlength: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn QueryUmsThreadInformation(umsthread : *const core::ffi::c_void, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *mut core::ffi::c_void, umsthreadinformationlength : u32, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
    QueryUmsThreadInformation(core::mem::transmute(umsthread), core::mem::transmute(umsthreadinfoclass), core::mem::transmute(umsthreadinformation), core::mem::transmute(umsthreadinformationlength), core::mem::transmute(returnlength.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn QueueUserAPC<P1>(pfnapc: super::super::Foundation::PAPCFUNC, hthread: P1, dwdata: usize) -> u32
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueueUserAPC(pfnapc : super::super::Foundation:: PAPCFUNC, hthread : super::super::Foundation:: HANDLE, dwdata : usize) -> u32);
    QueueUserAPC(core::mem::transmute(pfnapc), hthread.param().abi(), core::mem::transmute(dwdata))
}
#[inline]
pub unsafe fn QueueUserAPC2<P1>(apcroutine: super::super::Foundation::PAPCFUNC, thread: P1, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueueUserAPC2(apcroutine : super::super::Foundation:: PAPCFUNC, thread : super::super::Foundation:: HANDLE, data : usize, flags : QUEUE_USER_APC_FLAGS) -> super::super::Foundation:: BOOL);
    QueueUserAPC2(core::mem::transmute(apcroutine), thread.param().abi(), core::mem::transmute(data), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: Option<*const core::ffi::c_void>, flags: WORKER_THREAD_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn QueueUserWorkItem(function : LPTHREAD_START_ROUTINE, context : *const core::ffi::c_void, flags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
    QueueUserWorkItem(core::mem::transmute(function), core::mem::transmute(context.unwrap_or(core::ptr::null())), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn RegisterWaitForSingleObject<P1>(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: P1, callback: WAITORTIMERCALLBACK, context: Option<*const core::ffi::c_void>, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn RegisterWaitForSingleObject(phnewwaitobject : *mut super::super::Foundation:: HANDLE, hobject : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, context : *const core::ffi::c_void, dwmilliseconds : u32, dwflags : WORKER_THREAD_FLAGS) -> super::super::Foundation:: BOOL);
    RegisterWaitForSingleObject(core::mem::transmute(phnewwaitobject), hobject.param().abi(), core::mem::transmute(callback), core::mem::transmute(context.unwrap_or(core::ptr::null())), core::mem::transmute(dwmilliseconds), core::mem::transmute(dwflags)).ok()
}
#[inline]
pub unsafe fn ReleaseMutex<P0>(hmutex: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ReleaseMutex(hmutex.param().abi()).ok()
}
#[inline]
pub unsafe fn ReleaseMutexWhenCallbackReturns<P0, P1>(pci: P0, r#mut: P1)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReleaseMutexWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, r#mut : super::super::Foundation:: HANDLE));
    ReleaseMutexWhenCallbackReturns(pci.param().abi(), r#mut.param().abi())
}
#[inline]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: *mut SRWLOCK) {
    windows_targets::link!("kernel32.dll" "system" fn ReleaseSRWLockExclusive(srwlock : *mut SRWLOCK));
    ReleaseSRWLockExclusive(core::mem::transmute(srwlock))
}
#[inline]
pub unsafe fn ReleaseSRWLockShared(srwlock: *mut SRWLOCK) {
    windows_targets::link!("kernel32.dll" "system" fn ReleaseSRWLockShared(srwlock : *mut SRWLOCK));
    ReleaseSRWLockShared(core::mem::transmute(srwlock))
}
#[inline]
pub unsafe fn ReleaseSemaphore<P0>(hsemaphore: P0, lreleasecount: i32, lppreviouscount: Option<*mut i32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReleaseSemaphore(hsemaphore : super::super::Foundation:: HANDLE, lreleasecount : i32, lppreviouscount : *mut i32) -> super::super::Foundation:: BOOL);
    ReleaseSemaphore(hsemaphore.param().abi(), core::mem::transmute(lreleasecount), core::mem::transmute(lppreviouscount.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns<P0, P1>(pci: P0, sem: P1, crel: u32)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ReleaseSemaphoreWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, sem : super::super::Foundation:: HANDLE, crel : u32));
    ReleaseSemaphoreWhenCallbackReturns(pci.param().abi(), sem.param().abi(), core::mem::transmute(crel))
}
#[inline]
pub unsafe fn ResetEvent<P0>(hevent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ResetEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ResetEvent(hevent.param().abi()).ok()
}
#[inline]
pub unsafe fn ResumeThread<P0>(hthread: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn ResumeThread(hthread : super::super::Foundation:: HANDLE) -> u32);
    ResumeThread(hthread.param().abi())
}
#[inline]
pub unsafe fn RtwqAddPeriodicCallback<P1>(callback: RTWQPERIODICCALLBACK, context: P1, key: Option<*mut u32>) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqAddPeriodicCallback(callback : RTWQPERIODICCALLBACK, context : * mut core::ffi::c_void, key : *mut u32) -> windows_core::HRESULT);
    RtwqAddPeriodicCallback(core::mem::transmute(callback), context.param().abi(), core::mem::transmute(key.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RtwqAllocateSerialWorkQueue(workqueueidin: u32) -> windows_core::Result<u32> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqAllocateSerialWorkQueue(workqueueidin : u32, workqueueidout : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqAllocateSerialWorkQueue(core::mem::transmute(workqueueidin), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqAllocateWorkQueue(workqueuetype: RTWQ_WORKQUEUE_TYPE) -> windows_core::Result<u32> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqAllocateWorkQueue(workqueuetype : RTWQ_WORKQUEUE_TYPE, workqueueid : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqAllocateWorkQueue(core::mem::transmute(workqueuetype), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqBeginRegisterWorkQueueWithMMCSS<P1, P4, P5>(workqueueid: u32, usageclass: P1, dwtaskid: u32, lpriority: i32, donecallback: P4, donestate: P5) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<IRtwqAsyncCallback>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqBeginRegisterWorkQueueWithMMCSS(workqueueid : u32, usageclass : windows_core::PCWSTR, dwtaskid : u32, lpriority : i32, donecallback : * mut core::ffi::c_void, donestate : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqBeginRegisterWorkQueueWithMMCSS(core::mem::transmute(workqueueid), usageclass.param().abi(), core::mem::transmute(dwtaskid), core::mem::transmute(lpriority), donecallback.param().abi(), donestate.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqBeginUnregisterWorkQueueWithMMCSS<P1, P2>(workqueueid: u32, donecallback: P1, donestate: P2) -> windows_core::Result<()>
where
    P1: windows_core::Param<IRtwqAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqBeginUnregisterWorkQueueWithMMCSS(workqueueid : u32, donecallback : * mut core::ffi::c_void, donestate : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqBeginUnregisterWorkQueueWithMMCSS(core::mem::transmute(workqueueid), donecallback.param().abi(), donestate.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqCancelDeadline<P0>(prequest: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqCancelDeadline(prequest : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RtwqCancelDeadline(prequest.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqCancelWorkItem(key: u64) -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqCancelWorkItem(key : u64) -> windows_core::HRESULT);
    RtwqCancelWorkItem(core::mem::transmute(key)).ok()
}
#[inline]
pub unsafe fn RtwqCreateAsyncResult<P0, P1, P2>(appobject: P0, callback: P1, appstate: P2) -> windows_core::Result<IRtwqAsyncResult>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<IRtwqAsyncCallback>,
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqCreateAsyncResult(appobject : * mut core::ffi::c_void, callback : * mut core::ffi::c_void, appstate : * mut core::ffi::c_void, asyncresult : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqCreateAsyncResult(appobject.param().abi(), callback.param().abi(), appstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
}
#[inline]
pub unsafe fn RtwqEndRegisterWorkQueueWithMMCSS<P0>(result: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqEndRegisterWorkQueueWithMMCSS(result : * mut core::ffi::c_void, taskid : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqEndRegisterWorkQueueWithMMCSS(result.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSClass(workqueueid: u32, usageclass: windows_core::PWSTR, usageclasslength: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSClass(workqueueid : u32, usageclass : windows_core::PWSTR, usageclasslength : *mut u32) -> windows_core::HRESULT);
    RtwqGetWorkQueueMMCSSClass(core::mem::transmute(workqueueid), core::mem::transmute(usageclass), core::mem::transmute(usageclasslength)).ok()
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSPriority(workqueueid: u32) -> windows_core::Result<i32> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSPriority(workqueueid : u32, priority : *mut i32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqGetWorkQueueMMCSSPriority(core::mem::transmute(workqueueid), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqGetWorkQueueMMCSSTaskId(workqueueid: u32) -> windows_core::Result<u32> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqGetWorkQueueMMCSSTaskId(workqueueid : u32, taskid : *mut u32) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqGetWorkQueueMMCSSTaskId(core::mem::transmute(workqueueid), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqInvokeCallback<P0>(result: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqInvokeCallback(result : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqInvokeCallback(result.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqJoinWorkQueue<P1>(workqueueid: u32, hfile: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqJoinWorkQueue(workqueueid : u32, hfile : super::super::Foundation:: HANDLE, out : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqJoinWorkQueue(core::mem::transmute(workqueueid), hfile.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqLockPlatform() -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqLockPlatform() -> windows_core::HRESULT);
    RtwqLockPlatform().ok()
}
#[inline]
pub unsafe fn RtwqLockSharedWorkQueue<P0>(usageclass: P0, basepriority: i32, taskid: *mut u32, id: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqLockSharedWorkQueue(usageclass : windows_core::PCWSTR, basepriority : i32, taskid : *mut u32, id : *mut u32) -> windows_core::HRESULT);
    RtwqLockSharedWorkQueue(usageclass.param().abi(), core::mem::transmute(basepriority), core::mem::transmute(taskid), core::mem::transmute(id)).ok()
}
#[inline]
pub unsafe fn RtwqLockWorkQueue(workqueueid: u32) -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqLockWorkQueue(workqueueid : u32) -> windows_core::HRESULT);
    RtwqLockWorkQueue(core::mem::transmute(workqueueid)).ok()
}
#[inline]
pub unsafe fn RtwqPutWaitingWorkItem<P0, P2>(hevent: P0, lpriority: i32, result: P2, key: Option<*mut u64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<IRtwqAsyncResult>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqPutWaitingWorkItem(hevent : super::super::Foundation:: HANDLE, lpriority : i32, result : * mut core::ffi::c_void, key : *mut u64) -> windows_core::HRESULT);
    RtwqPutWaitingWorkItem(hevent.param().abi(), core::mem::transmute(lpriority), result.param().abi(), core::mem::transmute(key.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RtwqPutWorkItem<P2>(dwqueue: u32, lpriority: i32, result: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<IRtwqAsyncResult>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqPutWorkItem(dwqueue : u32, lpriority : i32, result : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqPutWorkItem(core::mem::transmute(dwqueue), core::mem::transmute(lpriority), result.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqRegisterPlatformEvents<P0>(platformevents: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRtwqPlatformEvents>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformEvents(platformevents : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqRegisterPlatformEvents(platformevents.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqRegisterPlatformWithMMCSS<P0>(usageclass: P0, taskid: *mut u32, lpriority: i32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqRegisterPlatformWithMMCSS(usageclass : windows_core::PCWSTR, taskid : *mut u32, lpriority : i32) -> windows_core::HRESULT);
    RtwqRegisterPlatformWithMMCSS(usageclass.param().abi(), core::mem::transmute(taskid), core::mem::transmute(lpriority)).ok()
}
#[inline]
pub unsafe fn RtwqRemovePeriodicCallback(dwkey: u32) -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqRemovePeriodicCallback(dwkey : u32) -> windows_core::HRESULT);
    RtwqRemovePeriodicCallback(core::mem::transmute(dwkey)).ok()
}
#[inline]
pub unsafe fn RtwqScheduleWorkItem<P0>(result: P0, timeout: i64, key: Option<*mut u64>) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRtwqAsyncResult>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqScheduleWorkItem(result : * mut core::ffi::c_void, timeout : i64, key : *mut u64) -> windows_core::HRESULT);
    RtwqScheduleWorkItem(result.param().abi(), core::mem::transmute(timeout), core::mem::transmute(key.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn RtwqSetDeadline(workqueueid: u32, deadlineinhns: i64) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqSetDeadline(workqueueid : u32, deadlineinhns : i64, prequest : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqSetDeadline(core::mem::transmute(workqueueid), core::mem::transmute(deadlineinhns), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqSetDeadline2(workqueueid: u32, deadlineinhns: i64, predeadlineinhns: i64) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqSetDeadline2(workqueueid : u32, deadlineinhns : i64, predeadlineinhns : i64, prequest : *mut super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    let mut result__ = core::mem::zeroed();
    RtwqSetDeadline2(core::mem::transmute(workqueueid), core::mem::transmute(deadlineinhns), core::mem::transmute(predeadlineinhns), &mut result__).map(|| core::mem::transmute(result__))
}
#[inline]
pub unsafe fn RtwqSetLongRunning<P1>(workqueueid: u32, enable: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqSetLongRunning(workqueueid : u32, enable : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    RtwqSetLongRunning(core::mem::transmute(workqueueid), enable.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqShutdown() -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqShutdown() -> windows_core::HRESULT);
    RtwqShutdown().ok()
}
#[inline]
pub unsafe fn RtwqStartup() -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqStartup() -> windows_core::HRESULT);
    RtwqStartup().ok()
}
#[inline]
pub unsafe fn RtwqUnjoinWorkQueue<P1>(workqueueid: u32, hfile: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqUnjoinWorkQueue(workqueueid : u32, hfile : super::super::Foundation:: HANDLE) -> windows_core::HRESULT);
    RtwqUnjoinWorkQueue(core::mem::transmute(workqueueid), hfile.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqUnlockPlatform() -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqUnlockPlatform() -> windows_core::HRESULT);
    RtwqUnlockPlatform().ok()
}
#[inline]
pub unsafe fn RtwqUnlockWorkQueue(workqueueid: u32) -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqUnlockWorkQueue(workqueueid : u32) -> windows_core::HRESULT);
    RtwqUnlockWorkQueue(core::mem::transmute(workqueueid)).ok()
}
#[inline]
pub unsafe fn RtwqUnregisterPlatformEvents<P0>(platformevents: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRtwqPlatformEvents>,
{
    windows_targets::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformEvents(platformevents : * mut core::ffi::c_void) -> windows_core::HRESULT);
    RtwqUnregisterPlatformEvents(platformevents.param().abi()).ok()
}
#[inline]
pub unsafe fn RtwqUnregisterPlatformFromMMCSS() -> windows_core::Result<()> {
    windows_targets::link!("rtworkq.dll" "system" fn RtwqUnregisterPlatformFromMMCSS() -> windows_core::HRESULT);
    RtwqUnregisterPlatformFromMMCSS().ok()
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn SetCriticalSectionSpinCount(lpcriticalsection: *mut CRITICAL_SECTION, dwspincount: u32) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn SetCriticalSectionSpinCount(lpcriticalsection : *mut CRITICAL_SECTION, dwspincount : u32) -> u32);
    SetCriticalSectionSpinCount(core::mem::transmute(lpcriticalsection), core::mem::transmute(dwspincount))
}
#[inline]
pub unsafe fn SetEvent<P0>(hevent: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEvent(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetEvent(hevent.param().abi()).ok()
}
#[inline]
pub unsafe fn SetEventWhenCallbackReturns<P0, P1>(pci: P0, evt: P1)
where
    P0: windows_core::Param<PTP_CALLBACK_INSTANCE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetEventWhenCallbackReturns(pci : PTP_CALLBACK_INSTANCE, evt : super::super::Foundation:: HANDLE));
    SetEventWhenCallbackReturns(pci.param().abi(), evt.param().abi())
}
#[inline]
pub unsafe fn SetPriorityClass<P0>(hprocess: P0, dwpriorityclass: PROCESS_CREATION_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetPriorityClass(hprocess : super::super::Foundation:: HANDLE, dwpriorityclass : PROCESS_CREATION_FLAGS) -> super::super::Foundation:: BOOL);
    SetPriorityClass(hprocess.param().abi(), core::mem::transmute(dwpriorityclass)).ok()
}
#[inline]
pub unsafe fn SetProcessAffinityMask<P0>(hprocess: P0, dwprocessaffinitymask: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessAffinityMask(hprocess : super::super::Foundation:: HANDLE, dwprocessaffinitymask : usize) -> super::super::Foundation:: BOOL);
    SetProcessAffinityMask(hprocess.param().abi(), core::mem::transmute(dwprocessaffinitymask)).ok()
}
#[inline]
pub unsafe fn SetProcessAffinityUpdateMode<P0>(hprocess: P0, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessAffinityUpdateMode(hprocess : super::super::Foundation:: HANDLE, dwflags : PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation:: BOOL);
    SetProcessAffinityUpdateMode(hprocess.param().abi(), core::mem::transmute(dwflags)).ok()
}
#[inline]
pub unsafe fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetProcessDEPPolicy(dwflags : PROCESS_DEP_FLAGS) -> super::super::Foundation:: BOOL);
    SetProcessDEPPolicy(core::mem::transmute(dwflags)).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn SetProcessDefaultCpuSetMasks<P0>(process: P0, cpusetmasks: Option<&[super::SystemInformation::GROUP_AFFINITY]>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSetMasks(process : super::super::Foundation:: HANDLE, cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16) -> super::super::Foundation:: BOOL);
    SetProcessDefaultCpuSetMasks(process.param().abi(), core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SetProcessDefaultCpuSets<P0>(process: P0, cpusetids: Option<&[u32]>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessDefaultCpuSets(process : super::super::Foundation:: HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> super::super::Foundation:: BOOL);
    SetProcessDefaultCpuSets(process.param().abi(), core::mem::transmute(cpusetids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SetProcessDynamicEHContinuationTargets<P0>(process: P0, targets: &mut [PROCESS_DYNAMIC_EH_CONTINUATION_TARGET]) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessDynamicEHContinuationTargets(process : super::super::Foundation:: HANDLE, numberoftargets : u16, targets : *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation:: BOOL);
    SetProcessDynamicEHContinuationTargets(process.param().abi(), targets.len().try_into().unwrap(), core::mem::transmute(targets.as_ptr())).ok()
}
#[inline]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges<P0>(process: P0, ranges: &mut [PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE]) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessDynamicEnforcedCetCompatibleRanges(process : super::super::Foundation:: HANDLE, numberofranges : u16, ranges : *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation:: BOOL);
    SetProcessDynamicEnforcedCetCompatibleRanges(process.param().abi(), ranges.len().try_into().unwrap(), core::mem::transmute(ranges.as_ptr()))
}
#[inline]
pub unsafe fn SetProcessInformation<P0>(hprocess: P0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const core::ffi::c_void, processinformationsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessInformation(hprocess : super::super::Foundation:: HANDLE, processinformationclass : PROCESS_INFORMATION_CLASS, processinformation : *const core::ffi::c_void, processinformationsize : u32) -> super::super::Foundation:: BOOL);
    SetProcessInformation(hprocess.param().abi(), core::mem::transmute(processinformationclass), core::mem::transmute(processinformation), core::mem::transmute(processinformationsize)).ok()
}
#[inline]
pub unsafe fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const core::ffi::c_void, dwlength: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetProcessMitigationPolicy(mitigationpolicy : PROCESS_MITIGATION_POLICY, lpbuffer : *const core::ffi::c_void, dwlength : usize) -> super::super::Foundation:: BOOL);
    SetProcessMitigationPolicy(core::mem::transmute(mitigationpolicy), core::mem::transmute(lpbuffer), core::mem::transmute(dwlength)).ok()
}
#[inline]
pub unsafe fn SetProcessPriorityBoost<P0, P1>(hprocess: P0, bdisablepriorityboost: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessPriorityBoost(hprocess : super::super::Foundation:: HANDLE, bdisablepriorityboost : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetProcessPriorityBoost(hprocess.param().abi(), bdisablepriorityboost.param().abi()).ok()
}
#[inline]
pub unsafe fn SetProcessRestrictionExemption<P0>(fenableexemption: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn SetProcessRestrictionExemption(fenableexemption : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetProcessRestrictionExemption(fenableexemption.param().abi()).ok()
}
#[inline]
pub unsafe fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetProcessShutdownParameters(dwlevel : u32, dwflags : u32) -> super::super::Foundation:: BOOL);
    SetProcessShutdownParameters(core::mem::transmute(dwlevel), core::mem::transmute(dwflags)).ok()
}
#[inline]
pub unsafe fn SetProcessWorkingSetSize<P0>(hprocess: P0, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetProcessWorkingSetSize(hprocess : super::super::Foundation:: HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize) -> super::super::Foundation:: BOOL);
    SetProcessWorkingSetSize(hprocess.param().abi(), core::mem::transmute(dwminimumworkingsetsize), core::mem::transmute(dwmaximumworkingsetsize)).ok()
}
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: *const windows_core::GUID, policyvalue: usize, oldpolicyvalue: Option<*mut usize>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetProtectedPolicy(policyguid : *const windows_core::GUID, policyvalue : usize, oldpolicyvalue : *mut usize) -> super::super::Foundation:: BOOL);
    SetProtectedPolicy(core::mem::transmute(policyguid), core::mem::transmute(policyvalue), core::mem::transmute(oldpolicyvalue.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetThreadAffinityMask<P0>(hthread: P0, dwthreadaffinitymask: usize) -> usize
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadAffinityMask(hthread : super::super::Foundation:: HANDLE, dwthreadaffinitymask : usize) -> usize);
    SetThreadAffinityMask(hthread.param().abi(), core::mem::transmute(dwthreadaffinitymask))
}
#[inline]
pub unsafe fn SetThreadDescription<P0, P1>(hthread: P0, lpthreaddescription: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadDescription(hthread : super::super::Foundation:: HANDLE, lpthreaddescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    SetThreadDescription(hthread.param().abi(), lpthreaddescription.param().abi()).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn SetThreadGroupAffinity<P0>(hthread: P0, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: Option<*mut super::SystemInformation::GROUP_AFFINITY>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::super::Foundation:: HANDLE, groupaffinity : *const super::SystemInformation:: GROUP_AFFINITY, previousgroupaffinity : *mut super::SystemInformation:: GROUP_AFFINITY) -> super::super::Foundation:: BOOL);
    SetThreadGroupAffinity(hthread.param().abi(), core::mem::transmute(groupaffinity), core::mem::transmute(previousgroupaffinity.unwrap_or(core::ptr::null_mut())))
}
#[inline]
pub unsafe fn SetThreadIdealProcessor<P0>(hthread: P0, dwidealprocessor: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadIdealProcessor(hthread : super::super::Foundation:: HANDLE, dwidealprocessor : u32) -> u32);
    SetThreadIdealProcessor(hthread.param().abi(), core::mem::transmute(dwidealprocessor))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn SetThreadIdealProcessorEx<P0>(hthread: P0, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: Option<*mut super::Kernel::PROCESSOR_NUMBER>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadIdealProcessorEx(hthread : super::super::Foundation:: HANDLE, lpidealprocessor : *const super::Kernel:: PROCESSOR_NUMBER, lppreviousidealprocessor : *mut super::Kernel:: PROCESSOR_NUMBER) -> super::super::Foundation:: BOOL);
    SetThreadIdealProcessorEx(hthread.param().abi(), core::mem::transmute(lpidealprocessor), core::mem::transmute(lppreviousidealprocessor.unwrap_or(core::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn SetThreadInformation<P0>(hthread: P0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const core::ffi::c_void, threadinformationsize: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadInformation(hthread : super::super::Foundation:: HANDLE, threadinformationclass : THREAD_INFORMATION_CLASS, threadinformation : *const core::ffi::c_void, threadinformationsize : u32) -> super::super::Foundation:: BOOL);
    SetThreadInformation(hthread.param().abi(), core::mem::transmute(threadinformationclass), core::mem::transmute(threadinformation), core::mem::transmute(threadinformationsize)).ok()
}
#[inline]
pub unsafe fn SetThreadPriority<P0>(hthread: P0, npriority: THREAD_PRIORITY) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadPriority(hthread : super::super::Foundation:: HANDLE, npriority : THREAD_PRIORITY) -> super::super::Foundation:: BOOL);
    SetThreadPriority(hthread.param().abi(), core::mem::transmute(npriority)).ok()
}
#[inline]
pub unsafe fn SetThreadPriorityBoost<P0, P1>(hthread: P0, bdisablepriorityboost: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadPriorityBoost(hthread : super::super::Foundation:: HANDLE, bdisablepriorityboost : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetThreadPriorityBoost(hthread.param().abi(), bdisablepriorityboost.param().abi()).ok()
}
#[cfg(feature = "Win32_System_SystemInformation")]
#[inline]
pub unsafe fn SetThreadSelectedCpuSetMasks<P0>(thread: P0, cpusetmasks: Option<&[super::SystemInformation::GROUP_AFFINITY]>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSetMasks(thread : super::super::Foundation:: HANDLE, cpusetmasks : *const super::SystemInformation:: GROUP_AFFINITY, cpusetmaskcount : u16) -> super::super::Foundation:: BOOL);
    SetThreadSelectedCpuSetMasks(thread.param().abi(), core::mem::transmute(cpusetmasks.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), cpusetmasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn SetThreadSelectedCpuSets<P0>(thread: P0, cpusetids: &[u32]) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadSelectedCpuSets(thread : super::super::Foundation:: HANDLE, cpusetids : *const u32, cpusetidcount : u32) -> super::super::Foundation:: BOOL);
    SetThreadSelectedCpuSets(thread.param().abi(), core::mem::transmute(cpusetids.as_ptr()), cpusetids.len().try_into().unwrap())
}
#[inline]
pub unsafe fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetThreadStackGuarantee(stacksizeinbytes : *mut u32) -> super::super::Foundation:: BOOL);
    SetThreadStackGuarantee(core::mem::transmute(stacksizeinbytes)).ok()
}
#[inline]
pub unsafe fn SetThreadToken<P1>(thread: Option<*const super::super::Foundation::HANDLE>, token: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("advapi32.dll" "system" fn SetThreadToken(thread : *const super::super::Foundation:: HANDLE, token : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    SetThreadToken(core::mem::transmute(thread.unwrap_or(core::ptr::null())), token.param().abi()).ok()
}
#[inline]
pub unsafe fn SetThreadpoolStackInformation<P0>(ptpp: P0, ptpsi: *const TP_POOL_STACK_INFORMATION) -> windows_core::Result<()>
where
    P0: windows_core::Param<PTP_POOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolStackInformation(ptpp : PTP_POOL, ptpsi : *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation:: BOOL);
    SetThreadpoolStackInformation(ptpp.param().abi(), core::mem::transmute(ptpsi)).ok()
}
#[inline]
pub unsafe fn SetThreadpoolThreadMaximum<P0>(ptpp: P0, cthrdmost: u32)
where
    P0: windows_core::Param<PTP_POOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolThreadMaximum(ptpp : PTP_POOL, cthrdmost : u32));
    SetThreadpoolThreadMaximum(ptpp.param().abi(), core::mem::transmute(cthrdmost))
}
#[inline]
pub unsafe fn SetThreadpoolThreadMinimum<P0>(ptpp: P0, cthrdmic: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<PTP_POOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolThreadMinimum(ptpp : PTP_POOL, cthrdmic : u32) -> super::super::Foundation:: BOOL);
    SetThreadpoolThreadMinimum(ptpp.param().abi(), core::mem::transmute(cthrdmic)).ok()
}
#[inline]
pub unsafe fn SetThreadpoolTimer<P0>(pti: P0, pftduetime: Option<*const super::super::Foundation::FILETIME>, msperiod: u32, mswindowlength: u32)
where
    P0: windows_core::Param<PTP_TIMER>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolTimer(pti : PTP_TIMER, pftduetime : *const super::super::Foundation:: FILETIME, msperiod : u32, mswindowlength : u32));
    SetThreadpoolTimer(pti.param().abi(), core::mem::transmute(pftduetime.unwrap_or(core::ptr::null())), core::mem::transmute(msperiod), core::mem::transmute(mswindowlength))
}
#[inline]
pub unsafe fn SetThreadpoolTimerEx<P0>(pti: P0, pftduetime: Option<*const super::super::Foundation::FILETIME>, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<PTP_TIMER>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolTimerEx(pti : PTP_TIMER, pftduetime : *const super::super::Foundation:: FILETIME, msperiod : u32, mswindowlength : u32) -> super::super::Foundation:: BOOL);
    SetThreadpoolTimerEx(pti.param().abi(), core::mem::transmute(pftduetime.unwrap_or(core::ptr::null())), core::mem::transmute(msperiod), core::mem::transmute(mswindowlength))
}
#[inline]
pub unsafe fn SetThreadpoolWait<P0, P1>(pwa: P0, h: P1, pfttimeout: Option<*const super::super::Foundation::FILETIME>)
where
    P0: windows_core::Param<PTP_WAIT>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolWait(pwa : PTP_WAIT, h : super::super::Foundation:: HANDLE, pfttimeout : *const super::super::Foundation:: FILETIME));
    SetThreadpoolWait(pwa.param().abi(), h.param().abi(), core::mem::transmute(pfttimeout.unwrap_or(core::ptr::null())))
}
#[inline]
pub unsafe fn SetThreadpoolWaitEx<P0, P1>(pwa: P0, h: P1, pfttimeout: Option<*const super::super::Foundation::FILETIME>, reserved: Option<*const core::ffi::c_void>) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<PTP_WAIT>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetThreadpoolWaitEx(pwa : PTP_WAIT, h : super::super::Foundation:: HANDLE, pfttimeout : *const super::super::Foundation:: FILETIME, reserved : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    SetThreadpoolWaitEx(pwa.param().abi(), h.param().abi(), core::mem::transmute(pfttimeout.unwrap_or(core::ptr::null())), core::mem::transmute(reserved.unwrap_or(core::ptr::null())))
}
#[inline]
pub unsafe fn SetTimerQueueTimer<P0, P5>(timerqueue: P0, callback: WAITORTIMERCALLBACK, parameter: Option<*const core::ffi::c_void>, duetime: u32, period: u32, preferio: P5) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P5: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetTimerQueueTimer(timerqueue : super::super::Foundation:: HANDLE, callback : WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, preferio : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
    SetTimerQueueTimer(timerqueue.param().abi(), core::mem::transmute(callback), core::mem::transmute(parameter.unwrap_or(core::ptr::null())), core::mem::transmute(duetime), core::mem::transmute(period), preferio.param().abi())
}
#[inline]
pub unsafe fn SetUmsThreadInformation(umsthread: *const core::ffi::c_void, umsthreadinfoclass: UMS_THREAD_INFO_CLASS, umsthreadinformation: *const core::ffi::c_void, umsthreadinformationlength: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetUmsThreadInformation(umsthread : *const core::ffi::c_void, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *const core::ffi::c_void, umsthreadinformationlength : u32) -> super::super::Foundation:: BOOL);
    SetUmsThreadInformation(core::mem::transmute(umsthread), core::mem::transmute(umsthreadinfoclass), core::mem::transmute(umsthreadinformation), core::mem::transmute(umsthreadinformationlength)).ok()
}
#[inline]
pub unsafe fn SetWaitableTimer<P0, P5>(htimer: P0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<*const core::ffi::c_void>, fresume: P5) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P5: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetWaitableTimer(htimer : super::super::Foundation:: HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, fresume : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    SetWaitableTimer(htimer.param().abi(), core::mem::transmute(lpduetime), core::mem::transmute(lperiod), core::mem::transmute(pfncompletionroutine), core::mem::transmute(lpargtocompletionroutine.unwrap_or(core::ptr::null())), fresume.param().abi()).ok()
}
#[inline]
pub unsafe fn SetWaitableTimerEx<P0>(htimer: P0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: Option<*const core::ffi::c_void>, wakecontext: Option<*const REASON_CONTEXT>, tolerabledelay: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetWaitableTimerEx(htimer : super::super::Foundation:: HANDLE, lpduetime : *const i64, lperiod : i32, pfncompletionroutine : PTIMERAPCROUTINE, lpargtocompletionroutine : *const core::ffi::c_void, wakecontext : *const REASON_CONTEXT, tolerabledelay : u32) -> super::super::Foundation:: BOOL);
    SetWaitableTimerEx(htimer.param().abi(), core::mem::transmute(lpduetime), core::mem::transmute(lperiod), core::mem::transmute(pfncompletionroutine), core::mem::transmute(lpargtocompletionroutine.unwrap_or(core::ptr::null())), core::mem::transmute(wakecontext.unwrap_or(core::ptr::null())), core::mem::transmute(tolerabledelay)).ok()
}
#[inline]
pub unsafe fn SignalObjectAndWait<P0, P1, P3>(hobjecttosignal: P0, hobjecttowaiton: P1, dwmilliseconds: u32, balertable: P3) -> super::super::Foundation::WAIT_EVENT
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SignalObjectAndWait(hobjecttosignal : super::super::Foundation:: HANDLE, hobjecttowaiton : super::super::Foundation:: HANDLE, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
    SignalObjectAndWait(hobjecttosignal.param().abi(), hobjecttowaiton.param().abi(), core::mem::transmute(dwmilliseconds), balertable.param().abi())
}
#[inline]
pub unsafe fn Sleep(dwmilliseconds: u32) {
    windows_targets::link!("kernel32.dll" "system" fn Sleep(dwmilliseconds : u32));
    Sleep(core::mem::transmute(dwmilliseconds))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn SleepConditionVariableCS(conditionvariable: *mut CONDITION_VARIABLE, criticalsection: *mut CRITICAL_SECTION, dwmilliseconds: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SleepConditionVariableCS(conditionvariable : *mut CONDITION_VARIABLE, criticalsection : *mut CRITICAL_SECTION, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
    SleepConditionVariableCS(core::mem::transmute(conditionvariable), core::mem::transmute(criticalsection), core::mem::transmute(dwmilliseconds)).ok()
}
#[inline]
pub unsafe fn SleepConditionVariableSRW(conditionvariable: *mut CONDITION_VARIABLE, srwlock: *mut SRWLOCK, dwmilliseconds: u32, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SleepConditionVariableSRW(conditionvariable : *mut CONDITION_VARIABLE, srwlock : *mut SRWLOCK, dwmilliseconds : u32, flags : u32) -> super::super::Foundation:: BOOL);
    SleepConditionVariableSRW(core::mem::transmute(conditionvariable), core::mem::transmute(srwlock), core::mem::transmute(dwmilliseconds), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn SleepEx<P1>(dwmilliseconds: u32, balertable: P1) -> u32
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn SleepEx(dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> u32);
    SleepEx(core::mem::transmute(dwmilliseconds), balertable.param().abi())
}
#[inline]
pub unsafe fn StartThreadpoolIo<P0>(pio: P0)
where
    P0: windows_core::Param<PTP_IO>,
{
    windows_targets::link!("kernel32.dll" "system" fn StartThreadpoolIo(pio : PTP_IO));
    StartThreadpoolIo(pio.param().abi())
}
#[inline]
pub unsafe fn SubmitThreadpoolWork<P0>(pwk: P0)
where
    P0: windows_core::Param<PTP_WORK>,
{
    windows_targets::link!("kernel32.dll" "system" fn SubmitThreadpoolWork(pwk : PTP_WORK));
    SubmitThreadpoolWork(pwk.param().abi())
}
#[inline]
pub unsafe fn SuspendThread<P0>(hthread: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SuspendThread(hthread : super::super::Foundation:: HANDLE) -> u32);
    SuspendThread(hthread.param().abi())
}
#[inline]
pub unsafe fn SwitchToFiber(lpfiber: *const core::ffi::c_void) {
    windows_targets::link!("kernel32.dll" "system" fn SwitchToFiber(lpfiber : *const core::ffi::c_void));
    SwitchToFiber(core::mem::transmute(lpfiber))
}
#[inline]
pub unsafe fn SwitchToThread() -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn SwitchToThread() -> super::super::Foundation:: BOOL);
    SwitchToThread()
}
#[inline]
pub unsafe fn TerminateProcess<P0>(hprocess: P0, uexitcode: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn TerminateProcess(hprocess : super::super::Foundation:: HANDLE, uexitcode : u32) -> super::super::Foundation:: BOOL);
    TerminateProcess(hprocess.param().abi(), core::mem::transmute(uexitcode)).ok()
}
#[inline]
pub unsafe fn TerminateThread<P0>(hthread: P0, dwexitcode: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn TerminateThread(hthread : super::super::Foundation:: HANDLE, dwexitcode : u32) -> super::super::Foundation:: BOOL);
    TerminateThread(hthread.param().abi(), core::mem::transmute(dwexitcode)).ok()
}
#[inline]
pub unsafe fn TlsAlloc() -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn TlsAlloc() -> u32);
    TlsAlloc()
}
#[inline]
pub unsafe fn TlsFree(dwtlsindex: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TlsFree(dwtlsindex : u32) -> super::super::Foundation:: BOOL);
    TlsFree(core::mem::transmute(dwtlsindex)).ok()
}
#[inline]
pub unsafe fn TlsGetValue(dwtlsindex: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn TlsGetValue(dwtlsindex : u32) -> *mut core::ffi::c_void);
    TlsGetValue(core::mem::transmute(dwtlsindex))
}
#[inline]
pub unsafe fn TlsSetValue(dwtlsindex: u32, lptlsvalue: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TlsSetValue(dwtlsindex : u32, lptlsvalue : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    TlsSetValue(core::mem::transmute(dwtlsindex), core::mem::transmute(lptlsvalue.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn TryAcquireSRWLockExclusive(srwlock: *mut SRWLOCK) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("kernel32.dll" "system" fn TryAcquireSRWLockExclusive(srwlock : *mut SRWLOCK) -> super::super::Foundation:: BOOLEAN);
    TryAcquireSRWLockExclusive(core::mem::transmute(srwlock))
}
#[inline]
pub unsafe fn TryAcquireSRWLockShared(srwlock: *mut SRWLOCK) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("kernel32.dll" "system" fn TryAcquireSRWLockShared(srwlock : *mut SRWLOCK) -> super::super::Foundation:: BOOLEAN);
    TryAcquireSRWLockShared(core::mem::transmute(srwlock))
}
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn TryEnterCriticalSection(lpcriticalsection: *mut CRITICAL_SECTION) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn TryEnterCriticalSection(lpcriticalsection : *mut CRITICAL_SECTION) -> super::super::Foundation:: BOOL);
    TryEnterCriticalSection(core::mem::transmute(lpcriticalsection))
}
#[inline]
pub unsafe fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: Option<*mut core::ffi::c_void>, pcbe: Option<*const TP_CALLBACK_ENVIRON_V3>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TrySubmitThreadpoolCallback(pfns : PTP_SIMPLE_CALLBACK, pv : *mut core::ffi::c_void, pcbe : *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation:: BOOL);
    TrySubmitThreadpoolCallback(core::mem::transmute(pfns), core::mem::transmute(pv.unwrap_or(core::ptr::null_mut())), core::mem::transmute(pcbe.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn UmsThreadYield(schedulerparam: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UmsThreadYield(schedulerparam : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    UmsThreadYield(core::mem::transmute(schedulerparam)).ok()
}
#[inline]
pub unsafe fn UnregisterWait<P0>(waithandle: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn UnregisterWait(waithandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    UnregisterWait(waithandle.param().abi()).ok()
}
#[inline]
pub unsafe fn UnregisterWaitEx<P0, P1>(waithandle: P0, completionevent: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn UnregisterWaitEx(waithandle : super::super::Foundation:: HANDLE, completionevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    UnregisterWaitEx(waithandle.param().abi(), completionevent.param().abi()).ok()
}
#[inline]
pub unsafe fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: Option<*const core::ffi::c_void>, cbsize: usize, lppreviousvalue: Option<*mut core::ffi::c_void>, lpreturnsize: Option<*const usize>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UpdateProcThreadAttribute(lpattributelist : LPPROC_THREAD_ATTRIBUTE_LIST, dwflags : u32, attribute : usize, lpvalue : *const core::ffi::c_void, cbsize : usize, lppreviousvalue : *mut core::ffi::c_void, lpreturnsize : *const usize) -> super::super::Foundation:: BOOL);
    UpdateProcThreadAttribute(core::mem::transmute(lpattributelist), core::mem::transmute(dwflags), core::mem::transmute(attribute), core::mem::transmute(lpvalue.unwrap_or(core::ptr::null())), core::mem::transmute(cbsize), core::mem::transmute(lppreviousvalue.unwrap_or(core::ptr::null_mut())), core::mem::transmute(lpreturnsize.unwrap_or(core::ptr::null()))).ok()
}
#[inline]
pub unsafe fn WaitForInputIdle<P0>(hprocess: P0, dwmilliseconds: u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("user32.dll" "system" fn WaitForInputIdle(hprocess : super::super::Foundation:: HANDLE, dwmilliseconds : u32) -> u32);
    WaitForInputIdle(hprocess.param().abi(), core::mem::transmute(dwmilliseconds))
}
#[inline]
pub unsafe fn WaitForMultipleObjects<P2>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: P2, dwmilliseconds: u32) -> super::super::Foundation::WAIT_EVENT
where
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForMultipleObjects(ncount : u32, lphandles : *const super::super::Foundation:: HANDLE, bwaitall : super::super::Foundation:: BOOL, dwmilliseconds : u32) -> super::super::Foundation:: WAIT_EVENT);
    WaitForMultipleObjects(lphandles.len().try_into().unwrap(), core::mem::transmute(lphandles.as_ptr()), bwaitall.param().abi(), core::mem::transmute(dwmilliseconds))
}
#[inline]
pub unsafe fn WaitForMultipleObjectsEx<P2, P4>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: P2, dwmilliseconds: u32, balertable: P4) -> super::super::Foundation::WAIT_EVENT
where
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P4: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForMultipleObjectsEx(ncount : u32, lphandles : *const super::super::Foundation:: HANDLE, bwaitall : super::super::Foundation:: BOOL, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
    WaitForMultipleObjectsEx(lphandles.len().try_into().unwrap(), core::mem::transmute(lphandles.as_ptr()), bwaitall.param().abi(), core::mem::transmute(dwmilliseconds), balertable.param().abi())
}
#[inline]
pub unsafe fn WaitForSingleObject<P0>(hhandle: P0, dwmilliseconds: u32) -> super::super::Foundation::WAIT_EVENT
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : super::super::Foundation:: HANDLE, dwmilliseconds : u32) -> super::super::Foundation:: WAIT_EVENT);
    WaitForSingleObject(hhandle.param().abi(), core::mem::transmute(dwmilliseconds))
}
#[inline]
pub unsafe fn WaitForSingleObjectEx<P0, P2>(hhandle: P0, dwmilliseconds: u32, balertable: P2) -> super::super::Foundation::WAIT_EVENT
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObjectEx(hhandle : super::super::Foundation:: HANDLE, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: WAIT_EVENT);
    WaitForSingleObjectEx(hhandle.param().abi(), core::mem::transmute(dwmilliseconds), balertable.param().abi())
}
#[inline]
pub unsafe fn WaitForThreadpoolIoCallbacks<P0, P1>(pio: P0, fcancelpendingcallbacks: P1)
where
    P0: windows_core::Param<PTP_IO>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolIoCallbacks(pio : PTP_IO, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
    WaitForThreadpoolIoCallbacks(pio.param().abi(), fcancelpendingcallbacks.param().abi())
}
#[inline]
pub unsafe fn WaitForThreadpoolTimerCallbacks<P0, P1>(pti: P0, fcancelpendingcallbacks: P1)
where
    P0: windows_core::Param<PTP_TIMER>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolTimerCallbacks(pti : PTP_TIMER, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
    WaitForThreadpoolTimerCallbacks(pti.param().abi(), fcancelpendingcallbacks.param().abi())
}
#[inline]
pub unsafe fn WaitForThreadpoolWaitCallbacks<P0, P1>(pwa: P0, fcancelpendingcallbacks: P1)
where
    P0: windows_core::Param<PTP_WAIT>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolWaitCallbacks(pwa : PTP_WAIT, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
    WaitForThreadpoolWaitCallbacks(pwa.param().abi(), fcancelpendingcallbacks.param().abi())
}
#[inline]
pub unsafe fn WaitForThreadpoolWorkCallbacks<P0, P1>(pwk: P0, fcancelpendingcallbacks: P1)
where
    P0: windows_core::Param<PTP_WORK>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitForThreadpoolWorkCallbacks(pwk : PTP_WORK, fcancelpendingcallbacks : super::super::Foundation:: BOOL));
    WaitForThreadpoolWorkCallbacks(pwk.param().abi(), fcancelpendingcallbacks.param().abi())
}
#[inline]
pub unsafe fn WaitOnAddress(address: *const core::ffi::c_void, compareaddress: *const core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WaitOnAddress(address : *const core::ffi::c_void, compareaddress : *const core::ffi::c_void, addresssize : usize, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
    WaitOnAddress(core::mem::transmute(address), core::mem::transmute(compareaddress), core::mem::transmute(addresssize), core::mem::transmute(dwmilliseconds)).ok()
}
#[inline]
pub unsafe fn WakeAllConditionVariable(conditionvariable: *mut CONDITION_VARIABLE) {
    windows_targets::link!("kernel32.dll" "system" fn WakeAllConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
    WakeAllConditionVariable(core::mem::transmute(conditionvariable))
}
#[inline]
pub unsafe fn WakeByAddressAll(address: *const core::ffi::c_void) {
    windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressAll(address : *const core::ffi::c_void));
    WakeByAddressAll(core::mem::transmute(address))
}
#[inline]
pub unsafe fn WakeByAddressSingle(address: *const core::ffi::c_void) {
    windows_targets::link!("api-ms-win-core-synch-l1-2-0.dll" "system" fn WakeByAddressSingle(address : *const core::ffi::c_void));
    WakeByAddressSingle(core::mem::transmute(address))
}
#[inline]
pub unsafe fn WakeConditionVariable(conditionvariable: *mut CONDITION_VARIABLE) {
    windows_targets::link!("kernel32.dll" "system" fn WakeConditionVariable(conditionvariable : *mut CONDITION_VARIABLE));
    WakeConditionVariable(core::mem::transmute(conditionvariable))
}
#[inline]
pub unsafe fn WinExec<P0>(lpcmdline: P0, ucmdshow: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WinExec(lpcmdline : windows_core::PCSTR, ucmdshow : u32) -> u32);
    WinExec(lpcmdline.param().abi(), core::mem::transmute(ucmdshow))
}
#[inline]
pub unsafe fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16 {
    windows_targets::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn Wow64SetThreadDefaultGuestMachine(machine : u16) -> u16);
    Wow64SetThreadDefaultGuestMachine(core::mem::transmute(machine))
}
#[inline]
pub unsafe fn Wow64SuspendThread<P0>(hthread: P0) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn Wow64SuspendThread(hthread : super::super::Foundation:: HANDLE) -> u32);
    Wow64SuspendThread(hthread.param().abi())
}
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32768u32);
pub const ALL_PROCESSOR_GROUPS: u16 = 65535u16;
pub type APC_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(param0: u32, param1: *mut core::ffi::c_void, param2: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl Default for APP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for APP_MEMORY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AVRT_PRIORITY(pub i32);
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = AVRT_PRIORITY(2i32);
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = AVRT_PRIORITY(1i32);
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = AVRT_PRIORITY(-1i32);
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = AVRT_PRIORITY(0i32);
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = AVRT_PRIORITY(-2i32);
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16384u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONDITION_VARIABLE {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CONDITION_VARIABLE {
    type TypeKind = windows_core::CopyType;
}
pub const CONDITION_VARIABLE_INIT: CONDITION_VARIABLE = CONDITION_VARIABLE { Ptr: core::ptr::null_mut() };
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16777216u32);
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(67108864u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_EVENT(pub u32);
impl CREATE_EVENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = CREATE_EVENT(2u32);
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = CREATE_EVENT(1u32);
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8192u32);
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2147483648u32);
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16u32);
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(512u32);
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(134217728u32);
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(33554432u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CREATE_PROCESS_LOGON_FLAGS(pub u32);
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(262144u32);
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4194304u32);
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2048u32);
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4096u32);
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4u32);
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1024u32);
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRITICAL_SECTION {
    pub DebugInfo: *mut CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for CRITICAL_SECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub Identifier: u16,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for CRITICAL_SECTION_DEBUG {
    type TypeKind = windows_core::CopyType;
}
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2u32);
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1u32);
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8u32);
pub const EVENT_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
pub const EVENT_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(524288u32);
pub const FLS_OUT_OF_INDEXES: u32 = 4294967295u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GET_GUI_RESOURCES_FLAGS(pub u32);
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(0u32);
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(2u32);
pub const GR_GLOBAL: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4294967294u32);
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(1u32);
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4u32);
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(128u32);
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(64u32);
pub const INFINITE: u32 = 4294967295u32;
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(131072u32);
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(65536u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub union INIT_ONCE {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for INIT_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INIT_ONCE {
    type TypeKind = windows_core::CopyType;
}
pub const INIT_ONCE_ASYNC: u32 = 2u32;
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
pub const INIT_ONCE_STATIC_INIT: INIT_ONCE = INIT_ONCE { Ptr: core::ptr::null_mut() };
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl Default for IO_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IO_COUNTERS {
    type TypeKind = windows_core::CopyType;
}
windows_core::imp::define_interface!(IRtwqAsyncCallback, IRtwqAsyncCallback_Vtbl, 0xa27003cf_2354_4f2a_8d6a_ab7cff15437e);
windows_core::imp::interface_hierarchy!(IRtwqAsyncCallback, windows_core::IUnknown);
impl IRtwqAsyncCallback {
    pub unsafe fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), core::mem::transmute(pdwflags), core::mem::transmute(pdwqueue)).ok()
    }
    pub unsafe fn Invoke<P0>(&self, pasyncresult: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IRtwqAsyncResult>,
    {
        (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), pasyncresult.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IRtwqAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRtwqAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn GetParameters(&self, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::Result<()>;
    fn Invoke(&self, pasyncresult: Option<&IRtwqAsyncResult>) -> windows_core::Result<()>;
}
impl IRtwqAsyncCallback_Vtbl {
    pub const fn new<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParameters<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut u32, pdwqueue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqAsyncCallback_Impl::GetParameters(this, core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pdwqueue)).into()
        }
        unsafe extern "system" fn Invoke<Identity: IRtwqAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqAsyncCallback_Impl::Invoke(this, windows_core::from_raw_borrowed(&pasyncresult)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParameters: GetParameters::<Identity, OFFSET>,
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqAsyncCallback {}
windows_core::imp::define_interface!(IRtwqAsyncResult, IRtwqAsyncResult_Vtbl, 0xac6b7889_0740_4d51_8619_905994a55cc6);
windows_core::imp::interface_hierarchy!(IRtwqAsyncResult, windows_core::IUnknown);
impl IRtwqAsyncResult {
    pub unsafe fn GetState(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(hrstatus)).ok()
    }
    pub unsafe fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown> {
        (windows_core::Interface::vtable(self).GetStateNoAddRef)(windows_core::Interface::as_raw(self))
    }
}
#[repr(C)]
pub struct IRtwqAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStateNoAddRef: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<windows_core::IUnknown>,
}
pub trait IRtwqAsyncResult_Impl: windows_core::IUnknownImpl {
    fn GetState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStatus(&self) -> windows_core::Result<()>;
    fn SetStatus(&self, hrstatus: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetStateNoAddRef(&self) -> Option<windows_core::IUnknown>;
}
impl IRtwqAsyncResult_Vtbl {
    pub const fn new<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRtwqAsyncResult_Impl::GetState(this) {
                Ok(ok__) => {
                    ppunkstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqAsyncResult_Impl::GetStatus(this).into()
        }
        unsafe extern "system" fn SetStatus<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqAsyncResult_Impl::SetStatus(this, core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetObject<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRtwqAsyncResult_Impl::GetObject(this) {
                Ok(ok__) => {
                    ppobject.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStateNoAddRef<Identity: IRtwqAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<windows_core::IUnknown> {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqAsyncResult_Impl::GetStateNoAddRef(this)
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqAsyncResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqAsyncResult {}
windows_core::imp::define_interface!(IRtwqPlatformEvents, IRtwqPlatformEvents_Vtbl, 0x63d9255a_7ff1_4b61_8faf_ed6460dacf2b);
windows_core::imp::interface_hierarchy!(IRtwqPlatformEvents, windows_core::IUnknown);
impl IRtwqPlatformEvents {
    pub unsafe fn InitializationComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).InitializationComplete)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownStart(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownStart)(windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ShutdownComplete(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ShutdownComplete)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IRtwqPlatformEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializationComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownStart: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShutdownComplete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRtwqPlatformEvents_Impl: windows_core::IUnknownImpl {
    fn InitializationComplete(&self) -> windows_core::Result<()>;
    fn ShutdownStart(&self) -> windows_core::Result<()>;
    fn ShutdownComplete(&self) -> windows_core::Result<()>;
}
impl IRtwqPlatformEvents_Vtbl {
    pub const fn new<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializationComplete<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqPlatformEvents_Impl::InitializationComplete(this).into()
        }
        unsafe extern "system" fn ShutdownStart<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqPlatformEvents_Impl::ShutdownStart(this).into()
        }
        unsafe extern "system" fn ShutdownComplete<Identity: IRtwqPlatformEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRtwqPlatformEvents_Impl::ShutdownComplete(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializationComplete: InitializationComplete::<Identity, OFFSET>,
            ShutdownStart: ShutdownStart::<Identity, OFFSET>,
            ShutdownComplete: ShutdownComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRtwqPlatformEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRtwqPlatformEvents {}
pub const KernelEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(2i32);
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(2u32);
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(1u32);
pub type LPFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpfiberparameter: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub *mut core::ffi::c_void);
impl windows_core::TypeKind for LPPROC_THREAD_ATTRIBUTE_LIST {
    type TypeKind = windows_core::CopyType;
}
impl LPPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl windows_core::Free for LPPROC_THREAD_ATTRIBUTE_LIST {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn DeleteProcThreadAttributeList(lpattributelist : *mut core::ffi::c_void));
            DeleteProcThreadAttributeList(self.0);
        }
    }
}
impl Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPTHREAD_START_ROUTINE = Option<unsafe extern "system" fn(lpthreadparameter: *mut core::ffi::c_void) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MACHINE_ATTRIBUTES(pub i32);
impl MACHINE_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_PRIORITY(pub u32);
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(4u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl Default for MEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MEMORY_PRIORITY_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(2u32);
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = MEMORY_PRIORITY(3u32);
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(5u32);
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(1u32);
pub const MUTEX_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031617u32);
pub const MUTEX_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1u32);
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(20i32);
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OVERRIDE_PREFETCH_PARAMETER {
    pub Value: u32,
}
impl Default for OVERRIDE_PREFETCH_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OVERRIDE_PREFETCH_PARAMETER {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut core::ffi::c_void,
    pub Reserved5: *mut core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for PEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for PEB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for PEB_LDR_DATA {
    type TypeKind = windows_core::CopyType;
}
pub type PFLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(lpflsdata: *const core::ffi::c_void)>;
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(7u32);
pub const PF_ALPHA_BYTE_INSTRUCTIONS: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(5u32);
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(25u32);
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(24u32);
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(26u32);
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(27u32);
pub const PF_ARM_NEON_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(19u32);
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(34u32);
pub const PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(43u32);
pub const PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(44u32);
pub const PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(45u32);
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(31u32);
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(30u32);
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(29u32);
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(18u32);
pub const PF_AVX2_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(40u32);
pub const PF_AVX512F_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(41u32);
pub const PF_AVX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(39u32);
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(16u32);
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(15u32);
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(14u32);
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(2u32);
pub const PF_ERMS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(42u32);
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(23u32);
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(1u32);
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(0u32);
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(3u32);
pub const PF_MONITORX_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(35u32);
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(12u32);
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(9u32);
pub const PF_PPC_MOVEMEM_64BIT_OK: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(4u32);
pub const PF_RDPID_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(33u32);
pub const PF_RDRAND_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(28u32);
pub const PF_RDTSCP_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(32u32);
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(8u32);
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(22u32);
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(20u32);
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(13u32);
pub const PF_SSE4_1_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(37u32);
pub const PF_SSE4_2_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(38u32);
pub const PF_SSE_DAZ_MODE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(11u32);
pub const PF_SSSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(36u32);
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(21u32);
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(10u32);
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(6u32);
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(17u32);
pub type PINIT_ONCE_FN = Option<unsafe extern "system" fn(initonce: *mut INIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(0i32);
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(1i32);
pub const PME_CURRENT_VERSION: u32 = 1u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POWER_REQUEST_CONTEXT_FLAGS(pub u32);
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(1u32);
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESSOR_FEATURE_ID(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_ACCESS_RIGHTS(pub u32);
impl PROCESS_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(pub u32);
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(0u32);
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(1u32);
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2097151u32);
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_BASIC_INFORMATION {
    pub ExitStatus: super::super::Foundation::NTSTATUS,
    pub PebBaseAddress: *mut PEB,
    pub AffinityMask: usize,
    pub BasePriority: i32,
    pub UniqueProcessId: usize,
    pub InheritedFromUniqueProcessId: usize,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for PROCESS_BASIC_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(128u32);
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_CREATION_FLAGS(pub u32);
impl PROCESS_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(65536u32);
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(2u32);
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(1u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_DEP_FLAGS(pub u32);
impl PROCESS_DEP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(0u32);
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(64u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
impl Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_INFORMATION_CLASS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl Default for PROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_LEAP_SECOND_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_System_SystemInformation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: super::SystemInformation::IMAGE_FILE_MACHINE,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl Default for PROCESS_MACHINE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl windows_core::TypeKind for PROCESS_MACHINE_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl Default for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_MEMORY_EXHAUSTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MITIGATION_POLICY(pub i32);
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1048576u32);
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2097152u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_NAME_FORMAT(pub u32);
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(1u32);
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(0u32);
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl Default for PROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_POWER_THROTTLING_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_PROTECTION_LEVEL(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl Default for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PROCESS_PROTECTION_LEVEL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1024u32);
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4096u32);
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(131072u32);
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(512u32);
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8192u32);
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(256u32);
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4u32);
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(983040u32);
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2048u32);
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1048576u32);
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1u32);
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8u32);
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(16u32);
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(32u32);
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(262144u32);
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(524288u32);
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROC_THREAD_ATTRIBUTE_NUM(pub u32);
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(536870912u32);
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1073741824u32);
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(268435456u32);
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(3u32);
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(7u32);
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(6u32);
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4u32);
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4294967294u32);
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(8u32);
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(1u32);
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(2u32);
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(5u32);
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(0u32);
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const core::ffi::c_void)>;
pub type PTIMERAPCROUTINE = Option<unsafe extern "system" fn(lpargtocompletionroutine: *const core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_CALLBACK_INSTANCE(pub isize);
impl windows_core::TypeKind for PTP_CALLBACK_INSTANCE {
    type TypeKind = windows_core::CopyType;
}
impl PTP_CALLBACK_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_CLEANUP_GROUP(pub isize);
impl windows_core::TypeKind for PTP_CLEANUP_GROUP {
    type TypeKind = windows_core::CopyType;
}
impl PTP_CLEANUP_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_CLEANUP_GROUP {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolCleanupGroup(ptpcg : isize));
            CloseThreadpoolCleanupGroup(self.0);
        }
    }
}
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, cleanupcontext: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_IO(pub isize);
impl windows_core::TypeKind for PTP_IO {
    type TypeKind = windows_core::CopyType;
}
impl PTP_IO {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_IO {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolIo(pio : isize));
            CloseThreadpoolIo(self.0);
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_POOL(pub isize);
impl windows_core::TypeKind for PTP_POOL {
    type TypeKind = windows_core::CopyType;
}
impl PTP_POOL {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_POOL {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpool(ptpp : isize));
            CloseThreadpool(self.0);
        }
    }
}
pub type PTP_SIMPLE_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_TIMER(pub isize);
impl windows_core::TypeKind for PTP_TIMER {
    type TypeKind = windows_core::CopyType;
}
impl PTP_TIMER {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_TIMER {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolTimer(pti : isize));
            CloseThreadpoolTimer(self.0);
        }
    }
}
pub type PTP_TIMER_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, timer: PTP_TIMER)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_WAIT(pub isize);
impl windows_core::TypeKind for PTP_WAIT {
    type TypeKind = windows_core::CopyType;
}
impl PTP_WAIT {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_WAIT {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWait(pwa : isize));
            CloseThreadpoolWait(self.0);
        }
    }
}
pub type PTP_WAIT_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, wait: PTP_WAIT, waitresult: u32)>;
pub type PTP_WIN32_IO_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, overlapped: *mut core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: PTP_IO)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTP_WORK(pub isize);
impl windows_core::TypeKind for PTP_WORK {
    type TypeKind = windows_core::CopyType;
}
impl PTP_WORK {
    pub fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}
impl windows_core::Free for PTP_WORK {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("kernel32.dll" "system" fn CloseThreadpoolWork(pwk : isize));
            CloseThreadpoolWork(self.0);
        }
    }
}
pub type PTP_WORK_CALLBACK = Option<unsafe extern "system" fn(instance: PTP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, work: PTP_WORK)>;
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(15u32);
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(14u32);
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(26u32);
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(18u32);
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(27u32);
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(3u32);
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(2u32);
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(5u32);
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(13u32);
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(25u32);
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(24u32);
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(7u32);
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(0u32);
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(4u32);
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(11u32);
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(22u32);
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(17u32);
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(9u32);
pub const ProcThreadAttributeTrustedApp: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(29u32);
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(6u32);
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(16u32);
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(1i32);
pub const ProcessActivationContextTrustPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(19i32);
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(2i32);
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(13i32);
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(7i32);
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(0i32);
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(2i32);
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(6i32);
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(9i32);
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(10i32);
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(3i32);
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(12i32);
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(8i32);
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(9i32);
pub const ProcessMaxOverridePrefetchParameter: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(11i32);
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(1i32);
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(0i32);
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(5i32);
pub const ProcessOverrideSubsequentPrefetchParameter: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(10i32);
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(12i32);
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(4i32);
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(7i32);
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(16i32);
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(5i32);
pub const ProcessSEHOPPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(18i32);
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(14i32);
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(8i32);
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(3i32);
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(4i32);
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(11i32);
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(6i32);
pub const ProcessUserPointerAuthPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(17i32);
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(15i32);
pub const QUEUE_USER_APC_CALLBACK_DATA_CONTEXT: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(65536i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QUEUE_USER_APC_FLAGS(pub i32);
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(0i32);
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(1i32);
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(256u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
impl Default for REASON_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REASON_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: windows_core::PWSTR,
}
impl Default for REASON_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REASON_CONTEXT_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HMODULE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut windows_core::PWSTR,
}
impl Default for REASON_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for REASON_CONTEXT_0_0 {
    type TypeKind = windows_core::CopyType;
}
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: u32 = 4278190080u32;
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: u32 = 1u32;
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: u32 = 33554432u32;
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: u32 = 268435456u32;
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: u32 = 16777216u32;
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: u32 = 134217728u32;
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: u32 = 67108864u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RTL_USER_PROCESS_PARAMETERS {
    type TypeKind = windows_core::CopyType;
}
windows_core::imp::define_interface!(RTWQASYNCRESULT, RTWQASYNCRESULT_Vtbl, 0);
impl core::ops::Deref for RTWQASYNCRESULT {
    type Target = IRtwqAsyncResult;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(RTWQASYNCRESULT, windows_core::IUnknown, IRtwqAsyncResult);
#[repr(C)]
pub struct RTWQASYNCRESULT_Vtbl {
    pub base__: IRtwqAsyncResult_Vtbl,
}
pub trait RTWQASYNCRESULT_Impl: IRtwqAsyncResult_Impl {}
impl RTWQASYNCRESULT_Vtbl {
    pub const fn new<Identity: RTWQASYNCRESULT_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IRtwqAsyncResult_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<RTWQASYNCRESULT as windows_core::Interface>::IID || iid == &<IRtwqAsyncResult as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for RTWQASYNCRESULT {}
pub type RTWQPERIODICCALLBACK = Option<unsafe extern "system" fn(context: Option<windows_core::IUnknown>)>;
pub const RTWQ_MULTITHREADED_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = RTWQ_WORKQUEUE_TYPE(2i32);
pub const RTWQ_STANDARD_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = RTWQ_WORKQUEUE_TYPE(0i32);
pub const RTWQ_WINDOW_WORKQUEUE: RTWQ_WORKQUEUE_TYPE = RTWQ_WORKQUEUE_TYPE(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RTWQ_WORKQUEUE_TYPE(pub i32);
pub const SEMAPHORE_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
pub const SEMAPHORE_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SRWLOCK {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for SRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SRWLOCK {
    type TypeKind = windows_core::CopyType;
}
pub const SRWLOCK_INIT: SRWLOCK = SRWLOCK { Ptr: core::ptr::null_mut() };
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(65536u32);
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(128u32);
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(64u32);
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8192u32);
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32u32);
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4096u32);
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2048u32);
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32768u32);
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8u32);
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(16u32);
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(512u32);
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4u32);
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(1u32);
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2u32);
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(256u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: windows_core::PSTR,
    pub lpDesktop: windows_core::PSTR,
    pub lpTitle: windows_core::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
impl Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STARTUPINFOA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STARTUPINFOEXA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
impl Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STARTUPINFOEXW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: windows_core::PWSTR,
    pub lpDesktop: windows_core::PWSTR,
    pub lpTitle: windows_core::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
impl Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for STARTUPINFOW {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUPINFOW_FLAGS(pub u32);
impl STARTUPINFOW_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYNCHRONIZATION_ACCESS_RIGHTS(pub u32);
impl SYNCHRONIZATION_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYNCHRONIZATION_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl Default for SYNCHRONIZATION_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SYNCHRONIZATION_BARRIER {
    type TypeKind = windows_core::CopyType;
}
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
pub const SYNCHRONIZATION_DELETE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(65536u32);
pub const SYNCHRONIZATION_READ_CONTROL: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(131072u32);
pub const SYNCHRONIZATION_SYNCHRONIZE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1048576u32);
pub const SYNCHRONIZATION_WRITE_DAC: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(262144u32);
pub const SYNCHRONIZATION_WRITE_OWNER: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(524288u32);
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TEB {
    pub Reserved1: [*mut core::ffi::c_void; 12],
    pub ProcessEnvironmentBlock: *mut PEB,
    pub Reserved2: [*mut core::ffi::c_void; 399],
    pub Reserved3: [u8; 1952],
    pub TlsSlots: [*mut core::ffi::c_void; 64],
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 26],
    pub ReservedForOle: *mut core::ffi::c_void,
    pub Reserved6: [*mut core::ffi::c_void; 4],
    pub TlsExpansionSlots: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for TEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl windows_core::TypeKind for TEB {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREAD_ACCESS_RIGHTS(pub u32);
impl THREAD_ACCESS_RIGHTS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2097151u32);
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(0u32);
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREAD_CREATION_FLAGS(pub u32);
impl THREAD_CREATION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(65536u32);
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(512u32);
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(8u32);
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(256u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREAD_INFORMATION_CLASS(pub i32);
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = THREAD_PRIORITY(65536i32);
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = THREAD_PRIORITY(131072i32);
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl Default for THREAD_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for THREAD_POWER_THROTTLING_STATE {
    type TypeKind = windows_core::CopyType;
}
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREAD_PRIORITY(pub i32);
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(1i32);
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(-1i32);
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = THREAD_PRIORITY(2i32);
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = THREAD_PRIORITY(-15i32);
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(0i32);
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = THREAD_PRIORITY(15i32);
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(64u32);
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2048u32);
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(131072u32);
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(4096u32);
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(16u32);
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(32u32);
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1024u32);
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(128u32);
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(983040u32);
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2u32);
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1048576u32);
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1u32);
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(262144u32);
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(524288u32);
pub const TIMER_ALL_ACCESS: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2031619u32);
pub const TIMER_MODIFY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(2u32);
pub const TIMER_QUERY_STATE: SYNCHRONIZATION_ACCESS_RIGHTS = SYNCHRONIZATION_ACCESS_RIGHTS(1u32);
pub const TLS_OUT_OF_INDEXES: u32 = 4294967295u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TP_CALLBACK_ENVIRON_V3 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_0_0,
}
impl Default for TP_CALLBACK_ENVIRON_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TP_CALLBACK_ENVIRON_V3_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
impl Default for TP_CALLBACK_ENVIRON_V3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TP_CALLBACK_ENVIRON_V3_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TP_CALLBACK_PRIORITY(pub i32);
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(0i32);
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(2i32);
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl Default for TP_POOL_STACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for TP_POOL_STACK_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(1i32);
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(2i32);
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(4i32);
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(0i32);
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(3i32);
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl windows_core::TypeKind for UMS_SCHEDULER_STARTUP_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl Default for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMS_THREAD_INFO_CLASS(pub i32);
pub const UmsThreadAffinity: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(3i32);
pub const UmsThreadInvalidInfoClass: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(0i32);
pub const UmsThreadIsSuspended: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(5i32);
pub const UmsThreadIsTerminated: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(6i32);
pub const UmsThreadMaxInfoClass: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(7i32);
pub const UmsThreadPriority: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(2i32);
pub const UmsThreadTeb: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(4i32);
pub const UmsThreadUserContext: UMS_THREAD_INFO_CLASS = UMS_THREAD_INFO_CLASS(1i32);
pub const UserEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(1i32);
pub type WAITORTIMERCALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: super::super::Foundation::BOOLEAN)>;
pub type WORKERCALLBACKFUNC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WORKER_THREAD_FLAGS(pub u32);
impl WORKER_THREAD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(0u32);
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(1u32);
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(128u32);
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(32u32);
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(4u32);
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(16u32);
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(8u32);
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(256u32);
pub const Wow64Container: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(4i32);
