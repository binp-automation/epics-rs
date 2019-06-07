use std::ffi::{CStr};
use std::str::{from_utf8, Utf8Error};

use libc::{c_char};


pub fn cstr_array_write(dst: &mut [c_char], src: &str) {
    cstr_array_write_bytes(dst, src.as_bytes())
}

pub fn cstr_array_read(src: &[c_char]) -> Result<&str, Utf8Error> {
    from_utf8(cstr_array_read_bytes(src))
}

pub fn cstr_array_write_bytes(dst: &mut [c_char], src: &[u8]) {
    let maxlen = dst.len() - 1;
    let src = if src.len() > maxlen {
        &src[..maxlen]
    } else {
        src
    };
    let src = unsafe{ &*( src as *const [u8] as *const [i8] ) };
    dst[..src.len()].copy_from_slice(src);
    dst[src.len()] = b'\0' as i8;
}

pub fn cstr_array_read_bytes(src: &[c_char]) -> &[u8] {
    let maxlen = src.len();
    let mut len = maxlen;
    for i in 0..maxlen {
        if unsafe { *src.get_unchecked(i) } == b'\0' as i8 {
            len = i;
            break;
        }
    }
    unsafe{ &*( &src[..len] as *const _ as *const [u8] ) }
}

pub unsafe fn cstr_ptr_read<'a>(src: *const c_char) -> Option<Result<&'a str, Utf8Error>> {
    cstr_ptr_read_bytes(src).map(|s| from_utf8(s))
}

pub unsafe fn cstr_ptr_read_bytes<'a>(src: *const c_char) -> Option<&'a [u8]> {
    if src.is_null() {
        None
    } else {
        Some(CStr::from_ptr::<'a>(src).to_bytes())
    }
}
