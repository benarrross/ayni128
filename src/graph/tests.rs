#[cfg(test)]
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::blobstore::*;
use crate::graph::*;
//use crate::graph::strings::StringId;

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

    let node = create_node_str(&view, &test_data);
    assert_attributes_match(&test_data, node, &view);
    graph.commit(&view);

    let view = graph.get_view();
    assert_attributes_match(&test_data, node, &view);
}


#[test]
fn enumerate_several_nodes_by_attribute() {
    let mut memory_buffer = Box::new(MemoryStream::new());
    let mut graph = Graph::new(memory_buffer);

    let view = graph.get_view();

    let a1 : AttributeName = view.get_stringid(b"a1").into();
    let a2 : AttributeName = view.get_stringid(b"a2").into();
    let value1 = view.get_stringid(b"value1");
    let value2 = view.get_stringid(b"value2");
    let value3 = view.get_stringid(b"value3");

    let n1 = create_node(&view, &[(a1, value1), (a2, value2) ]);
    let n2 = create_node(&view, &[(a1, value1), (a2, value3) ]);
    let n3 = create_node(&view, &[(a1, value2), (a2, value3) ]);
    let n4 = create_node(&view, &[(a1, value2), (a2, value2) ]);
    let n5 = create_node(&view, &[(a1, value1), (a2, value2) ]);

    assert_nodes_match(&view.iter_nodes_with_attribute(a1, value1).collect::<Vec<NodeId>>(), &[n1, n2, n5]);
    assert_nodes_match(&view.iter_nodes_with_attribute(a1, value2).collect::<Vec<NodeId>>(), &[n3, n4]);
    assert_nodes_match(&view.iter_nodes_with_attribute(a2, value2).collect::<Vec<NodeId>>(), &[n1, n4, n5]);
    assert_nodes_match(&view.iter_nodes_with_attribute(a2, value3).collect::<Vec<NodeId>>(), &[n2, n3]);
    assert_nodes_match(&view.iter_nodes_with_attribute(a1, value3).collect::<Vec<NodeId>>(), &[]);
    assert_nodes_match(&view.iter_nodes_with_attribute(a2, value1).collect::<Vec<NodeId>>(), &[]);
    
    graph.commit(&view);
}


fn assert_nodes_match(actual: &[NodeId], expected: &[NodeId]) {
    
    assert_eq!(actual.len(), expected.len());

    let mut actual_vec : Vec<NodeId> = actual.iter().map(|&n| { n }).collect();
    actual_vec.sort();
    for node in expected {
        if let Ok(index) = actual_vec.binary_search(node) {
            actual_vec.remove(index);            
        }
        else {
            assert!(false);
        }
    }
}

fn create_node(view: &GraphView, attributes: &[(AttributeName, StringId)]) -> NodeId {
    let node = view.create_node();
    for attribute in attributes {
        view.set_attribute(node, attribute.0, attribute.1);
    }
    node
}

fn create_node_str(view: &GraphView, attributes: &[(&[u8], &[u8])]) -> NodeId {
    let node = view.create_node();
    for attribute in attributes {
        view.set_attribute_str(node, attribute.0, attribute.1);
    }
    node
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