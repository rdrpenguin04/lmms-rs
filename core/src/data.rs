use crate::util::{self, ReadSeek};
use flate2::read::ZlibDecoder;
use std::io::{self, Read};

#[derive(Debug)]
pub struct Track {}

#[derive(Debug)]
pub struct Project {
    pub tracks: Vec<Track>,
}

impl Project {
    /// # Errors
    /// Returns Err if there is an error reading the file
    pub fn from_file<F: ReadSeek>(mut f: F) -> io::Result<Self> {
        if util::is_zlib(&mut f)? {
            Self::from_file_uncompressed(&mut ZlibDecoder::new(f))
        } else {
            Self::from_file_uncompressed(&mut f)
        }
    }

    /// # Errors
    /// Returns Err if there is an error reading the file
    #[allow(clippy::unnecessary_wraps)]
    fn from_file_uncompressed<F: Read>(_f: &mut F) -> io::Result<Self> {
        Ok(Self { tracks: Vec::new() })
    }
}
