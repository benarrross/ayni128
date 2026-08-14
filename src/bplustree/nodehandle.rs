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
pub struct NodeHandle<const K: usize>(Arc<RwLock<Node<K>>>);


impl<const K: usize> NodeHandle<K> {
    
    pub fn new(inner: Node<K>) -> Self {
        NodeHandle(Arc::new(RwLock::new(inner)))
    }

    pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, Node<K>> {
        self.0.read().unwrap()
    }

    pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, Node<K>> {
        self.0.write().unwrap()
    }
}


