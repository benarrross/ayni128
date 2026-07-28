#![allow(unused)]
mod blobstore;
mod sortedarray;
mod bplustree;

use std::fs::File;
use std::io::Cursor;
use std::io::BufWriter;
use std::path::Path;
use blobstore::*;
use bplustree::*;


fn main() {

    let path = Path::new("test.blobstore");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);

    let blob_contents: [u8;_] = [1, 2, 3];
    let root_blob_id = blobs.put(&blob_contents);

    let _read = blobs.get(root_blob_id);
}