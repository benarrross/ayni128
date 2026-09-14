use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::PersistedSortedList;
use super::graph::*;
use super::attribute;
use super::node;
use super::node::*;
use super::attribute::{by_node::*, by_name::*};
use super::edge::{*, edge_from::*, edge_to::*};


pub struct GraphView<'a> {
    based_on: &'a Graph,
    nodes: NodesView<'a>,
    edges_from: EdgesFromView<'a>,
    edges_to: EdgesToView<'a>,
    attributes_by_node: AttributesByNodeView<'a>,
    attributes_by_name: AttributesByNameView<'a>,
}


impl <'a> GraphView<'a> {

    pub(crate) fn new(
        based_on: &'a Graph, 
        nodes_table: &'a NodesTable,
        edges_from_table: &'a EdgesFromTable,
        edges_to_table: &'a EdgesToTable,
        attributes_by_node_table: &'a AttributesByNodeTable,
        attributes_by_name_table: &'a AttributesByNameTable) -> Self {
        
        GraphView { 
            based_on: based_on,
            nodes: nodes_table.get_view(),
            edges_from: edges_from_table.get_view(),
            edges_to: edges_to_table.get_view(),
            attributes_by_node: attributes_by_node_table.get_view(),
            attributes_by_name: attributes_by_name_table.get_view()
        }
    }

    
    pub fn create_node(&self) -> NodeId {
        let node = self.based_on.get_next_node_id();
        self.nodes.put(&node);
        node
    }


    pub fn set_attribute(&self, node: NodeId, name: AttributeName, value: StringId) {
        self.attributes_by_node.put(&node, &name, &value);
        self.attributes_by_name.put(&name, &value, &node);
    }


    pub fn get_attribute(&self, node: NodeId, name: AttributeName) -> Option<StringId> {
        let attr = self.attributes_by_node.get(&node, &name);
        if (attr.node == node && attr.name == name) {
            Some(attr.value)
        } else {
            None
        }
    }


    pub fn iter_attributes(&self, node: NodeId) -> AttributeByNodeIterator<'a> {
        unimplemented!();
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
