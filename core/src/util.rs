use std::io::{self, Read, Seek};

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek + ?Sized> ReadSeek for T {}

/// # Errors
/// Returns Err if there is an error reading the file
pub fn is_zlib<F: ReadSeek>(f: &mut F) -> io::Result<bool> {
    let mut buf = [0u8; 5];
    let result = f.read_exact(&mut buf);
    f.rewind()?;
    Ok(result.map_or(false, |_| buf[4] == b'\x78'))
}
