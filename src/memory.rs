use core::ffi::c_void;
use core::ptr::null_mut;
use crate::syscalls::*;

pub struct RegionWalker {
    ptr: *mut c_void,
}

impl RegionWalker {
    pub fn new() -> Self {
        RegionWalker {
            ptr: null_mut(),
        }
    }
}

impl Iterator for RegionWalker {
    type Item = MEMORY_BASIC_INFORMATION;

    fn next(&mut self) -> Option<Self::Item> {
        let mut return_length = 0;
        let mbi_len = core::mem::size_of::<MEMORY_BASIC_INFORMATION>();

        unsafe {
            let mut mbi: MEMORY_BASIC_INFORMATION = core::mem::zeroed();

            let status = NtQueryVirtualMemory(
                !0 as _, self.ptr,
                MEMORY_INFORMATION_CLASS::MemoryBasicInformation,
                &mut mbi as *mut MEMORY_BASIC_INFORMATION as _,
                mbi_len, &mut return_length);
            assert!(mbi_len == return_length);
            if status != 0 {
                return None;
            }

            self.ptr = (mbi.BaseAddress as usize + mbi.RegionSize) as _;
            Some(mbi)
        }
    }
}
