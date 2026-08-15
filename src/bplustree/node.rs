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
    pub children: Option<Vec<RefCell<NodeLinkInner<K>>>>,
    pub next : NodeLinkInner<K>
}

// NYI probably don't need this anymore once we are using NodeLinkOuter everywhere
impl<const K: usize> Clone for Node<K> {
    fn clone(&self) -> Self {
        Node {
            id: self.id.clone(),
            values: self.values.clone(),
            children: match &self.children {
                Option::Some(children) => 
                    Some(
                        children
                        .iter()
                        .map(|nodelink_read_lock| {
                            let nodelink_reader = nodelink_read_lock.borrow();
                            RefCell::new(nodelink_reader.clone())
                        })
                        .collect()),
                    None => None
                },
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
            next: NodeLinkInner::Empty }
    }


    pub fn new_leaf(values: SortedArray<u128>, next: NodeLinkInner<K>) -> NodeHandle<K> {
        NodeHandle::new(
            Node {
                id: None,
                values: values,
                children: None,
                next: next 
            })
    }


    pub fn new_branch(values: SortedArray<u128>, children: Vec<RefCell<NodeLinkInner<K>>>) -> NodeHandle<K> {
        NodeHandle::new(
            Node { 
                id: None,
                values: values,
                children: Some(children),
                next: NodeLinkInner::Empty 
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