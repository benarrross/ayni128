use std::sync::{Arc, RwLock};
use crate::{blobstore::BlobId};
use super::table::*;
use super::pagehandle::*;


#[derive(Debug, Clone)]
pub(super) enum PageLinkKind<const K: usize> {
    /// Link to no node
    Empty,

    /// Link to a node that hasn't been loaded from storage yet
    Unloaded(BlobId),

    /// Link to a node that has not been modified in the current view
    Loaded(PageHandle<K>),

    /// Link to a node that has been modified in the current view
    Mutable(PageHandle<K>)
}


#[derive(Debug)]
pub(super) struct PageLink<const K:usize> {
    pub label: String,
    pub inner: RwLock<PageLinkKind<K>>  // NYI would a Mutex be faster?
}


impl<const K: usize> Clone for PageLink<K> {
    fn clone(&self) -> Self {
        PageLink { 
            inner: RwLock::new(self.inner.read().unwrap().clone()),
            label: self.label.clone()
        }
    }
}


/// Link to a node that may be loaded, or may still be on disk.
impl<const K: usize> PageLink<K> {
    
    pub fn new_empty() -> Self {
        PageLink { 
            inner: RwLock::new(PageLinkKind::Empty),
            label: format!("empty")
        }
    }


    pub(super) fn new_loaded(value: &PageHandle<K>) -> Self {
        PageLink { 
            inner: RwLock::new(PageLinkKind::Loaded(value.clone())),
            label: format!("immutable {}", &value.node_debug_id)
        }
    }


    pub(super) fn new_mutable(value: &PageHandle<K>) -> Self {
        PageLink { 
            inner: RwLock::new(PageLinkKind::Mutable(value.clone())),
            label: format!("mutable {}", &value.node_debug_id)
        }
    }


    pub(super) fn set_mutable(&self, value: &PageHandle<K>) {
        *self.inner.write().unwrap() = PageLinkKind::Mutable(value.clone());
    }


    pub(super) fn new_unloaded(value: BlobId) -> Self {
        PageLink {
            inner: RwLock::new(PageLinkKind::Unloaded(value)),
            label: format!("unloaded {}", value)
        }
    }


    pub(super) fn set_unloaded(&self, blobid: BlobId) {
        *self.inner.write().unwrap() = PageLinkKind::Unloaded(blobid);
    }
 

    pub(super) fn is_empty(&self) -> bool {
        let read_lock = self.inner.read().unwrap();
        matches!(*read_lock, PageLinkKind::Empty )
    }


    pub(super) fn is_mutable(&self) -> bool {
        let read_lock = self.inner.read().unwrap();
        matches!(*read_lock, PageLinkKind::Mutable(_) )
    }


    pub(super) fn get_blobid(&self) -> BlobId {
        match &*self.inner.read().unwrap() {
            PageLinkKind::Unloaded(blobid) => {
                *blobid
            },
            PageLinkKind::Loaded(hnode) => {
                unimplemented!()
                //hnode.read_lock().blobid.unwrap()
            },
            PageLinkKind::Mutable(hnode) => panic!(),
            PageLinkKind::Empty => BlobId::new_empty()
        }
    }
}
