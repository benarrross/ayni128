#![allow(unused)]
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::BlobId;
use crate::BlobStore;

use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;
use super::View;


pub struct BPlusTree<'a, const K: usize> {
    root_id: BlobId,
    loaded_hnodes: RefCell<HashMap<BlobId, NodeHandle<K>>>,
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

        BPlusTree { root_id, loaded_hnodes: RefCell::new(nodes), backing_store }
    }


    pub fn open(store: &mut BlobStore<'a>) -> Self {
        panic!("NYI");
    }


    pub fn get_view(&'a self) -> View<'a, K> {
        View::new(self, self.get_node_link_outer(self.root_id))
    }


    pub fn commit(&self, view: View<'a, K>) {
        panic!("NYI");
    }

    // NYI rename to load_node?
    fn get_node_link_outer(&self, id: BlobId) -> NodeLinkOuter<K> {
        match self.loaded_hnodes.borrow().get(&id) {
            Some(loaded_hnode) => NodeLinkOuter::loaded(loaded_hnode.clone()),
            None => NodeLinkOuter::blobid(id) // NYI need to actually load the node
        }
    }


    pub(super) fn load_node(&self, node_link_outer: &NodeLinkOuter<K>) -> NodeHandle<K> {
        unimplemented!()
    }

}
