use std::collections::HashMap;
use crate::BlobId;
use super::node::*;
use super::nodehandle::*;


// NYI rename to PageMap
pub(super) struct NodeMap<const K: usize> {
    // NYI change u32 to NodeId at some point
    map: HashMap<u32, (BlobId, Node<K>)>
}


impl<const K: usize> NodeMap<K> {
    pub fn new() -> Self {
        NodeMap {
            map: HashMap::new()
        }
    }

    pub fn alloc_id(&self) -> u32 {
        self.map.len() as u32
    }


    pub fn insert(&mut self, blobid: BlobId, node: Node<K>) -> u32 {
        let id = self.alloc_id();
        self.map.insert(id, (blobid, node));
        id
    }

    pub fn get(&self, nodeid: u32) -> &Node<K> {
        match (self.map.get(&nodeid).map(|t| {&t.1})) {
            Some(node) => node,
            None => panic!("Asked for a page that doesn't exist")
        }
    }
}
