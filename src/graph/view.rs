use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::Table;
use super::graph::*;
use super::attribute;
use super::node;
use super::node::*;
use super::attribute::{*, by_node::*, by_attr::*};
use super::edge::{*, edge_from::*, edge_to::*};
use super::node::*;
use super::strings::*;


pub struct GraphView<'a> {
    pub(crate) based_on: &'a Graph,
    pub(crate) strings: Arc<Mutex<StringTable>>,
    pub(crate) nodes: NodesView<'a>,
    pub(crate) edges_from: EdgesFromView<'a>,
    pub(crate) edges_to: EdgesToView<'a>,
    pub(crate) attributes_by_node: AttributesByNodeTableView<'a>,
    pub(crate) nodes_by_attribute: NodesByAttributeTableView<'a>,
}


impl <'a> GraphView<'a> {

    pub(crate) fn new(
        based_on: &'a Graph,
        strings: Arc<Mutex<StringTable>>,
        nodes_table: &'a NodesTable,
        edges_from_table: &'a EdgesFromTable,
        edges_to_table: &'a EdgesToTable,
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


    pub fn insert_attribute_str(&self, node: NodeId, name: &[u8], value: &[u8]) {
        // BUG BUG NYI need to delete any other attribute values with the same name
        let name_id : AttributeName = self.insert_string(name).into();
        let value_id : StringId = self.insert_string(value);
        self.attributes_by_node.insert(node, name_id, value_id);
        self.nodes_by_attribute.insert(name_id, value_id, node);
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
        unimplemented!();
    }


    pub fn get_edge(&self, node:NodeId, name: EdgeName) -> Option<Edge> {
        unimplemented!();
    }


    pub fn iter_edges_from(&self, node: NodeId, name: EdgeName) -> EdgeFromIterator<'a> {
        unimplemented!();
    }


    pub fn iter_edges_to(&self, node: NodeId, name: EdgeName) -> EdgeToIterator<'a> {
        unimplemented!();
    }
}
