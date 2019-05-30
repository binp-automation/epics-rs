use std::borrow::Cow;

use libc::c_char;


pub fn cstr_copy_from_slice(dst: &mut [c_char], src: &[u8]) {
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

pub fn cstr_to_slice(src: &[c_char]) -> &[u8] {
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

pub fn lossy<'a>(r: &'a [u8]) -> Cow<'a, str> {
    String::from_utf8_lossy(r)
}
