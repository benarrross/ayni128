#[cfg(test)]
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::blobstore::*;
use crate::graph::*;

/* 
TO DO
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

    assert_attributes_match(&test_data, n1, &view);
}


fn assert_attributes_match(test_data: &[(&[u8], &[u8])], node: NodeId, graph_view: &GraphView) {
    let mut test_copy = test_data.to_vec();
    for attr in graph_view.iter_attributes(node) {
        let name = graph_view.get_string(&attr.name.into());
        let value = graph_view.get_string(&attr.value.into());

        let mut found = false;
        for index in 0..test_copy.len() {
            let datum = test_copy[index];
            if (datum.0 == name && datum.1 == value) {
                test_copy.remove(index);
                found = true;
                break;
            }
        }
        assert!(found);
    }
    assert_eq!(0, test_copy.len());
}