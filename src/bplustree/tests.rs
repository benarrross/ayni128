#[cfg(test)]
use std::io::Cursor;
use crate::blobstore::*;
use super::BPlusTree;


#[test]
fn create_empty() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);
    let mut list = BPlusTree::<4>::new(&mut blobs);

    //list.put(32);
}


#[test]
fn enum_empty() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);
    let mut list = BPlusTree::<4>::new(&mut blobs);

    let mut view = list.get_view();

    for item in view.iter(0, u128::MAX) {
        assert!(false, "Should not find an item");
    }
    //list.put(32);
}


#[test]
fn insert_one() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);
    let mut list = BPlusTree::<4>::new(&mut blobs);

    // Insert 99 in a view (but don't commit it yet)
    let mut view = list.get_view();
    view.put(99);
    for item in view.iter(0, u128::MAX) {
        assert_eq!(99, item);
    }
    assert_eq!(1, view.iter(0, u128::MAX).into_iter().count());

    // Ensure we don't see 99 outside of the view until we commit
    for item in list.get_view().iter(0, u128::MAX) {
        assert!(false, "Should not find an item");
    }

    // Commit and ensure we can see 99
    // list.commit(view);
    // for item in list.get_view().iter(0, u128::MAX) {
    //     assert_eq!(99, item);
    // }
}

#[test]
fn insert_several() {
    let mut memory_buffer = MemoryStream::new();
    let mut blobs = BlobStore::new(& mut memory_buffer);
    let mut list = BPlusTree::<4>::new(&mut blobs);

    // Insert 99 in a view (but don't commit it yet)
    let mut view = list.get_view();
    view.put(99);
    view.put(10);
    view.put(32);
    view.put(999);

    let mut iter = view.iter(0, u128::MAX).into_iter();
    assert_eq!(10_u128, iter.next().unwrap());
    assert_eq!(32_u128, iter.next().unwrap());
    assert_eq!(99_u128, iter.next().unwrap());
    assert_eq!(999_u128, iter.next().unwrap());
    assert!(iter.next().is_none());

    // Commit and ensure we can see 99
    // list.commit(view);
    // for item in list.get_view().iter(0, u128::MAX) {
    //     assert_eq!(99, item);
    // }
}
