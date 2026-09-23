use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::*;
use crate::BlobId;
use crate::BlobStore;
use crate::Table;
use crate::blobstore::*;
use crate::graph::attribute::by_node::*;
use crate::graph::attribute::by_attr::*;
use crate::graph::edge::*;
use crate::graph::strings::StringTable;
use super::view::*;
use super::node::*;
use super::strings::StringId;


pub static TREE_NODE_SIZE : usize = 512;


pub struct Graph {
    strings: Arc<Mutex<StringTable>>,
    blobs: Arc<Mutex<BlobStore>>,
    nodes: NodesTable,
    edges_from: EdgesTable,
    edges_to: EdgesTable,
    attributes_by_node: AttributesByNodeTable,
    attributes_by_name: NodesByAttributeTable,
    // NYI bloom filters table
    // NYI index table
    // NYI text ranges table
    next_node_id: AtomicU32
}


impl Graph {
    pub fn new(mut backing_store: Box<dyn Stream>) -> Self {

        let blob_store = Arc::new(Mutex::new(BlobStore::new(backing_store)));
        let mut strings = StringTable::new(blob_store.clone());

        // NYI expose these in a Well Known Strings list of some sort
        let name_stringid = strings.map_to_id(b"Name");
        let type_stringid = strings.map_to_id(b"Type");

        Graph {
            strings: Arc::new(Mutex::new(strings)),
            blobs: blob_store.clone(),
            nodes: NodesTable::new(Table::new(blob_store.clone())),
            edges_from: EdgesTable::new(Table::new(blob_store.clone())),
            edges_to: EdgesTable::new(Table::new(blob_store.clone())),
            attributes_by_node: AttributesByNodeTable::new(Table::new(blob_store.clone())),
            attributes_by_name: NodesByAttributeTable::new(Table::new(blob_store.clone())),
            next_node_id: AtomicU32::new(1),
        }
    }


    pub fn new_from_stream(mut backing_store: Box<dyn crate::blobstore::Stream>) -> Self {
        unimplemented!();
    }


    pub fn get_view<'a>(&'a self) -> GraphView<'a> {
        // NYI lock something so this only happens one at a time
        GraphView::new(
            &self,
            self.strings.clone(),
            &self.nodes,
            &self.edges_from,
            &self.edges_to,
            &self.attributes_by_node,
            &self.attributes_by_name)
    }


    pub fn commit<'a>(&self, view: &GraphView<'a>) {
        
        self.nodes.commit(&view.nodes);
        self.attributes_by_name.commit(&view.nodes_by_attribute);
        self.attributes_by_node.commit(&view.attributes_by_node);
        self.edges_from.commit(&view.edges_from);
        self.edges_to.commit(&view.edges_to);
        //self.strings.commit();

        // NYI write a new blob header with the proper offset for each root node
    }


    pub(crate) fn get_next_node_id(&self) -> NodeId {
        // BUG BUG BUG this is broken if you use new_from_stream
        NodeId::new(self.next_node_id.fetch_add(1, Ordering::Relaxed))
    }
}
