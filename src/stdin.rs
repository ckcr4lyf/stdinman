use std::io::{Read, Seek};
use songbird::input::reader::MediaSource;

/// A custom songbird::input::Reader which reades from stdin. 
/// The reader needs to songbird::input::reader::MediaSource
/// in order for it to be usable with songbird::input::Input
pub struct StdinReader;

impl Read for StdinReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        std::io::stdin().read(buf)
    }
}

impl Seek for StdinReader {
    // as per symphonia_core:io::MediaSource, we need to implement seek,
    // even though it may never actually be called
    fn seek(&mut self, _: std::io::SeekFrom) -> std::io::Result<u64> {
        unreachable!()
    }
}

impl MediaSource for StdinReader {
    // not possible to get length for stdin
    fn byte_len(&self) -> Option<u64> {
        None
    }

    // cannot seek stdin
    fn is_seekable(&self) -> bool {
        false
    }
}