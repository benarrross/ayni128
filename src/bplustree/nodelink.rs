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
use super::nodehandle::*;


// Node that may or may not be loaded into memory yet
#[derive(Debug, Clone)]
pub enum NodeLinkInner<const K: usize> {
    Empty,
    Unloaded(BlobId),
    Loaded(NodeHandle<K>),
    Edited(NodeHandle<K>)
}


#[derive(Debug)]
pub struct NodeLinkOuter<const K:usize> {
    inner: RwLock<NodeLinkInner<K>>
}

impl<const K: usize> Clone for NodeLinkOuter<K> {
    fn clone(&self) -> Self {
        NodeLinkOuter { 
            inner: RwLock::new(self.inner.read().unwrap().clone())
        }
    }
}


impl<const K: usize> NodeLinkOuter<K> {
    
    pub fn new() -> Self {
        NodeLinkOuter { inner: RwLock::new(NodeLinkInner::Empty) }
    }

    pub fn loaded(value: NodeHandle<K>) -> Self {
        NodeLinkOuter { inner: RwLock::new(NodeLinkInner::Loaded(value)) }
    }

    pub fn edited(value: NodeHandle<K>) -> Self {
        NodeLinkOuter { inner: RwLock::new(NodeLinkInner::Edited(value)) }
    }

    pub fn blobid(value: BlobId) -> Self {
        NodeLinkOuter { inner: RwLock::new(NodeLinkInner::Unloaded(value)) }
    }

    //pub fn inner_deprecate_x(&self) -> NodeLinkInner<K> { self.inner.borrow().clone() }
    // pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, Node<K>> {
    //     self.0.read().unwrap()
    // }

    // pub fn write_lock(&self) -> std::sync::RwLockWriteGuard<'_, Node<K>> {
    //     self.0.write().unwrap()
    // }

    /// Gets a node handle from a link, loading the node from storage if necessary.
    pub fn get(&self, based_on: &BPlusTree<'_, K>) -> NodeHandle<K> {

        let mut new_inner = NodeLinkInner::Empty;

        let loaded_hnode = match &*self.inner.read().unwrap() {
            NodeLinkInner::Unloaded(id) => {
                let hnode = based_on.load_node(&self);
                new_inner = NodeLinkInner::Edited(hnode.clone());
                hnode
            },
            NodeLinkInner::Loaded(hnode) => hnode.clone(),
            NodeLinkInner::Edited(hnode) => hnode.clone(),
            NodeLinkInner::Empty => panic!("Can't get an empty node link")
        };

        if matches!(&new_inner, NodeLinkInner::Unloaded(new_inner)) {
            *self.inner.write().unwrap() = new_inner;
        }

        loaded_hnode
    }


    /// Gets a mutable node handle from a link, loading the node from storage if necessary.
    /// This should ONLY be used by views when editing the tree.
    pub fn get_mutable(&self, based_on: &BPlusTree<'_, K>) -> NodeHandle<K> {

        let mut new_inner = NodeLinkInner::Empty;

        let loaded_hnode = match &*self.inner.read().unwrap() {
            NodeLinkInner::Unloaded(id) => {
                let loaded_hnode = based_on.load_node(&self);
                let mutable_node = loaded_hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                new_inner = NodeLinkInner::Edited(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLinkInner::Loaded(hnode) => {
                let mutable_node = hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                new_inner = NodeLinkInner::Edited(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLinkInner::Edited(hnode) => hnode.clone(),
            NodeLinkInner::Empty => panic!("Can't get an empty node link")
        };

        if matches!(&new_inner, NodeLinkInner::Unloaded(new_inner)) {
            *self.inner.write().unwrap() = new_inner;
        } else if matches!(&new_inner, NodeLinkInner::Loaded(new_inner)) {
            *self.inner.write().unwrap() = new_inner;
        }
        
        loaded_hnode
    }
    
}
