#![allow(unused)]
use std::{num::NonZeroU64};
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::thread;
use super::blobid::*;
use super::fileheader::*;
use super::blobdescriptor::*;
use super::stream::*;

/*
TO DO
- Implement delete and recyling space
- Tests, including multithreaded tests
*/


pub struct BlobStore <'a> {
    backing_store_lock : Arc<Mutex<&'a mut dyn Stream>>
}


impl <'a> BlobStore <'a>{

    pub fn new (backing_store : & 'a mut dyn Stream) -> Self {

        // Figure out if we need to initialize a new blob store
        let file_length = backing_store.seek(SeekFrom::End(0)).unwrap();

        if file_length == 0 {

            // Write out a default file with just a header
            let file_header = FileHeader::default();
            file_header.serialize(backing_store);

            return BlobStore { 
                backing_store_lock : Arc::new(Mutex::new(backing_store))
            };
        }
        else {
            backing_store.seek(SeekFrom::Start(0));
            let file_header = FileHeader::read(backing_store);

            BlobStore { 
                backing_store_lock : Arc::new(Mutex::new(backing_store)),
           }
        }   
    }


    pub fn put(& mut self, contents: &[u8]) -> BlobId {

        // NYI look for free space to put the blob in
        // NYI take a lock when looking for free space
        
        let mut backing_store = self.backing_store_lock.lock().unwrap();
        
        let position = backing_store.seek(SeekFrom::End(0)).unwrap();
        backing_store.write_all(&contents.len().to_le_bytes());
        backing_store.write_all(contents);

        return BlobId::new(NonZeroU64::new(position).unwrap());
    }


    pub fn get(& mut self, blobid: BlobId) -> Vec<u8> {

        let mut backing_store = self.backing_store_lock.lock().unwrap();

        backing_store.seek(SeekFrom::Start(blobid.value().into()));
        let mut buffer : Vec<u8> = vec![0; backing_store.read_usize()];
        backing_store.read(&mut buffer);

        buffer
    }
    

    pub fn delete(&mut self, blobid: BlobId) {
        panic!("NYI");
    }


    pub fn get_root_blobid(&mut self) -> BlobId {
        let mut backing_store = self.backing_store_lock.lock().unwrap();
        
        backing_store.seek(SeekFrom::Start(0));
        let file_header = FileHeader::read(*backing_store);
        
        file_header.root_blob_id
    }


    pub fn set_root_blobid(&mut self, blobid: BlobId) {
        let mut backing_store = self.backing_store_lock.lock().unwrap();
        
        backing_store.seek(SeekFrom::Start(0));
        let mut file_header = FileHeader::read(*backing_store);
        file_header.root_blob_id = blobid;

        backing_store.seek(SeekFrom::Start(0));
        file_header.serialize(*backing_store);
    }
}

