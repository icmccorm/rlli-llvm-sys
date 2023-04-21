use crate::execution_engine::LLVMGenericValueRef;
use crate::prelude::LLVMTypeRef;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MiriProvenance {
    pub alloc_id: ::std::os::raw::c_ulonglong,
    pub tag: ::std::os::raw::c_ulonglong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct MiriPointer {
    pub addr: ::std::os::raw::c_ulonglong,
    pub prov: MiriProvenance,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MiriErrorTrace {
    pub directory: *const ::std::os::raw::c_char,
    pub directory_len: u64,
    pub file: *const ::std::os::raw::c_char,
    pub file_len: u64,
    pub line: u32,
    pub column: u32,
}

pub type MiriInterpCxOpaque = ::std::os::raw::c_void;

pub type MiriMemset = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: MiriPointer,
        arg3: u8,
        arg4: u64,
    ) -> bool,
>;

pub type MiriMemcpy = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: MiriPointer,
        arg3: *const ::std::os::raw::c_uchar,
        arg4: u64,
    ) -> bool,
>;

pub type MiriAllocationHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: u64, arg3: u64) -> MiriPointer,
>;
pub type MiriFreeHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(arg1: *mut MiriInterpCxOpaque, arg2: MiriPointer) -> bool,
>;
pub type MiriLoadStoreHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        arg1: *mut MiriInterpCxOpaque,
        arg2: LLVMGenericValueRef,
        arg3: MiriPointer,
        arg4: LLVMTypeRef,
        arg5: u64,
        arg6: u64,
    ) -> bool,
>;
pub type MiriCallbackHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        ret_ref: LLVMGenericValueRef,
        args_ref: LLVMGenericValueRef,
        num_args: u64,
        name: *const ::libc::c_uchar,
        name_length: u64,
        tref: LLVMTypeRef,
    ) -> bool,
>;
pub type MiriStackTraceRecorderHook = ::std::option::Option<
    unsafe extern "C-unwind" fn(
        ctx_raw: *mut MiriInterpCxOpaque,
        traces: *const MiriErrorTrace,
        num_traces: u64,
    ),
>;
