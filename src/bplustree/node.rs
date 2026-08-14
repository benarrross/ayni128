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
use super::nodelink::*;


pub enum SplitResult<const K:usize> {
    Split(NodeHandle<K>),
    NoSplit
}


#[derive(Debug)]
pub struct Node<const K: usize> {
    pub id : Option<BlobId>,
    pub values : SortedArray<u128>,
    pub children: Option<Vec<RefCell<NodeLink<K>>>>,
    pub next : NodeLink<K>
}


impl<const K: usize> Clone for Node<K> {
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            values: self.values.clone(),
            children: self.children.clone(),
            next: self.next.clone(),
        }
    }
}


impl<const K: usize> Node<K> {  

    pub fn empty_leaf() -> Self {
        Node {
            id: None,
            values: SortedArray::new(),
            children: None,
            next: NodeLink::Empty }
    }


    pub fn new_leaf(values: SortedArray<u128>, next: NodeLink<K>) -> NodeHandle<K> {
        NodeHandle::new(
            Node {
                id: None,
                values: values,
                children: None,
                next: next 
            })
    }


    pub fn new_branch(values: SortedArray<u128>, children: Vec<RefCell<NodeLink<K>>>) -> NodeHandle<K> {
        NodeHandle::new(
            Node { 
                id: None,
                values: values,
                children: Some(children),
                next: NodeLink::Empty 
            })
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