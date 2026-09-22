use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::Table;
use super::graph::*;
use super::attribute;
use super::node;
use super::node::*;
use super::attribute::{*, by_node::*, by_attr::*};
use super::edge::*;
use super::node::*;
use super::strings::*;


pub struct GraphView<'a> {
    pub(crate) based_on: &'a Graph,
    pub(crate) strings: Arc<Mutex<StringTable>>,
    pub(crate) nodes: NodesView<'a>,
    pub(crate) edges_from: EdgesFromView<'a>,
    pub(crate) edges_to: EdgesFromView<'a>,
    pub(crate) attributes_by_node: AttributesByNodeTableView<'a>,
    pub(crate) nodes_by_attribute: NodesByAttributeTableView<'a>,
}


impl <'a> GraphView<'a> {

    pub(crate) fn new(
        based_on: &'a Graph,
        strings: Arc<Mutex<StringTable>>,
        nodes_table: &'a NodesTable,
        edges_from_table: &'a EdgesTable,
        edges_to_table: &'a EdgesTable,
        attributes_by_node_table: &'a AttributesByNodeTable,
        attributes_by_name_table: &'a NodesByAttributeTable) -> Self {
        
        GraphView { 
            based_on: based_on,
            strings: strings,
            nodes: nodes_table.get_view(),
            edges_from: edges_from_table.get_view(),
            edges_to: edges_to_table.get_view(),
            attributes_by_node: attributes_by_node_table.get_view(),
            nodes_by_attribute: attributes_by_name_table.get_view()
        }
    }


    pub fn insert_string(&self, value: &[u8]) -> StringId {
        let mut s = self.strings.lock().unwrap();
        s.map_to_id(value)
    }


    pub fn get_string(&self, id: &StringId) -> Vec<u8> {
        let mut s = self.strings.lock().unwrap();
        s.get(id)
    }
    
    pub fn insert_node(&self) -> NodeId {
        let node = self.based_on.get_next_node_id();
        self.nodes.put(&node);
        node
    }


    pub fn insert_attribute(&self, node: NodeId, name: AttributeName, value: StringId) {
        // BUG BUG NYI need to delete any other attribute values with the same name
        self.attributes_by_node.insert(node, name, value);
        self.nodes_by_attribute.insert(name, value, node);
    }


    pub fn get_attribute(&self, node: NodeId, name: AttributeName) -> Option<StringId> {
        if let Some(attr) = self.attributes_by_node.get(node, name) {
            Some(attr.value)
        } else {
            None
        }
    }


    pub fn iter_attributes(&'a self, node: NodeId) -> AttributeByNodeIterator<'a> {
        self.attributes_by_node.iter(node)
    }


    pub fn iter_nodes_with_attribute(&'a self, name: AttributeName, value: StringId) -> NodeByAttributeIterator<'a> {
        self.nodes_by_attribute.iter(name, value)
    }


    pub fn insert_edge(&self, from: NodeId, to: NodeId, name: EdgeName, edge_type: EdgeType, order: EdgeOrder ) {
        self.edges_from.insert(from, edge_type, name, to, order);
        self.edges_to.insert(to, edge_type, name, from, order);
    }


    pub fn get_edge_from(&self, from: NodeId, edge_type: EdgeType, name: EdgeName) -> Option<Edge> {
        self.edges_from.get_by_name(from, edge_type, name)
    }


    pub fn get_edge_to(&self, to: NodeId, edge_type: EdgeType, name: EdgeName) -> Option<Edge> {
        self.edges_to.get_by_name(to, edge_type, name)
    }


    pub fn get_parent_edge(&self, to: NodeId) -> Option<Edge> {
        self.edges_to.get_by_type(to, EdgeType::Child)
    }


    pub fn iter_edges_from(&'a self, from: NodeId, edge_type: Option<EdgeType>, name: Option<EdgeName>) -> EdgeIterator<'a> {
        self.edges_from.iter(from, edge_type, name)
    }


    pub fn iter_edges_to(&'a self, to: NodeId, edge_type: Option<EdgeType>, name: Option<EdgeName>) -> EdgeIterator<'a> {
        self.edges_to.iter(to, edge_type, name)
    }
}
