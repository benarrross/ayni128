#![allow(unused)]
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::mem;

use crate::bplustree::node::SplitResult::NoSplit;
use crate::{blobstore::*, sortedarray::*};

use super::bplustree::*;


// Refcounted pointer to a node that is loaded into memory
#[derive(Debug, Clone)]
pub struct NodeHandle<const K: usize>(Arc<RwLock<Node<K>>>);


impl<const K: usize> NodeHandle<K> {
    
    pub fn new(inner: Node<K>) -> Self {
        NodeHandle(Arc::new(RwLock::new(inner)))
    }

    pub fn read_lock(&self) -> std::sync::RwLockReadGuard<'_, Node<K>> {
        self.0.read().unwrap()
    }

    pub fn write_lock(&mut self) -> std::sync::RwLockWriteGuard<'_, Node<K>> {
        self.0.write().unwrap()
    }
}


// Node that may or may not be loaded into memory yet
#[derive(Debug, Clone)]
pub enum NodeLink<const K: usize> {
    Empty,
    Unloaded(BlobId),
    Loaded(NodeHandle<K>),
    Edited(NodeHandle<K>)
}


// NYI replace this with std Option?
pub enum SplitResult<const K:usize> {
    Split(NodeHandle<K>),
    NoSplit
}


#[derive(Debug, Clone)]
pub struct Node<const K: usize> {
    pub id : Option<BlobId>, // NYI do we need this afterall?
    pub values : SortedArray<u128>,
    pub children: Option<Vec<NodeLink<K>>>,
    pub next : NodeLink<K>
}


impl<const K: usize> Node<K> {  

    pub fn new_leaf() -> Self {
        Node {
            id: None,
            values: SortedArray::new(),
            children: None,
            next: NodeLink::Empty }
    }


    pub fn new_branch(values: Vec<u128>, children: Vec<NodeLink<K>>, next: NodeLink<K>) -> Self {
        Node { 
            id: None,
            values: SortedArray::from_values(values),
            children: Some(children),
            next: next }
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
}