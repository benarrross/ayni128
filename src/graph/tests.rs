#[cfg(test)]
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use crate::blobstore::*;
use crate::BlobStore;
use crate::PersistedSortedList;
use super::attribute::*;
use super::view::*;
use super::graph::*;
use super::strings::StringId;


#[test]
fn create_one_node_and_attribute() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut graph = Graph::new(memory_buffer);

    let view = graph.get_view();

    let name_id = view.get_stringid(b"name");
    let attr_id = AttributeName(view.get_stringid(b"attr1"));
    let test_id = view.get_stringid(b"test_value");

    let n1 = view.create_node();
    view.set_attribute(n1, attr_id, test_id);

    assert_eq!(test_id, view.get_attribute(n1, attr_id).unwrap());
}
