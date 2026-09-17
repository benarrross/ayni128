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
    pub id : Option<BlobId>,
    pub values : SortedArray<u128>,
    pub children: Option<Vec<NodeLink<K>>>,
    pub next_link : NodeLink<K>
}


// NYI probably don't need this anymore once we are using NodeLinkOuter everywhere
impl<const K: usize> Clone for Node<K> {
    fn clone(&self) -> Self {
        Node {
            debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
            id: self.id.clone(),
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
            id: None,
            values: SortedArray::new(),
            children: None,
            next_link: NodeLink::empty() 
        }
    }


    pub fn new_leaf(values: SortedArray<u128>, next: NodeLink<K>) -> NodeHandle<K> {
        NodeHandle::new(
            Node {
                debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
                id: None,
                values: values,
                children: None,
                next_link: next 
            })
    }


    pub fn new_branch(values: SortedArray<u128>, children: Vec<NodeLink<K>>) -> NodeHandle<K> {
        NodeHandle::new(
            Node { 
                debug_id: NEXT_NODE_DEBUG_ID.fetch_add(1, Ordering::Relaxed),
                id: None,
                values: values,
                children: Some(children),
                next_link: NodeLink::empty() 
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
                assert(value == child_node_after.values[0]);
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

// #[derive(Debug)]
// pub(super) struct Node<const K: usize> {
//     pub debug_id: usize,
//     pub id : Option<BlobId>,
//     pub values : SortedArray<u128>,
//     pub children: Option<Vec<NodeLink<K>>>,
//     pub next_link : NodeLink<K>
// }
