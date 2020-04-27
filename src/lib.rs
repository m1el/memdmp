#![no_std]
#![feature(llvm_asm, lang_items)]
#![feature(global_asm, start)]

mod core_reqs;
mod syscalls;
mod cursor;
mod memory;
mod wstr;

//use core::ffi::{c_void};
use core::ptr::{null_mut};
use syscalls::*;
use cursor::Cursor;
use memory::RegionWalker;
use wstr::ascii_to_wstr;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[panic_handler]
unsafe fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop { }
}

#[no_mangle]
pub unsafe fn shellcode() -> isize {
    let mut file_handle: HANDLE = null_mut();
    let mut attrib = core::mem::zeroed::<OBJECT_ATTRIBUTES>();
    attrib.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as u32;
    attrib.Attributes = ObjectAttributes::OBJ_CASE_INSENSITIVE as u32;
    let mut buf = [0u16; 256];
    let w_path = ascii_to_wstr(r#"\??\c:\users\null\memory_dump.bin"#, &mut buf)
        .unwrap();

    let path_len = core::mem::size_of_val(w_path);
    let mut object_name = UNICODE_STRING {
        Length: path_len as u16,
        MaximumLength: path_len as u16,
        Buffer: w_path.as_ptr() as _,
    };
    attrib.ObjectName = &mut object_name;

    let mut io_status = core::mem::zeroed::<IO_STATUS_BLOCK>();
    let mut allocation_size = 0_u64;

    let status = NtCreateFile(
        &mut file_handle, AccessMask::FILE_GENERIC_WRITE,
        &mut attrib, &mut io_status, &mut allocation_size,
        FileAttributes::FILE_ATTRIBUTE_NORMAL as u32,
        ShareMode::FILE_SHARE_WRITE as u32,
        CreateDisposition::TRUNCATE_EXISTING as u32,
        CreateOptions::FILE_NON_DIRECTORY_FILE as u32,
        null_mut(), 0);

    assert!(status == 0);

    //let mut wait_event = null_mut();
    //let status = NtCreateEvent(
    //    &mut wait_event, EventAccess::EVENT_ALL_ACCESS, null_mut(),
    //    EVENT_TYPE::SynchronizationEvent, 0);
    // if status != 0 { return status as usize; }

    let buf = &mut [0_u8; 32];
    let mut offset = 0;
    for memory_info in RegionWalker::new() {
        if memory_info.State != MemoryState::MEM_COMMIT as u32 { continue; }
        let mut cursor = Cursor::new(buf);
        cursor.write_bytes(&(memory_info.BaseAddress as u64).to_ne_bytes());
        cursor.write_bytes(&memory_info.RegionSize.to_ne_bytes());
        cursor.write_bytes(&memory_info.State.to_ne_bytes());
        cursor.write_bytes(&memory_info.Protect.to_ne_bytes());
        cursor.write_bytes(&memory_info.Type.to_ne_bytes());
        let result = cursor.get();
        let status = NtWriteFile(
            file_handle, null_mut(), null_mut(), null_mut(), &mut io_status,
            result.as_ptr() as _, result.len() as u32, &mut offset, null_mut());
        assert!(status == 0 || status == STATUS_PENDING);
        assert!(io_status.Information == result.len());
        offset += result.len() as u64;
    }

    {
        let result = 0_u64.to_ne_bytes();
        let status = NtWriteFile(
            file_handle, null_mut(), null_mut(), null_mut(), &mut io_status,
            result.as_ptr() as _, result.len() as u32, &mut offset, null_mut());
        assert!(status == 0 || status == STATUS_PENDING);
        assert!(io_status.Information == result.len());
        offset += result.len() as u64;
    }

    for memory_info in RegionWalker::new() {
        if memory_info.State != MemoryState::MEM_COMMIT as u32 { continue }
        let status = NtWriteFile(
            file_handle, null_mut(), null_mut(), null_mut(), &mut io_status,
            memory_info.BaseAddress, memory_info.RegionSize as u32, &mut offset, null_mut());
        assert!(status == 0 || status == STATUS_PENDING);
        assert!(io_status.Information == memory_info.RegionSize);
        offset += memory_info.RegionSize as u64;
    }

    // NtClose(wait_event);
    NtClose(file_handle);

    return status as isize;
}
