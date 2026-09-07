use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BPlusTree;
use super::graph::*;
use super::attributebynodetable;
use super::attributesbynametable;
use super::nodestable;
use super::edge::*;

pub struct GraphView<'a> {
    based_on: &'a Graph,
    nodes: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    edges_from: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    edges_to: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    attributes_by_node: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    attributes_by_name: crate::bplustree::View<'a, TREE_NODE_SIZE>,
}


impl <'a> GraphView<'a> {

    pub fn create_node(&self) -> NodeId {
        let node = self.based_on.get_next_node_id();
        self.nodes.put(nodestable::encode(&node));
        node
    }

    pub fn set_attribute(&self, node: NodeId, name: AttributeName, value: StringId) {
        self.attributes_by_node.put(attributebynodetable::encode(&node, &name, &value));
        self.attributes_by_name.put(attributesbynametable::encode(&name, &value, &node));
    }

    pub fn get_attribute(&self, node: NodeId, name: AttributeName) -> Option<StringId> {
        let found_encoded = self.attributes_by_node.get(attributebynodetable::encode_any_value(&node, &name));
        let attr = attributebynodetable::decode(&found_encoded);
        if (attr.node == node && attr.name == name) {
            Some(attr.value)
        } else {
            None
        }
    }

    pub fn iter_attributes(&self, node: NodeId) -> AttrByNodeIterator<'a> {
        unimplemented!();
    }

    pub fn iter_nodes_with_attribute(&self, name: AttributeName, value: StringId) -> AttrByNameValueIterator<'a> {
        unimplemented!();
    }

    pub fn insert_edge(&self, from: NodeId, to: NodeId, name: EdgeName, edge_type: EdgeType, order: EdgeOrder ) {
        unimplemented!();
    }

    pub fn get_edge_from(&self, node:NodeId, name: EdgeName) -> Option<Edge> {
        unimplemented!();
    }

    pub fn iter_edges_from(&self, node: NodeId, name: EdgeName) -> EdgeFromIterator<'a> {
        unimplemented!();
    }

    pub fn iter_edges_to(&self, node: NodeId, name: EdgeName) -> EdgeToIterator<'a> {
        unimplemented!();
    }

}


pub struct AttrByNameValueIterator<'a> {
    based_on_view: &'a GraphView<'a>,
}


impl<'a> Iterator for AttrByNameValueIterator<'a> {

    type Item = StringId;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }   
}


pub struct AttrByNodeIterator<'a> {
    based_on_view: &'a GraphView<'a>,
}


impl<'a> Iterator for AttrByNodeIterator<'a> {

    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }   
}

