#[cfg(test)]
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use crate::blobstore::*;
use crate::BlobStore;
use crate::BPlusTree;
use super::view::*;
use super::graph::*;


#[test]
fn create_one_node_and_attribute() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut graph = Graph::new(memory_buffer);

    let view = graph.get_view();

    let name_attribute : AttributeName = AttributeName { 0: 99 };
    let name_value : StringId = StringId { 0: 100 };

    let n1 = view.create_node();
    view.set_attribute(n1, name_attribute, name_value);

    assert_eq!(name_value, view.get_attribute(n1, name_attribute).unwrap());
}
