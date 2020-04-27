use core::fmt;

pub struct Cursor<'a> {
    buf: &'a mut [u8],
    written: usize,
}

impl<'a> Cursor<'a> {
    pub fn new<'b>(buf: &'b mut [u8]) -> Cursor<'b> {
        Cursor {
            buf,
            written: 0
        }
    }
    pub fn get(&self) -> &[u8] {
        &self.buf[..self.written]
    }
    /*
    pub fn reset(&mut self) -> usize {
        let mut written = 0;
        core::mem::swap(&mut written, &mut self.written);
        written
    }
    */
    pub fn write_bytes(&mut self, data: &[u8]) -> Option<()> {
        let data_len = data.len();
        if self.buf.len() < data_len + self.written {
            return None;
        }
        self.buf[self.written..(data_len + self.written)]
            .copy_from_slice(data);
        self.written += data_len;
        Some(())
    }
}

impl<'a> fmt::Write for Cursor<'a> {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self.write_bytes(data.as_bytes())
            .ok_or(fmt::Error)
    }
}
