use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BPlusTree;
use super::graph::*;
use super::attribute;
use super::node;
use super::node::*;
use super::attribute::by_node::*;
use super::attribute::by_name::*;
use super::edge::{*, edge_from::*, edge_to::*};


pub struct GraphView<'a> {
    based_on: &'a Graph,
    nodes: crate::bplustree::View<'a, TREE_NODE_SIZE>,  // NYI make view wrapper classes
    edges_from: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    edges_to: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    attributes_by_node: crate::bplustree::View<'a, TREE_NODE_SIZE>,
    attributes_by_name: crate::bplustree::View<'a, TREE_NODE_SIZE>,
}


impl <'a> GraphView<'a> {

    pub(crate) fn new(
        based_on: &'a Graph, 
        nodes_table: &'a NodesTable,
        edges_from_table: &'a BPlusTree<TREE_NODE_SIZE>,
        edges_to_table: &'a BPlusTree<TREE_NODE_SIZE>,
        attributes_by_node_table: &'a AttributeByNodeTable,
        attributes_by_name_table: &'a AttributeByNameTable) -> Self {
        
        GraphView { 
            based_on: based_on,
            nodes: nodes_table.0.get_view(),
            edges_from: edges_from_table.get_view(),
            edges_to: edges_to_table.get_view(),
            attributes_by_node: attributes_by_node_table.0.get_view(),
            attributes_by_name: attributes_by_name_table.0.get_view()
        }
    }

    
    pub fn create_node(&self) -> NodeId {
        let node = self.based_on.get_next_node_id();
        self.nodes.put(node::encode(&node));
        node
    }


    pub fn set_attribute(&self, node: NodeId, name: AttributeName, value: StringId) {
        self.attributes_by_node.put(attribute::by_node::encode(&node, &name, &value));
        self.attributes_by_name.put(attribute::by_name::encode(&name, &value, &node));
    }


    pub fn get_attribute(&self, node: NodeId, name: AttributeName) -> Option<StringId> {
        let found_encoded = self.attributes_by_node.get(attribute::by_node::encode_any_value(&node, &name));
        let attr = attribute::by_node::decode(&found_encoded);
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

