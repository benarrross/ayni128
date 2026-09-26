use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Write;
use crate::blobstore::*;
use crate::sortedarray::*;
use crate::Table;
use super::nodehandle::*;
use super::nodelink::*;


pub enum SplitResult<const K:usize> {
    Split(NodeHandle<K>),
    NoSplit
}


// This is only used for debugging
static NEXT_NODE_DEBUG_ID: AtomicUsize  = AtomicUsize::new(1);


#[derive(Debug)]
pub(super) struct Node<const K: usize> {
    pub debug_id: usize,
    pub blobid : Option<BlobId>,
    pub values : SortedArray<u128>,
    pub children: Option<Vec<NodeLink<K>>>,
    pub next_link : NodeLink<K>
}


// NYI probably don't need this anymore once we are using NodeLinkOuter everywhere
impl<const K: usize> Clone for Node<K> {
    fn clone(&self) -> Self {
        Node {
            debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
            blobid: self.blobid.clone(),
            values: self.values.clone(),
            children: self.children.clone(),
            next_link: self.next_link.clone(),
        }
    }
}


impl<const K: usize> Node<K> {  

    pub fn empty_leaf() -> Self {
        Node {
            debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
            blobid: None,
            values: SortedArray::new(),
            children: None,
            next_link: NodeLink::new_empty() 
        }
    }


    pub fn new_leaf(values: SortedArray<u128>, next: NodeLink<K>) -> Self {
        Node {
            debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
            blobid: None,
            values: values,
            children: None,
            next_link: next 
        }
    }


    pub fn new_branch(values: SortedArray<u128>, children: Vec<NodeLink<K>>) -> Self {
        Node { 
            debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
            blobid: None,
            values: values,
            children: Some(children),
            next_link: NodeLink::new_empty() 
        }
    }   


    pub fn store(&self, backing_store: &mut BlobStore) -> BlobId {
        
        let mut serialized_node = MemoryStream::new();

        // Serialize each of our child nodes
        // NYI

        // Serialize the lengths of values and children
        let value_count = self.values.len();
        let child_count : usize = match &self.children {
            Some(children) => children.len(),
            None => 0 
        };
        serialized_node.write_all(&value_count.to_le_bytes());
        serialized_node.write_all(&child_count.to_le_bytes());                
        
        // Serialize the values
        for &value in self.values.iter() {
            serialized_node.write_all(&value.to_le_bytes());
        }

        // Serialize the children if we have any (by blobid)
        if (!self.is_leaf()) {
            for child_node_link in self.children.as_ref().unwrap() {
                let child_blobid = child_node_link.get_blobid();
                serialized_node.write_all(&child_blobid.to_le_bytes());
            }
        }

        // Serialize our next link
        serialized_node.write_all(&self.next_link.get_blobid().to_le_bytes());        

        // Store the serialized node in a blob and return the blobid
        backing_store.put(serialized_node.as_slice())
    }


    pub fn is_leaf(&self) -> bool { self.children.is_none() }


    pub fn first_value(&self, table: &Table<K>) -> u128 {
        if (self.is_leaf()) {
            self.values[0]
        }
        else {
            let first_child_hnode = self.children.as_ref().unwrap().get(0).unwrap().get_immutable_hnode(table);
            let first_child_node = first_child_hnode.read_lock();
            first_child_node.first_value(table)
        }
    }


    pub fn check(&self, table: &Table<K>) {

        let mut last: u128 = 0;
        for value in self.values.iter() {
            assert(*value > last);
            last = *value;
        }

        if (self.is_leaf()) {
            assert(self.children.is_none());
            if (!self.next_link.is_empty()) {
                let next_hnode = self.next_link.get_immutable_hnode(table);
                let next_node = next_hnode.read_lock();
                assert(next_node.values[0] > self.values[self.values.len()-1]);
            }
        }
        else {
            assert(self.children.is_some());
            assert(self.values.len() == self.children.as_ref().unwrap().iter().len() - 1);

            for index in 0..self.values.len() {
                let value = self.values[index];

                let child_hnode_before = self.children.as_ref().unwrap().get(index).unwrap().get_immutable_hnode(table);
                let child_node_before = child_hnode_before.read_lock();
                assert(value > child_node_before.values[child_node_before.values.len()-1]);

                let child_hnode_after = self.children.as_ref().unwrap().get(index+1).unwrap().get_immutable_hnode(table);
                let child_node_after = child_hnode_after.read_lock();
                assert(value == child_node_after.first_value(table));
                let x =  child_node_after.values[0];
                assert(value <= child_node_after.values[0]);
            }

            for child_nodelink in self.children.as_ref().unwrap().iter() {
                let child_hnode = child_nodelink.get_immutable_hnode(table);
                let child_node = child_hnode.read_lock();
                child_node.check(table);

            }
        }
    }
}


fn assert(condition: bool) {
    if (!condition) {
        panic!();
    }
}
