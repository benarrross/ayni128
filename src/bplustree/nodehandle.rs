#![allow(unused)]
use std::cell::RefCell;
use std::fmt;
use std::mem;
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::bplustree::node::SplitResult::NoSplit;
use crate::{blobstore::*, sortedarray::*};

use super::bplustree::*;
use super::node::*;


#[derive(Debug, Clone)]
pub struct NodeHandle<const K: usize> {
    node_debug_id: u32,
    node_lock: Arc<RwLock<Node<K>>> 
}


impl<const K: usize> NodeHandle<K> {
    
    pub fn new(node: Node<K>) -> Self {

        NodeHandle {
            node_debug_id: node.debug_id,
            node_lock: Arc::new(RwLock::new(node))
        }
    }

    pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, Node<K>> {
        self.node_lock.read().unwrap()
    }

    pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, Node<K>> {
        self.node_lock.write().unwrap()
    }
}


