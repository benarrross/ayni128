use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use super::page::*;
use super::pagehandle::*;


pub(super) struct PageMap<const K: usize> {
    // NYI change u32 to NodeId at some point
    map: HashMap<u32, (BlobId, Arc<Mutex<Page<K>>>)>
}


impl<const K: usize> PageMap<K> {
    pub fn new() -> Self {
        PageMap {
            map: HashMap::new()
        }
    }

    pub fn alloc_id(&self) -> u32 {
        self.map.len() as u32
    }


    pub fn insert(&mut self, blobid: BlobId, node: Page<K>) -> u32 {
        let id = self.alloc_id();
        self.map.insert(id, (blobid, Arc::new(Mutex::new(node))));
        id
    }


    // NYI should this return a MutexGuard? Probably so...
    pub fn get(&self, nodeid: u32) -> Arc<Mutex<Page<K>>> {
        match (self.map.get(&nodeid).map(|t| {&t.1})) {
            Some(node) => node.clone(),
            None => panic!("Asked for a page that doesn't exist")
        }
    }
}
