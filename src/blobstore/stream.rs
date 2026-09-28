#![allow(unused)]
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;

// NYI Stream is currently based on an in-memory buffer. Make a version tht is file backed, and only use the memory version
// for testing.


pub struct Stream {
     buffer : Cursor<Vec<u8>>   
}


impl Stream {
    pub fn new() -> Self {
        Stream {
            buffer : Cursor::new(Vec::new())
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Stream {
            buffer : Cursor::new(bytes)
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer.get_ref().as_slice()
    }

    
    pub fn read_usize(&mut self) -> usize {

        let mut buffer = [0_u8; std::mem::size_of::<usize>()];
        self.read_exact(&mut buffer);
        usize::from_le_bytes(buffer)
    }


    pub fn read_u32(&mut self) -> u32 {

        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        self.read_exact(&mut buffer);
        u32::from_le_bytes(buffer)
    }


    pub fn read_u64(&mut self) -> u64 {

        let mut buffer = [0_u8; std::mem::size_of::<u64>()];
        self.read_exact(&mut buffer);
        u64::from_le_bytes(buffer)
    }


    pub fn read_u128(&mut self) -> u128 {

        let mut buffer = [0_u8; std::mem::size_of::<u128>()];
        self.read_exact(&mut buffer);
        u128::from_le_bytes(buffer)
    }    

}


impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.buffer.read(buf)
    }
}


impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}


impl Seek for Stream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.buffer.seek(pos)
    }
}

