use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::PersistedSortedList;
use super::graph::*;
use super::attribute;
use super::node;
use super::node::*;
use super::attribute::{*, by_node::*, by_name::*};
use super::edge::{*, edge_from::*, edge_to::*};
use super::node::*;
use super::strings::*;


pub struct GraphView<'a> {
    based_on: &'a Graph,
    strings: Arc<Mutex<StringTable>>,
    nodes: NodesView<'a>,
    edges_from: EdgesFromView<'a>,
    edges_to: EdgesToView<'a>,
    attributes_by_node: AttributesByNodeTableView<'a>,
    attributes_by_name: AttributesByNameView<'a>,
}


impl <'a> GraphView<'a> {

    pub(crate) fn new(
        based_on: &'a Graph,
        strings: Arc<Mutex<StringTable>>,
        nodes_table: &'a NodesTable,
        edges_from_table: &'a EdgesFromTable,
        edges_to_table: &'a EdgesToTable,
        attributes_by_node_table: &'a AttributesByNodeTable,
        attributes_by_name_table: &'a AttributesByNameTable) -> Self {
        
        GraphView { 
            based_on: based_on,
            strings: strings,
            nodes: nodes_table.get_view(),
            edges_from: edges_from_table.get_view(),
            edges_to: edges_to_table.get_view(),
            attributes_by_node: attributes_by_node_table.get_view(),
            attributes_by_name: attributes_by_name_table.get_view()
        }
    }


    pub fn get_stringid(&self, value: &[u8]) -> StringId {
        let mut s = self.strings.lock().unwrap();
        s.map_to_id(value)
    }


    pub fn get_string(&self, id: &StringId) -> Vec<u8> {
        let mut s = self.strings.lock().unwrap();
        s.get(id)
    }
    
    pub fn create_node(&self) -> NodeId {
        let node = self.based_on.get_next_node_id();
        self.nodes.put(&node);
        node
    }


    pub fn set_attribute(&self, node: NodeId, name: AttributeName, value: StringId) {
        self.attributes_by_node.put(node, name, value);
        self.attributes_by_name.put(name, value, node);
    }


    pub fn set_attribute_str(&self, node: NodeId, name: &[u8], value: &[u8]) {
        let name_id : AttributeName = self.get_stringid(name).into();
        let value_id : StringId = self.get_stringid(value);
        self.attributes_by_node.put(node, name_id, value_id);
        self.attributes_by_name.put(name_id, value_id, node);
    }


    pub fn get_attribute(&self, node: NodeId, name: AttributeName) -> Option<StringId> {
        let attr = self.attributes_by_node.get(node, name);
        if (attr.node == node && attr.name == name) {
            Some(attr.value)
        } else {
            None
        }
    }


    pub fn iter_attributes(&'a self, node: NodeId) -> AttributeByNodeIterator<'a> {
        self.attributes_by_node.iter_attributes(node)
    }


    pub fn iter_nodes_with_attribute(&self, name: AttributeName, value: StringId) -> AttributeByNameValueIterator<'a> {
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
