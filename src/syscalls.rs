#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables)]

use core::ffi::c_void;

pub type PVOID = *mut c_void;
pub type HANDLE = *mut c_void;
pub type DWORD = u32;
pub type BOOLEAN = u32;
pub type NTSTATUS = u32;
pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG_PTR = usize;
pub type SIZE_T = usize;

pub type LONG = i32;
pub type VP_STATUS = LONG;
pub type PWSTR = *mut u16;
pub type LARGE_INTEGER = u64;

pub type ACCESS_MASK = DWORD;
pub mod AccessMask {
    use super::ACCESS_MASK;
    pub const DELETE                  : ACCESS_MASK = 0x00010000;
    pub const READ_CONTROL            : ACCESS_MASK = 0x00020000;
    pub const WRITE_DAC               : ACCESS_MASK = 0x00040000;
    pub const WRITE_OWNER             : ACCESS_MASK = 0x00080000;
    pub const SYNCHRONIZE             : ACCESS_MASK = 0x00100000;
    pub const STANDARD_RIGHTS_REQUIRED: ACCESS_MASK = 0x000F0000;
    pub const STANDARD_RIGHTS_READ    : ACCESS_MASK = READ_CONTROL;
    pub const STANDARD_RIGHTS_WRITE   : ACCESS_MASK = READ_CONTROL;
    pub const STANDARD_RIGHTS_EXECUTE : ACCESS_MASK = READ_CONTROL;
    pub const FILE_READ_DATA           : ACCESS_MASK = 0x0001;    // file & pipe
    pub const FILE_LIST_DIRECTORY      : ACCESS_MASK = 0x0001;    // directory
    pub const FILE_WRITE_DATA          : ACCESS_MASK = 0x0002;    // file & pipe
    pub const FILE_ADD_FILE            : ACCESS_MASK = 0x0002;    // directory
    pub const FILE_APPEND_DATA         : ACCESS_MASK = 0x0004;    // file
    pub const FILE_ADD_SUBDIRECTORY    : ACCESS_MASK = 0x0004;    // directory
    pub const FILE_CREATE_PIPE_INSTANCE: ACCESS_MASK = 0x0004;    // named pipe
    pub const FILE_READ_EA             : ACCESS_MASK = 0x0008;    // file & directory
    pub const FILE_WRITE_EA            : ACCESS_MASK = 0x0010;    // file & directory
    pub const FILE_EXECUTE             : ACCESS_MASK = 0x0020;    // file
    pub const FILE_TRAVERSE            : ACCESS_MASK = 0x0020;    // directory
    pub const FILE_DELETE_CHILD        : ACCESS_MASK = 0x0040;    // directory
    pub const FILE_READ_ATTRIBUTES     : ACCESS_MASK = 0x0080;    // all
    pub const FILE_WRITE_ATTRIBUTES    : ACCESS_MASK = 0x0100;    // all
    pub const FILE_ALL_ACCESS: ACCESS_MASK = STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1FF;
    pub const FILE_GENERIC_READ: ACCESS_MASK = STANDARD_RIGHTS_READ |
                                       FILE_READ_DATA           |
                                       FILE_READ_ATTRIBUTES     |
                                       FILE_READ_EA             |
                                       SYNCHRONIZE;
    pub const FILE_GENERIC_WRITE: ACCESS_MASK = STANDARD_RIGHTS_WRITE    |
                                       FILE_WRITE_DATA          |
                                       FILE_WRITE_ATTRIBUTES    |
                                       FILE_WRITE_EA            |
                                       FILE_APPEND_DATA         |
                                       SYNCHRONIZE;
    pub const FILE_GENERIC_EXECUTE: ACCESS_MASK = STANDARD_RIGHTS_EXECUTE  |
                                       FILE_READ_ATTRIBUTES     |
                                       FILE_EXECUTE             |
                                       SYNCHRONIZE;
}

