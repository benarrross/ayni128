#[cfg(test)]
use std::io::Cursor;
use crate::blobstore::*;


#[test]
fn put_get_one_blob() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);

    let blob_contents: [u8;_] = [1, 2, 3];
    let root_blob_id = blobs.put(&blob_contents);

    let read = blobs.get(root_blob_id);

    assert_eq_slices(&blob_contents, &read[..]);
}


#[test]
fn reopen_store_with_one_blob() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);

    let blob_contents: [u8;_] = [1, 2, 3];
    let root_blob_id = blobs.put(&blob_contents);

    let mut blobs = BlobStore::new(& mut memory_buffer);
    let read = blobs.get(root_blob_id);

    assert_eq_slices(&blob_contents, &read[..]);
}


fn assert_eq_slices(expected: &[u8], actual: &[u8]) {

    for index in 0..expected.len() {
        assert_eq!(expected[index], actual[index]);
    }
}
