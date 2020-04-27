#![no_std]
#![no_main]

extern crate memdmp;

#[no_mangle]
pub unsafe extern fn wmain(_argc: isize, _argv: *const *const u16) -> isize {
    memdmp::shellcode()
}
