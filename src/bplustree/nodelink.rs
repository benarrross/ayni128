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


#[derive(Debug, Clone)]
enum NodeLinkKind<const K: usize> {
    /// Link to no node
    Empty,

    /// Link to a node that hasn't been loaded from storage yet
    Unloaded(BlobId),

    /// Link to a node that has not been modified in the current view
    Immutable(NodeHandle<K>),

    /// Link to a node that has been modified in the current view
    Mutable(NodeHandle<K>)
}


#[derive(Debug)]
pub struct NodeLink<const K:usize> {
    inner: RwLock<NodeLinkKind<K>>  // NYI would a Mutex be faster?
}


impl<const K: usize> Clone for NodeLink<K> {
    fn clone(&self) -> Self {
        NodeLink { 
            inner: RwLock::new(self.inner.read().unwrap().clone())
        }
    }
}


/// Link to a node that may be loaded, or may still be on disk.
impl<const K: usize> NodeLink<K> {
    
    pub fn empty() -> Self {
        NodeLink { inner: RwLock::new(NodeLinkKind::Empty) }
    }

    pub fn immutable(value: NodeHandle<K>) -> Self {
        NodeLink { inner: RwLock::new(NodeLinkKind::Immutable(value)) }
    }

    pub fn mutable(value: NodeHandle<K>) -> Self {
        NodeLink { inner: RwLock::new(NodeLinkKind::Mutable(value)) }
    }

    pub fn unloaded(value: BlobId) -> Self {
        NodeLink { inner: RwLock::new(NodeLinkKind::Unloaded(value)) }
    }

    pub fn is_empty(&self) -> bool {
        let read_lock = self.inner.read().unwrap();
        matches!(*read_lock, NodeLinkKind::Empty )
    }


    /// Gets a node handle from a link, loading the node from storage if necessary.
    pub fn get_immutable(&self, based_on: &BPlusTree<'_, K>) -> NodeHandle<K> {

        let mut new_inner = NodeLinkKind::Empty;

        let loaded_hnode = match &*self.inner.read().unwrap() {
            NodeLinkKind::Unloaded(id) => {
                let hnode = based_on.load_node(&self);
                new_inner = NodeLinkKind::Mutable(hnode.clone());
                hnode
            },
            NodeLinkKind::Immutable(hnode) => hnode.clone(),
            NodeLinkKind::Mutable(hnode) => hnode.clone(),
            NodeLinkKind::Empty => panic!("Can't get an empty node link")
        };

        if !matches!(&new_inner, NodeLinkKind::Empty) {
            *self.inner.write().unwrap() = new_inner;
        }

        loaded_hnode
    }


    /// Gets a mutable node handle from a link, loading the node from storage if necessary.
    /// This should ONLY be used by views when editing the tree.
    pub fn get_mutable(&self, based_on: &BPlusTree<'_, K>) -> NodeHandle<K> {

        let mut new_inner = NodeLinkKind::Empty;

        let loaded_hnode = match &*self.inner.read().unwrap() {
            NodeLinkKind::Unloaded(id) => {
                let loaded_hnode = based_on.load_node(&self);
                let mutable_node = loaded_hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                new_inner = NodeLinkKind::Mutable(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLinkKind::Immutable(hnode) => {
                let mutable_node = hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                new_inner = NodeLinkKind::Mutable(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLinkKind::Mutable(hnode) => hnode.clone(),
            NodeLinkKind::Empty => panic!("Can't get an empty node link")
        };

        if !matches!(&new_inner, NodeLinkKind::Empty) {
            // NYI need to make sure nobody else set this between the release of the read lock and
            // aquisition of the write lock
            *self.inner.write().unwrap() = new_inner;
        }
        
        loaded_hnode
    }
    
}
