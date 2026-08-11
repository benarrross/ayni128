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
    blobs: HashMap<BlobId, NodeHandle<K>>,
    backing_store: &'a mut BlobStore<'a>
}


impl <'a, const K: usize> BPlusTree<'a, K> {

    pub fn new(backing_store: &'a mut BlobStore<'a>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_node = Node::<K>::new_leaf();
        let root_id = root_node.store(backing_store);
        let mut blobs : HashMap<BlobId, NodeHandle<K>> = HashMap::new();
        blobs.insert(root_id, NodeHandle::new(root_node));

        BPlusTree { root_id, blobs, backing_store }
    }


    pub fn open(store: &mut BlobStore<'a>) -> Self {
        panic!("NYI");
    }


    pub fn get_view(&'a self) -> View<'a, K> {

        let root_blob = self.get_blob_link(self.root_id);
        View::new(self, &root_blob)
    }


    pub fn commit(&mut self, view: View<'a, K>) {
        panic!("NYI");
    }


    fn get_blob_link(&self, id: BlobId) -> NodeLink<K> {

        // NYI this is going to need to mutate itself when loading blobs
        match self.blobs.get(&id) {
            Some(loaded_node) => NodeLink::Loaded(loaded_node.clone()),
            None => NodeLink::Unloaded(id)
        }
    }
}
