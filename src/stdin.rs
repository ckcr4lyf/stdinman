use std::io::{Read, Seek};
use log::debug;
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

pub fn early_stdin_consumer (rx: std::sync::mpsc::Receiver<bool>) {
    let mut buf = vec![0; 1024];

    loop {
        // check if we've received the stop signal
        if let Ok(_) = rx.try_recv() {
            debug!("received stop signal! will yield stdin");
            break;
        }

        // otherwise just read stdin and discard it
        if let Ok(n) = std::io::stdin().read(&mut buf) {
            if n == 0 {
                debug!("no data read! will yield stdin");
                break;
            }
        }
    }
}