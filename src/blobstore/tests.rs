#[cfg(test)]
use std::io::Cursor;
use crate::blobstore::*;


#[test]
fn put_get_one_blob() {
    let mut blobs = BlobStore::new(Stream::new());

    let blob_contents: [u8;_] = [1, 2, 3];
    let root_blob_id = blobs.put(&blob_contents);

    let read = blobs.get(root_blob_id);

    assert_eq_slices(&blob_contents, &read[..]);
}


#[test]
fn reopen_store_with_one_blob() {
    let mut blobs = BlobStore::new(Stream::new());

    let blob_contents: [u8;_] = [1, 2, 3];
    let root_blob_id = blobs.put(&blob_contents);

    let bytes_copy = blobs.get_bytes();
    let mut blobs = BlobStore::new(Stream::from_bytes(bytes_copy));
    let read = blobs.get(root_blob_id);

    assert_eq_slices(&blob_contents, &read[..]);
}


fn assert_eq_slices(expected: &[u8], actual: &[u8]) {

    for index in 0..expected.len() {
        assert_eq!(expected[index], actual[index]);
    }
}


// #[test]
// fn concurent_writes() {

//     const K:usize = 4;
//     let mut memory_buffer = Box::new(MemoryStream::new());
//     let mut blobs = Arc::new(Mutex::new(BlobStore::new(memory_buffer)));

//     let mut list = Table::<4>::new(blobs.clone());
//     for i in 0..10 {
//         let hthread = thread::spawn(move || {
//             let view = list.get_view();
//             view.insert(i);
//             list.commit(&view);
//         });
//     }
// }
