#![allow(unused)]
//use std::{num::NonZeroU64};
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;


pub trait Stream : Read + Write + Seek {

    fn read_usize(&mut self) -> usize {

        let mut buffer = [0_u8; std::mem::size_of::<usize>()];
        self.read_exact(&mut buffer);
        usize::from_le_bytes(buffer)
    }


    fn read_u32(&mut self) -> u32 {

        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        self.read_exact(&mut buffer);
        u32::from_le_bytes(buffer)
    }


    fn read_u64(&mut self) -> u64 {

        let mut buffer = [0_u8; std::mem::size_of::<u64>()];
        self.read_exact(&mut buffer);
        u64::from_le_bytes(buffer)
    }


    fn read_u128(&mut self) -> u128 {

        let mut buffer = [0_u8; std::mem::size_of::<u128>()];
        self.read_exact(&mut buffer);
        u128::from_le_bytes(buffer)
    }    
}


pub struct MemoryStream {
     buffer : Cursor<Vec<u8>>   
}


impl MemoryStream {
    pub fn new() -> Self {
        MemoryStream {
            buffer : Cursor::new(Vec::new())
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer.get_ref().as_slice()
    }
}

impl Stream for MemoryStream {}

impl Read for MemoryStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}


impl Write for MemoryStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}


impl Seek for MemoryStream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.buffer.seek(pos)
    }
}

