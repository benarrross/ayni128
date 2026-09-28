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


pub struct BlobStore {
    backing_store : Arc<Mutex<Stream>>
}


impl BlobStore {

    pub fn new (mut stream : Stream) -> Self {

        // Figure out if we need to initialize a new blob store
        let file_length = stream.seek(SeekFrom::End(0)).unwrap();

        if file_length == 0 {

            // Write out a default file with just a header
            let file_header = FileHeader::default();
            file_header.serialize(&mut stream);

            return BlobStore { 
                backing_store: Arc::new(Mutex::new(stream))
            };
        }
        else {
            stream.seek(SeekFrom::Start(0));
            let file_header = FileHeader::read(&mut stream);

            BlobStore { 
                backing_store: Arc::new(Mutex::new(stream)),
           }
        }   
    }


    pub fn put(& mut self, contents: &[u8]) -> BlobId {

        // NYI look for free space to put the blob in        
        let mut stream = self.backing_store.lock().unwrap();
        
        let position = stream.seek(SeekFrom::End(0)).unwrap();
        stream.write_all(&contents.len().to_le_bytes());
        stream.write_all(contents);

        return BlobId::new(NonZeroU64::new(position).unwrap());
    }


    pub fn get(& mut self, blobid: BlobId) -> Vec<u8> {

        let mut stream = self.backing_store.lock().unwrap();

        stream.seek(SeekFrom::Start(blobid.value().into()));
        let mut buffer : Vec<u8> = vec![0; stream.read_usize()];
        stream.read(&mut buffer);

        buffer
    }
    

    pub fn delete(&mut self, blobid: BlobId) {
        panic!("NYI");
    }


    pub fn get_root_blobid(&mut self) -> BlobId {
        //let mut backing_store = self.backing_store_lock.lock().unwrap();
        let mut stream = self.backing_store.lock().unwrap();
        stream.seek(SeekFrom::Start(0));
        let file_header = FileHeader::read(&mut stream);
        
        file_header.root_blob_id
    }


    pub fn set_root_blobid(&mut self, blobid: BlobId) {
        let mut stream = self.backing_store.lock().unwrap();
        
        stream.seek(SeekFrom::Start(0));
        let mut file_header = FileHeader::read(&mut stream);
        file_header.root_blob_id = blobid;

        stream.seek(SeekFrom::Start(0));
        file_header.serialize(&mut stream);
    }

    
    pub fn get_bytes(&mut self) -> Vec<u8> {
        let mut stream = self.backing_store.lock().unwrap();
        let len: usize = stream.seek(SeekFrom::End(0)).unwrap() as usize;

        let mut buffer : Vec<u8> = vec![0; len];
        stream.seek(SeekFrom::Start(0));
        stream.read(&mut buffer);

        buffer
    }
}

