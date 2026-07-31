use std::collections::HashMap;
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;
use std::sync::{Arc, RwLock};

use crate::BlobId;
use crate::sortedarray::*;

use super::bplustree::*;
use super::node::*;
use super::rangeiterator::{*};


pub struct View<'a, const K: usize> {
    based_on: &'a BPlusTree<'a, K>,
    root: LoadedNodeRef<K>, // Assumed to be mutable
    puts: SortedArray<u128>,
    deletes: SortedArray<u128>
}


impl<'a, const K: usize> View<'a, K> {

    pub fn new(based_on: &'a BPlusTree<'a, K>, root_link: &NodeLink<K>) -> Self {

        let root = match root_link {
            NodeLink::Loaded(node) => node,
            NodeLink::Unloaded(id) => unimplemented!(),
            NodeLink::Mutable(_) => panic!("Root node should not be mutable"),
            NodeLink::Empty => panic!("Root node should not be empty"),
        };
        let based_on_root = &*root.read().unwrap();
        let mut mutable_root = based_on_root.clone();

        View { 
            based_on, 
            root: Arc::new(RwLock::new(mutable_root)),
            puts: SortedArray::new(),
            deletes: SortedArray::new()
        }
    }


    pub fn put(&mut self, value : u128) {

        // NYI put should return a "I split or not" enum
        self.root.write().unwrap().put(value);
    }


    pub fn delete(&mut self, value : u128) {
        panic!("NYI");
    }


    pub fn get(&self, value : u128) -> u128 {
        self.root.read().unwrap().get(value)
    }


    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        RangeCollection::new(self, self.root.clone(), min, mac)
    }   


    fn get_editable_node(&mut self, id: BlobId) -> &mut Node<K> {

        unimplemented!();
        // match local_node {
        //     Some(mut node) => {
        //         let z: &mut Arc<Node<K>> = local_node.unwrap();
        //         //let x = Arc::get_mut(node);
        //         return z;
        //     },
        //     None => {}
        // }
        //panic!("NYI");
    }
}
