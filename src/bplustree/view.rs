use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{SeekFrom, Split, prelude::*};
use std::io::Cursor;
use std::sync::{Arc, RwLock};
use std::mem;

use crate::BlobId;
use crate::sortedarray::*;

use super::bplustree::*;
use super::node::*;
use super::rangeiterator::{*};


pub struct View<'a, const K: usize> {
    based_on: &'a BPlusTree<'a, K>,
    root_node_link: RefCell<NodeLink<K>>,
    puts: RefCell<SortedArray<u128>>,
    deletes: RefCell<SortedArray<u128>>
}


impl<'a, const K: usize> View<'a, K> {

    /// Creates a new read/write view on the B+tree. Each view should only be used by one thread.
    /// You must commit the view for your changes to be saved.
    pub fn new(based_on: &'a BPlusTree<'a, K>, root_node_link: NodeLink<K>) -> Self {

        View { 
            based_on, 
            root_node_link: RefCell::new(root_node_link.clone()),
            puts: RefCell::new(SortedArray::new()),
            deletes: RefCell::new(SortedArray::new())
        }
    }


    /// Gets the next value greater than or equal to the specified value.
    /// NYI this needs the same logic as the enumerator to go to the next leaf node
    pub fn get(&self, value : u128) -> u128 {
        let hnode = self.get_hnode_from_link(&mut self.root_node_link.borrow_mut());
        self.get_from_node(&hnode.read_lock(), value)
    }


    fn get_from_node(&self, node: &Node<K>, value: u128) -> u128 {
        if (node.is_leaf()) {
            node.values.find(value)
        }
        else {
            let index = node.values.find_range_index(value);
            let child_hnode = self.get_child_hnode(node, index);
            self.get_from_node(&child_hnode.read_lock(), value)
        }
    }


    /// Creates an iterator for the view over a given range of values in the B+tree view.
    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        let root_hnode = self.get_hnode_from_link(&mut self.root_node_link.borrow_mut());
        RangeCollection::new(self, root_hnode, min, mac)
    }   


    /// Inserts a value into the B+tree.
    pub fn put(&self, value : u128) {

        // Update our list of added/deleted values. This is used to commit the transaction later.
        if self.deletes.borrow().exists(value) {
            self.deletes.borrow_mut().remove(value);
        } else {
            self.puts.borrow_mut().insert(value)
        };

        // Update our b+tree and store the new root if necessary
        let mutable_root_hnode = self.get_mutable_hnode_from_link(&mut self.root_node_link.borrow_mut());
        if let SplitResult::Split(right_hnode) = self.insert_and_split(&mut mutable_root_hnode.write_lock(), value) {
           *self.root_node_link.borrow_mut() = NodeLink::Edited(
                Self::create_branch_node(&mutable_root_hnode, right_hnode.clone()));
        }
    }


    /// Creates a new branch node from the specified left and right nodes
    fn create_branch_node(left_hnode: &NodeHandle<K>, right_hnode: NodeHandle<K>) -> NodeHandle<K> {

        // Make a new parent node that has the old node on its left and the new node on its right
        let right_node = &*right_hnode.read_lock();
        Node::new_branch(
            SortedArray::from_values(vec![right_node.values[0]]),
            vec![
                RefCell::new(NodeLink::Edited(left_hnode.clone())),
                RefCell::new(NodeLink::Edited(right_hnode.clone())) 
            ])
    }


    /// Splits the right half of a leaf node off, and updates the leaf node linked list.
    fn split_leaf_node(node: &mut Node<K>) -> NodeHandle<K> {
        
        // Make a new right node
        let split_index = node.values.len() / 2;
        let right_values = node.values.split_off(split_index);
        let new_right_hnode = Node::new_leaf(right_values, node.next.clone());

        // Link the node we just split from to the new node in the leaf node linked list
        node.next = NodeLink::Edited(new_right_hnode.clone());
        new_right_hnode
    }


    /// Splits the right half of a node off into a new branch node and returns it.
    fn split_branch_node(node: &mut Node<K>) -> NodeHandle<K> {
        
        let split_index = node.values.len() / 2;
        let right_values = node.values.split_off(split_index);
        let right_children = node.children.as_mut().unwrap().split_off(split_index);
        Node::new_branch(right_values, right_children)
    }


    /// Inserts a value into a node and splits it if necessary
    fn insert_and_split(&self, node: &mut Node<K>, value : u128) -> SplitResult<K> {

        if node.is_leaf() {
            node.values.insert(value);

            if (node.values.len() > K) {
                SplitResult::Split(Self::split_leaf_node(node))
            } else {
                SplitResult::NoSplit
            }
        }
        else {
            // Find the child this should go in and ask the child to insert the value
            let index = node.values.find_range_index(value);
            let mut mutable_child_hnode = self.get_child_mutable_hnode(node, index);

            // Handle the child splitting (which might force us to split the current node also)
            if let SplitResult::Split(right_hnode) = self.insert_and_split(&mut mutable_child_hnode.write_lock(), value) {

                let right_node = &*right_hnode.read_lock();
                node.values.insert(right_node.values[0]);
                node.children.as_mut().unwrap().insert(index, RefCell::new(NodeLink::Edited(right_hnode.clone())));

                // Now see if we need to split
                if (node.values.len() > K) {
                    SplitResult::Split(Self::split_branch_node(node))
                } else {
                    SplitResult::NoSplit
                }
            } else {
                SplitResult::NoSplit
            }
        }
    }


    pub(super) fn get_child_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {
        let child_link = &node.children.as_ref().unwrap()[index];
        self.get_hnode_from_link(&mut child_link.borrow_mut())
    }


    /// Gets a handle to a child and ensures it is mutable. Clones it into the current
    /// transaction if necessary. Updates the NodeLink in the passed in node if necessary.
    fn get_child_mutable_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {
        let child_link = &node.children.as_ref().unwrap()[index];
        self.get_mutable_hnode_from_link(&mut child_link.borrow_mut())
    }


    pub(super) fn get_hnode_from_link(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {
        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode_from_link(node_link);
                *node_link = NodeLink::Loaded(hnode.clone());
                hnode
            },
            NodeLink::Loaded(hnode) => hnode.clone(),
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("NYI")
        }
    }


    pub(super) fn get_next_hnode(&self, hnode: &NodeHandle<K>) -> Option<NodeHandle<K>> {
        let mut node = hnode.write_lock();

        if matches!(node.next, NodeLink::Empty) {
            Option::None
        } else {
            Option::Some(self.get_hnode_from_link(&mut node.next))
        }
    }


    /// Gets a mutable copy of the node and updates the passed in link if necesssary
    fn get_mutable_hnode_from_link(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {

        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode_from_link(node_link);
                let mutable_node = hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                *node_link = NodeLink::Edited(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLink::Loaded(hnode) => {
                let mutable_node = hnode.read_lock().clone();
                let mutable_hnode = NodeHandle::new(mutable_node);
                *node_link = NodeLink::Edited(mutable_hnode.clone());
                mutable_hnode
            },
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("Can't make an empty node link mutable")
        }
    }
}

