#![allow(unused)]
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::BlobId;
use crate::BlobStore;

use super::node::*;
use super::View;


pub struct BPlusTree<'a, const K: usize> {
    root_id: BlobId,
    loaded_nodes: RefCell<HashMap<BlobId, NodeHandle<K>>>,
    backing_store: &'a mut BlobStore<'a>
}


impl <'a, const K: usize> BPlusTree<'a, K> {

    pub fn new(backing_store: &'a mut BlobStore<'a>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_node = Node::<K>::empty_leaf();
        let root_id = root_node.store(backing_store);

        // Start off with one node
        let mut nodes : HashMap<BlobId, NodeHandle<K>> = HashMap::new();
        nodes.insert(root_id, NodeHandle::new(root_node));

        BPlusTree { root_id, loaded_nodes: RefCell::new(nodes), backing_store }
    }


    pub fn open(store: &mut BlobStore<'a>) -> Self {
        panic!("NYI");
    }


    pub fn get_view(&'a self) -> View<'a, K> {
        View::new(self, self.get_node_link(self.root_id))
    }


    pub fn commit(&self, view: View<'a, K>) {
        panic!("NYI");
    }


    fn get_node_link(&self, id: BlobId) -> NodeLink<K> {
        match self.loaded_nodes.borrow().get(&id) {
            Some(loaded_node) => NodeLink::Loaded(loaded_node.clone()),
            None => NodeLink::Unloaded(id)
        }
    }


    pub(super) fn get_hnode_from_link(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {

        match node_link {
            NodeLink::Loaded(hnode) => hnode.clone(),
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Unloaded(id) => unimplemented!(), // NYI need to load it, or see if loaded_nodes has it
            NodeLink::Empty => panic!("Attempting to get an empty node link"),
        }
    }
}
