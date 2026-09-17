#[cfg(test)]
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::blobstore::*;
use super::Table;
use super::ListView;

/*
TESTS TO ADD
- Insert values out-of-order
- view.get(n-1) on each to make sure we are chasing leaf nodes correctly for both get and enum (if separate code paths)
- Insert at the beginning of a leaf node to ensure we are setting split values correctly up several levels
- Concurrent transactions (make 2 or 3, edit them, then commit them after editing each one)
- Make a really large list (10000 entries, K=16)
*/


#[test]
fn create_empty() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut blobs = BlobStore::new(memory_buffer);
    let mut list = Table::<4>::new(Arc::new(Mutex::new(blobs)));
}


#[test]
fn enum_empty() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut blobs = BlobStore::new(memory_buffer);
    let mut list = Table::<4>::new(Arc::new(Mutex::new(blobs)));

    let mut view = list.get_view();

    for item in view.iter(0, u128::MAX) {
        assert!(false, "Should not find an item");
    }
}


#[test]
fn insert_one() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut blobs = BlobStore::new(memory_buffer);
    let mut list = Table::<4>::new(Arc::new(Mutex::new(blobs)));

    // Insert 99 in a view (but don't commit it yet)
    let view = list.get_view();
    view.put(99);
    for item in view.iter(0, u128::MAX) {
        assert_eq!(99, item);
    }
    assert_eq!(99, view.get(0));
    assert_eq!(99, view.get(99));
    assert_eq!(u128::MAX, view.get(100));
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
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut blobs = BlobStore::new(memory_buffer);
    let mut list = Table::<4>::new(Arc::new(Mutex::new(blobs)));
    let expected_values = vec![10, 32, 99, 999];

    let view = list.get_view();
    for expected_value in &expected_values {
        view.put(*expected_value);
    }

    assert_expected_values(&expected_values, &view);
    let mut actual_iter = view.iter(0, u128::MAX).into_iter();
    for expected_value in &expected_values {
        assert_eq!(*expected_value, actual_iter.next().unwrap());
    }
    assert!(actual_iter.next().is_none());


    // Commit and ensure we can see 99
    list.commit(&view);
    let view2 = list.get_view();
    assert_expected_values(&expected_values, &list.get_view());

    // Ensure searching for the next value is sane for each
    assert_eq!(10, view2.get(0));
    assert_eq!(32, view2.get(11));
    assert_eq!(99, view2.get(33));
    assert_eq!(999, view2.get(100));

    assert_eq!(10, view2.get(10));
    assert_eq!(32, view2.get(32));
    assert_eq!(99, view2.get(99));
    assert_eq!(999, view2.get(999));
}


#[test]
fn insert_many_in_order() {
    const K:usize = 4;
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut blobs = BlobStore::new(memory_buffer);
    let mut list = Table::<4>::new(Arc::new(Mutex::new(blobs)));
    let mut inserted_count = 0;
    let expected_values : Vec<u128> = (0..10).collect();

    let view_v0 = list.get_view();

    // Insert enough nodes that we need to do three splits
    let view_v1 = list.get_view();
    for value in &expected_values {
        view_v1.put(*value as u128);
        inserted_count += 1;

        let mut iter = view_v1.iter(0, u128::MAX);
        for value in 0..inserted_count {
            assert_eq!(value, iter.next().unwrap());
        }
        assert!(iter.next().is_none());

        assert_eq!(*value, view_v1.get(*value));

        assert_eq!(0, view_v0.iter(0, u128::MAX).count());
        assert_eq!(u128::MAX, view_v0.get(*value));
    }

    // Commit the edits
    list.commit(&view_v1);

    // Ensure we can see the edits
    assert_expected_values(&expected_values, &list.get_view());
}


fn assert_expected_values<'a, const K: usize>(expected: &Vec<u128>, actual: &ListView<'a, K>) {

    let mut actual_iter = actual.iter(0, u128::MAX).into_iter();
    for expected_value in expected {
        assert_eq!(*expected_value, actual_iter.next().unwrap());
    }
    assert!(actual_iter.next().is_none());

}

