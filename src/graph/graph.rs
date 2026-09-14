use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::*;
use crate::BlobId;
use crate::BlobStore;
use crate::PersistedSortedList;
use crate::blobstore::*;
use crate::graph::attribute::by_node::*;
use crate::graph::attribute::by_name::*;
use crate::graph::edge::edge_from::*;
use crate::graph::edge::edge_to::*;
use super::view::*;
use super::node::NodesTable;


pub static TREE_NODE_SIZE : usize = 512;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeId (pub u32); // implement from trait instead of making this public?

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StringId (pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeName (pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeName (pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeOrder (pub u32);


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EdgeType {
    Child = 0,
    Reference = 1,
}

pub struct Graph {
    blobs: Arc<Mutex<BlobStore>>,
    nodes: NodesTable,
    edges_from: EdgesFromTable,
    edges_to: EdgesToTable,
    attributes_by_node: AttributesByNodeTable,
    attributes_by_name: AttributesByNameTable,
    // NYI bloom filters table
    // NYI strings table
    next_node_id: AtomicU32
}


impl Graph {
    pub fn new(mut backing_store: Box<dyn Stream>) -> Self {

        let blobstore = Arc::new(Mutex::new(BlobStore::new(backing_store)));
        Graph {
            blobs: blobstore.clone(),
            nodes: NodesTable::new(PersistedSortedList::new(blobstore.clone())),
            edges_from: EdgesFromTable::new(PersistedSortedList::new(blobstore.clone())),
            edges_to: EdgesToTable::new(PersistedSortedList::new(blobstore.clone())),
            attributes_by_node: AttributesByNodeTable::new(PersistedSortedList::new(blobstore.clone())),
            attributes_by_name: AttributesByNameTable::new(PersistedSortedList::new(blobstore.clone())),
            next_node_id: AtomicU32::new(1),
        }
    }


    pub fn open(mut backing_store: Box<dyn crate::blobstore::Stream>) -> Self {
        unimplemented!();
    }


    pub fn get_view<'a>(&'a self) -> GraphView<'a> {
        // NYI lock something so this only happens one at a time
        GraphView::new(
            &self,
            &self.nodes,
            &self.edges_from,
            &self.edges_to,
            &self.attributes_by_node,
            &self.attributes_by_name)
    }


    pub fn commit<'a>(&self, view: &GraphView<'a>) {
        unimplemented!();
    }

    pub(crate) fn get_next_node_id(&self) -> NodeId {
        NodeId(self.next_node_id.fetch_add(1, Ordering::Relaxed))
    }
}