pub mod FileAttributes {
    use super::DWORD;
    pub const FILE_ATTRIBUTE_ARCHIVE: DWORD = 0x20;
    pub const FILE_ATTRIBUTE_COMPRESSED: DWORD = 0x800;
    pub const FILE_ATTRIBUTE_DEVICE: DWORD = 0x40;
    pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x10;
    pub const FILE_ATTRIBUTE_ENCRYPTED: DWORD = 0x4000;
    pub const FILE_ATTRIBUTE_HIDDEN: DWORD = 0x2;
    pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: DWORD = 0x8000;
    pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x80;
    pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: DWORD = 0x2000;
    pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: DWORD = 0x20000;
    pub const FILE_ATTRIBUTE_OFFLINE: DWORD = 0x1000;
    pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x1;
    pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: DWORD = 0x400000;
    pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: DWORD = 0x40000;
    pub const FILE_ATTRIBUTE_REPARSE_POINT: DWORD = 0x400;
    pub const FILE_ATTRIBUTE_SPARSE_FILE: DWORD = 0x200;
    pub const FILE_ATTRIBUTE_SYSTEM: DWORD = 0x4;
    pub const FILE_ATTRIBUTE_TEMPORARY: DWORD = 0x100;
    pub const FILE_ATTRIBUTE_VIRTUAL: DWORD = 0x10000;
}

pub mod CreateDisposition {
    use super::DWORD;
    pub const CREATE_NEW: DWORD = 1;
    pub const CREATE_ALWAYS: DWORD = 2;
    pub const OPEN_EXISTING: DWORD = 3;
    pub const OPEN_ALWAYS: DWORD = 4;
    pub const TRUNCATE_EXISTING: DWORD = 5;
}

pub mod ShareMode {
    use super::DWORD;
    pub const FILE_SHARE_DELETE: DWORD = 0x00000004;
    pub const FILE_SHARE_READ: DWORD = 0x00000001;
    pub const FILE_SHARE_WRITE: DWORD = 0x00000002;
}

pub mod CreateOptions {
    use super::DWORD;
    pub const FILE_OPEN: DWORD = 0x00000001;
    pub const FILE_CREATE: DWORD = 0x00000002;
    pub const FILE_OPEN_IF: DWORD = 0x00000003;
    pub const FILE_OVERWRITE: DWORD = 0x00000004;
    pub const FILE_OVERWRITE_IF: DWORD = 0x00000005;
    pub const FILE_NON_DIRECTORY_FILE: DWORD = 0x00000040;
}

pub mod ObjectAttributes {
    use super::DWORD;
    pub const OBJ_INHERIT         : DWORD = 0x00000002;
    pub const OBJ_PERMANENT       : DWORD = 0x00000010;
    pub const OBJ_EXCLUSIVE       : DWORD = 0x00000020;
    pub const OBJ_CASE_INSENSITIVE: DWORD = 0x00000040;
    pub const OBJ_OPENIF          : DWORD = 0x00000080;
    pub const OBJ_OPENLINK        : DWORD = 0x00000100;
    pub const OBJ_VALID_ATTRIBUTES: DWORD = 0x000001F2;
}

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length:        USHORT,
    pub MaximumLength: USHORT,
    pub Buffer:        PWSTR,
}
#[repr(C)]
pub struct OBJECT_ATTRIBUTES {
    pub Length:                   ULONG,
    pub RootDirectory:            HANDLE,
    pub ObjectName:               *mut UNICODE_STRING,
    pub Attributes:               ULONG,
    pub SecurityDescriptor:       PVOID,
    pub SecurityQualityOfService: PVOID,
}
#[repr(C)]
pub union IO_STATUS_BLOCK_u {
    pub Status: VP_STATUS,
    pub Pointer: PVOID,
}
#[repr(C)]
pub struct IO_STATUS_BLOCK {
    pub u: IO_STATUS_BLOCK_u,
    pub Information: ULONG_PTR,
}

#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress:       PVOID,
    pub AllocationBase:    PVOID,
    pub AllocationProtect: DWORD,
    pub RegionSize:        usize,
    pub State:             DWORD,
    pub Protect:           DWORD,
    pub Type:              DWORD,
}

#[repr(u32)]
pub enum MEMORY_INFORMATION_CLASS {
    MemoryBasicInformation = 0, // MEMORY_BASIC_INFORMATION
    MemoryWorkingSetInformation = 1, // MEMORY_WORKING_SET_INFORMATION
    MemoryMappedFilenameInformation = 2, // UNICODE_STRING
    MemoryRegionInformation = 3, // MEMORY_REGION_INFORMATION
    MemoryWorkingSetExInformation = 4, // MEMORY_WORKING_SET_EX_INFORMATION
    MemorySharedCommitInformation = 5, // MEMORY_SHARED_COMMIT_INFORMATION
}
// type IO_APC_ROUTINE = extern fn(PVOID, *mut IO_STATUS_BLOCK, u32);
type IO_APC_ROUTINE = PVOID;

