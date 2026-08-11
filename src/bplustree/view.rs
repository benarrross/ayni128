use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{SeekFrom, prelude::*};
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use std::mem;

use crate::BlobId;
use crate::sortedarray::*;

use super::bplustree::*;
use super::node::*;
use super::rangeiterator::{*};


pub struct View<'a, const K: usize> {
    based_on: &'a BPlusTree<'a, K>,
    root_hnode: RefCell<NodeHandle<K>>,
    puts: RefCell<SortedArray<u128>>,
    deletes: RefCell<SortedArray<u128>>
}


impl<'a, const K: usize> View<'a, K> {

    pub fn new(based_on: &'a BPlusTree<'a, K>, root_link: &NodeLink<K>) -> Self {

        let root = match root_link {
            NodeLink::Loaded(node) => node,
            NodeLink::Unloaded(id) => unimplemented!(),
            NodeLink::Dirty(_) => panic!("Root node should not be mutable"),
            NodeLink::Empty => panic!("Root node should not be empty"),
        };
        let based_on_root = &*root.read_lock();
        let mut mutable_root = based_on_root.clone();

        View { 
            based_on, 
            root_hnode: RefCell::new(NodeHandle::new(mutable_root)),
            puts: RefCell::new(SortedArray::new()),
            deletes: RefCell::new(SortedArray::new())
        }
    }


    pub fn put(& self, value : u128) {

        // Update our list of added/deleted values
        match self.deletes.borrow().get(value) {
            Some(value) => self.deletes.borrow_mut().remove(value),
            None => self.puts.borrow_mut().insert(value)
        };

        // Update our b+tree and store the new root if necessary
        let new_root = Node::put(&mut self.root_hnode.borrow_mut(), value); 
        if let Some(new_hnode_root) = new_root {
            *self.root_hnode.borrow_mut() = new_hnode_root;
        }
    }


    pub fn delete(&self, value : u128) {
        panic!("NYI");
    }


    pub fn get(&self, value : u128) -> u128 {
        self.root_hnode.borrow().read_lock().get(value)
    }


    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        RangeCollection::new(self, self.root_hnode.borrow().clone(), min, mac)
    }   


    fn get_mutable_node(&mut self, id: BlobId) -> &mut Node<K> {

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

