use std::sync::{Arc, RwLock};
use crate::{blobstore::BlobId};
use super::table::*;
use super::nodehandle::*;


#[derive(Debug, Clone)]
pub(super) enum NodeLinkKind<const K: usize> {
    /// Link to no node
    Empty,

    /// Link to a node that hasn't been loaded from storage yet
    Unloaded(BlobId),

    /// Link to a node that has not been modified in the current view
    Loaded(NodeHandle<K>),

    /// Link to a node that has been modified in the current view
    Mutable(NodeHandle<K>)
}


#[derive(Debug)]
pub(super) struct NodeLink<const K:usize> {
    pub label: String,
    pub inner: RwLock<NodeLinkKind<K>>  // NYI would a Mutex be faster?
}


impl<const K: usize> Clone for NodeLink<K> {
    fn clone(&self) -> Self {
        NodeLink { 
            inner: RwLock::new(self.inner.read().unwrap().clone()),
            label: self.label.clone()
        }
    }
}


/// Link to a node that may be loaded, or may still be on disk.
impl<const K: usize> NodeLink<K> {
    
    pub fn new_empty() -> Self {
        NodeLink { 
            inner: RwLock::new(NodeLinkKind::Empty),
            label: format!("empty")
        }
    }


    pub(super) fn new_loaded(value: &NodeHandle<K>) -> Self {
        NodeLink { 
            inner: RwLock::new(NodeLinkKind::Loaded(value.clone())),
            label: format!("immutable {}", &value.node_debug_id)
        }
    }


    pub(super) fn new_mutable(value: &NodeHandle<K>) -> Self {
        NodeLink { 
            inner: RwLock::new(NodeLinkKind::Mutable(value.clone())),
            label: format!("mutable {}", &value.node_debug_id)
        }
    }


    pub(super) fn set_mutable(&self, value: &NodeHandle<K>) {
        *self.inner.write().unwrap() = NodeLinkKind::Mutable(value.clone());
    }


    pub(super) fn new_unloaded(value: BlobId) -> Self {
        NodeLink {
            inner: RwLock::new(NodeLinkKind::Unloaded(value)),
            label: format!("unloaded {}", value)
        }
    }


    pub(super) fn set_unloaded(&self, blobid: BlobId) {
        *self.inner.write().unwrap() = NodeLinkKind::Unloaded(blobid);
    }
 

    pub(super) fn is_empty(&self) -> bool {
        let read_lock = self.inner.read().unwrap();
        matches!(*read_lock, NodeLinkKind::Empty )
    }


    pub(super) fn is_mutable(&self) -> bool {
        let read_lock = self.inner.read().unwrap();
        matches!(*read_lock, NodeLinkKind::Mutable(_) )
    }


    pub(super) fn get_blobid(&self) -> BlobId {
        match &*self.inner.read().unwrap() {
            NodeLinkKind::Unloaded(blobid) => {
                *blobid
            },
            NodeLinkKind::Loaded(hnode) => {
                unimplemented!()
                //hnode.read_lock().blobid.unwrap()
            },
            NodeLinkKind::Mutable(hnode) => panic!(),
            NodeLinkKind::Empty => BlobId::new_empty()
        }
    }
}
