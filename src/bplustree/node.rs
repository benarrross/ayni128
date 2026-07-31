#![allow(unused)]
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::{blobstore::*, sortedarray::*};

use super::bplustree::*;


// Refcounted pointer to a node that is loaded into memory
pub type LoadedNodeRef<const K:usize> = Arc<RwLock<Node<K>>>;


// Node that may or may not be loaded into memory yet
#[derive(Debug, Clone)]
pub enum NodeLink<const K: usize> {
    Empty,
    Unloaded(BlobId),
    Loaded(LoadedNodeRef<K>),
    Mutable(LoadedNodeRef<K>)
}


#[derive(Debug, Clone)]
pub struct Node<const K: usize> {
    pub values : SortedArray<u128>,
    children: Option<Vec<NodeLink<K>>>,
    next : NodeLink<K>
}


impl<const K: usize> Node<K> {  

    pub fn new_leaf() -> Self {
        Node { 
            values: SortedArray::new(),
            children: None,
            next: NodeLink::Empty }
    }


    pub fn store(&self, backing_store: &mut BlobStore) -> BlobId {
        
        let mut serialized_node = MemoryStream::new();

        // Serialize the lengths of values and children
        serialized_node.write_all(&self.values.len().to_le_bytes());
        
        match (&self.children) {
            Some(children) => {
                serialized_node.write_all(&children.len().to_le_bytes());                
            },
            None => {
                let zero : usize = 0;
                serialized_node.write_all(&zero.to_le_bytes());
            }
        }
        
        // Serialize the values
        // NYI

        // Serialize the children if we have any
        // NYI

        // Store the serialized node in a blob and return the blobid
        backing_store.put(serialized_node.as_slice())
    }



    pub fn is_leaf(&self) -> bool { self.children.is_none() }


    pub fn put(&mut self, value : u128) {

        if self.is_leaf() {
            self.values.insert(value);
        }
        else {
            unimplemented!();
        }
    }

    pub fn get(&self, value : u128) -> u128 {
        unimplemented!();
    }
}