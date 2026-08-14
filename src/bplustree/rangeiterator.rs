#![allow(unused)]
use std::iter::Map;
use std::io::{SeekFrom, prelude::*};
use std::iter::{*};
use std::sync::{Arc, RwLock};
use std::rc::Rc;

use crate::BlobId;

use super::bplustree::*;
use super::view::*;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;


pub struct RangeCollection<'a, const K: usize> {
    based_on: &'a View<'a, K>,
    root_node: NodeHandle<K>,
    min: u128,
    mac: u128
}


impl<'a, const K: usize> RangeCollection<'a,  K> {

    pub fn new(based_on: &'a View<'a, K>, root_node: NodeHandle<K>, min: u128, mac: u128) -> Self {
        RangeCollection { 
            based_on, 
            root_node: root_node, 
            min: min, 
            mac: mac  }
    }
}


impl<'a, const K: usize> IntoIterator for RangeCollection<'a, K> {
    type Item = u128;
    type IntoIter = RangeIterator<'a, K>;
    fn into_iter(self) -> Self::IntoIter { RangeIterator::new(self.based_on, self.root_node, self.min, self.mac) }
}


pub struct RangeIterator<'a, const K: usize> {
    based_on_view: &'a View<'a, K>,
    root_hnode: NodeHandle<K>,
    min: u128,
    mac: u128,
    current_hnode: Option<NodeHandle<K>>,
    current_index: usize
}


impl<'a, const K: usize> RangeIterator<'a,  K> {

    pub fn new(based_on_view: &'a View<'a, K>, root_node: NodeHandle<K>, min: u128, mac: u128) -> Self {
        RangeIterator { 
            based_on_view, 
            root_hnode: root_node.clone(), 
            min: min, 
            mac: mac,
            current_hnode: None,
            current_index: 0  }
    }


    fn find_first(&mut self) -> Option<u128> {

        // Find the leaf node
        let mut hnode = self.root_hnode.clone();
        loop {
            let hnode_cur = hnode.clone();
            let node_read_lock = hnode_cur.read_lock();
            if (node_read_lock.is_leaf()) {
                break;
            }

            let child_index = node_read_lock.values.find_range_index(self.min);
            hnode = self.based_on_view.get_child_hnode(&node_read_lock, child_index);
        }

        // The leaf node we are pointing at might be the one before the one we want, if the caller asks for a value 
        // between two leaf nodes. If this is the case, advance to the next one.
        let mut go_to_next_leaf = false;
        let mut index = 0;
        {
            let leaf_node_read_lock = hnode.read_lock();
            index = leaf_node_read_lock.values.find_index(self.min);
            if (index > leaf_node_read_lock.values.len()) {
                go_to_next_leaf = true;
            }
        }
        if (go_to_next_leaf) {
            hnode = match self.based_on_view.get_next_hnode(&hnode) {
                Some(hnode_next) => hnode_next,
                None => { return Option::None; }
            };
            index = 0;
        }

        // Now that we have the correct node and index, start enumerating
        self.current_hnode = Option::Some(hnode.clone());
        self.current_index = index;
        Option::Some(hnode.read_lock().values[index])
    }


    fn find_next(&mut self) -> Option<u128> {

        // Advance to the next value in this node
        self.current_index += 1;

        // Advance to the next leaf node if necessary
        let mut current_hnode = self.current_hnode.as_ref().unwrap().clone();
        let mut current_node_read_lock = current_hnode.read_lock();
        if self.current_index >= current_node_read_lock.values.len() {
            self.current_hnode = self.based_on_view.get_next_hnode(&current_hnode); // BUG this tries to take a write lock while we have a read lock
            self.current_index = 0;

            let current_node = match &self.current_hnode { 
                Some(current_hnode_unwrapped) => current_hnode_unwrapped.read_lock(),
                None => { return None; }
            };
        }

        if self.current_index < current_node_read_lock.values.len() {
            Option::Some(current_node_read_lock.values[self.current_index])
        }
        else {
            Option::None
        }
    }
}


impl<'a, const K: usize> Iterator for RangeIterator<'a, K> {

    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {

         match &self.current_hnode {
            None => self.find_first(),
            Some(node) => self.find_next()
        }
    }
}

