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
    pub id : Option<BlobId>,
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


    fn put_core(&mut self, value : u128) -> SplitResult<K> {

        if self.is_leaf() {
            self.values.insert(value);

            if (self.values.len() > K) {
                let split_index = self.values.len() / 2;
                let right_values = self.values.split_off(split_index);
                let new_node = Node {   // NYI ask the bplustree for a new node instead
                    id: None,
                    values: right_values,
                    children: None,
                    next: self.next.clone(),
                };
                SplitResult::Split(NodeHandle::new(new_node))
            }
            else {
                SplitResult::NoSplit
            }
        }
        else {
            // NYI binary search values to see which child it should go in. Then handle the child
            // splitting.
            unimplemented!();
        }
    }


    pub fn put(hnode: &mut NodeHandle<K>, value: u128) -> Option<NodeHandle<K>> {

        // Take a write lock on the node
        let original_hnode = &hnode.clone();
        let mut mutable_node = &mut hnode.write_lock(); // NYI use get_mutable_node here
       
        // Tell the node to insert the value. If it splits, create a new root with the two nodes as children.
        let split_result = mutable_node.put_core(value);
        match &split_result {
            SplitResult::Split(right_hnode) => {

                // Update the node we put into to have the new right node as its next link. But store the 
                // old next value so we can put it in the new right node.
                let old_next = mem::replace(&mut mutable_node.next, NodeLink::Edited(right_hnode.clone()));

                // Make a new parent node that has the old node on its left and the new node on its right
                let right_node = &*right_hnode.read_lock();
                Some(NodeHandle::new(Node::new_branch(
                    vec![right_node.values[0]],
                    vec![
                        NodeLink::Edited(original_hnode.clone()),
                        NodeLink::Edited(right_hnode.clone()) 
                    ],
                    old_next)))
            },
            SplitResult::NoSplit => { Option::None }
        }
    }


    pub fn get(&self, value : u128) -> u128 {
        unimplemented!();
    }
}