#[repr(u32)]
pub enum EVENT_TYPE {
    NotificationEvent = 0,
    SynchronizationEvent = 1,
}

pub mod EventAccess {
    use super::DWORD;
    pub const DELETE: DWORD = 0x00010000;
    pub const READ_CONTROL: DWORD = 0x00020000;
    pub const SYNCHRONIZE: DWORD = 0x00100000;
    pub const WRITE_DAC: DWORD = 0x00040000;
    pub const WRITE_OWNER: DWORD = 0x00080000;
    pub const EVENT_ALL_ACCESS: DWORD = 0x1F0003;
    pub const EVENT_MODIFY_STATE: DWORD = 0x0002;
    pub const MUTEX_ALL_ACCESS: DWORD = 0x1F0001;
    pub const MUTEX_MODIFY_STATE: DWORD = 0x0001;
    pub const SEMAPHORE_ALL_ACCESS: DWORD = 0x1F0003;
    pub const SEMAPHORE_MODIFY_STATE: DWORD = 0x0002;
    pub const TIMER_ALL_ACCESS: DWORD = 0x1F0003;
    pub const TIMER_MODIFY_STATE: DWORD = 0x0002;
    pub const TIMER_QUERY_STATE: DWORD = 0x0001;
}

#[repr(u32)]
pub enum MemoryState {
    MEM_COMMIT = 0x1000,
    MEM_FREE = 0x10000,
    MEM_RESERVE = 0x2000,
}

pub const STATUS_SUCCESS: NTSTATUS = 0x00000000;
pub const STATUS_PENDING: NTSTATUS = 0x00000103;

extern {
    pub fn NtCreateFile(
        FileHandle:        *mut HANDLE,
        DesiredAccess:     ACCESS_MASK,
        ObjectAttributes:  *mut OBJECT_ATTRIBUTES,
        IoStatusBlock:     *mut IO_STATUS_BLOCK,
        AllocationSize:    *mut LARGE_INTEGER,
        FileAttributes:    ULONG,
        ShareAccess:       ULONG,
        CreateDisposition: ULONG,
        CreateOptions:     ULONG,
        EaBuffer:          PVOID,
        EaLength:          ULONG,
    ) -> NTSTATUS;

    pub fn NtWriteFile(
        FileHandle:    HANDLE,
        Event:         HANDLE,
        ApcRoutine:    IO_APC_ROUTINE,
        ApcContext:    PVOID,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        Buffer:        PVOID,
        Length:        ULONG,
        ByteOffset:    *mut LARGE_INTEGER,
        Key:           *mut ULONG,
        ) -> NTSTATUS;

    pub fn NtQueryVirtualMemory(
        ProcessHandle:           HANDLE,
        BaseAddress:             PVOID,
        MemoryInformationClass:  MEMORY_INFORMATION_CLASS,
        MemoryInformation:       PVOID,
        MemoryInformationLength: SIZE_T,
        ReturnLength:            *mut SIZE_T,
        ) -> NTSTATUS;

    pub fn NtCreateEvent(
        EventHandle: *mut HANDLE,
        DesiredAccess: ACCESS_MASK,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        EventType: EVENT_TYPE,
        InitialState: BOOLEAN,
        ) -> NTSTATUS;

    pub fn NtWaitForSingleObject(
        ObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        TimeOut: *mut LARGE_INTEGER,
        ) -> NTSTATUS;

    pub fn NtResetEvent(
        EventHandle: HANDLE,
        PreviousState: *mut LONG) -> NTSTATUS;

    pub fn NtClose(Handle: HANDLE) -> NTSTATUS;
}

global_asm!(r#"
.intel_syntax
.macro define_syscall name, id
.global \name
\name:
    mov r10, rcx
    mov eax, \id
    syscall
    ret
.endm

define_syscall NtWaitForSingleObject, 0x04
define_syscall NtWriteFile, 0x08
define_syscall NtClose, 0x0f
define_syscall NtQueryVirtualMemory, 0x23
define_syscall NtCreateEvent, 0x48
define_syscall NtResetEvent, 0x172
define_syscall NtCreateFile, 0x55

.att_syntax
"#);
