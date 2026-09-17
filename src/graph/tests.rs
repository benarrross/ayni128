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

/* 
TO DO
- Add several attributes and enumerate them
- Add several nodes with attributes
- Find nodes by attribute
- Commit a view
- Add edges between nodes
- Enumerate edges to and from a node
- Poplulate a graph, commit it, reload it from storage
- Create many nodes in several concurrent transactions
 */

#[test]
fn create_one_node_and_attribute() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut graph = Graph::new(memory_buffer);

    let view = graph.get_view();

    let attr_name : AttributeName = view.get_stringid(b"attr1").into();
    let test_value = view.get_stringid(b"test_value");

    let n1 = view.create_node();
    view.set_attribute(n1, attr_name, test_value);

    assert_eq!(test_value, view.get_attribute(n1, attr_name).unwrap());
}


#[test]
fn enumerate_several_attributes() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut graph = Graph::new(memory_buffer);

    let view = graph.get_view();

    let test_data: [(&[u8], &[u8]); _] = [
        (b"a1", b"value1"),
        (b"a2", b"value2"),
        (b"a3", b"value3"),
        (b"a4", b"value4")
    ];

    let n1 = view.create_node();
    for datum in test_data {
        view.set_attribute_str(n1, datum.0, datum.1);
    }

    let mut test_data_iter = test_data.iter();
    for attr in view.iter_attributes(n1) {

        let datum = &test_data_iter.next().unwrap();
        let expected_name = datum.0;
        let expected_value = datum.1;

        let name = view.get_string(&attr.name.into());
        let value = view.get_string(&attr.value.into());

        assert_eq!(n1, attr.node);
        assert_eq!(expected_name, name);
        assert_eq!(expected_value, value);
    }

    assert_eq!(None, test_data_iter.next());
}
