// convert ascii string to wide string
pub fn ascii_to_wstr<'a>(src: &str, dst: &'a mut[u16]) -> Option<&'a[u16]> {
    let src = src.as_bytes();
    if src.len() > dst.len() {
        return None;
    }
    unsafe {
        for index in 0..src.len() {
            let chr = *src.get_unchecked(index);
            if chr > 127 { return None; }
            *dst.get_unchecked_mut(index) = chr as u16;
        }
        Some(dst.get_unchecked_mut(..src.len()))
    }
}
