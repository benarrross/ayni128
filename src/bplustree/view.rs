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
    root_node_link: RefCell<NodeLink<K>>,
    puts: RefCell<SortedArray<u128>>,
    deletes: RefCell<SortedArray<u128>>
}


impl<'a, const K: usize> View<'a, K> {

    pub fn new(based_on: &'a BPlusTree<'a, K>, root_node_link: NodeLink<K>) -> Self {

        View { 
            based_on, 
            root_node_link: RefCell::new(root_node_link.clone()),
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
        let mut mutable_hnode_root = self.get_mutable_hnode(&mut self.root_node_link.borrow_mut());
        let updated_root = Node::put(& mut mutable_hnode_root, value); 
        if let Some(new_hnode_root) = updated_root {
            *self.root_node_link.borrow_mut() = NodeLink::Edited(new_hnode_root);
        }
    }


    pub fn delete(&self, value : u128) {
        panic!("NYI");
    }


    pub fn get(&self, value : u128) -> u128 {
        let root_hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        root_hnode.read_lock().get(value)
    }


    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        let root_hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        RangeCollection::new(self, root_hnode, min, mac)
    }   


    fn get_hnode(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {
        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode(node_link.clone());
                *node_link = NodeLink::Loaded(hnode.clone());
                hnode
            },
            NodeLink::Loaded(hnode) => hnode.clone(),
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("NYI")
        }
    }

    /// Gets a mutable version of the node and updates the passed in link if necesssary
    fn get_mutable_hnode(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {

        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode(node_link.clone());
                let node_copy = hnode.read_lock().clone();
                let new_hnode = NodeHandle::new(node_copy);
                *node_link = NodeLink::Edited(new_hnode.clone());
                new_hnode
            },
            NodeLink::Loaded(hnode) => {
                let node_copy = hnode.read_lock().clone();
                let new_hnode = NodeHandle::new(node_copy);
                *node_link = NodeLink::Edited(new_hnode.clone());
                new_hnode
            },
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("Can't make an empty node link mutable")
        }
    }
}

