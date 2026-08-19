use std::cell::RefCell;
use crate::sortedarray::*;
use super::bplustree::*;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;
use super::rangeiterator::{*};


pub struct View<'a, const K: usize> {
    based_on: &'a BPlusTree<'a, K>,
    root_node_link: RefCell<NodeLink<K>>,   // NYI consider making a set method on NodeLink and getting rid of the refcell here
    puts: RefCell<SortedArray<u128>>,
    deletes: RefCell<SortedArray<u128>>
}


impl<'a, const K: usize> View<'a, K> {

    /// Creates a new read/write view on the B+tree. Each view should only be used by one thread.
    /// You must commit the view for your changes to be saved.
    pub fn new(based_on: &'a BPlusTree<'a, K>, root_node_link: NodeLink<K>) -> Self {
        View { 
            based_on: based_on,
            root_node_link: RefCell::new(root_node_link.clone()),
            puts: RefCell::new(SortedArray::new()),
            deletes: RefCell::new(SortedArray::new())
        }
    }


    /// Gets the next value greater than or equal to the specified value.
    /// NYI this needs the same logic as the enumerator to go to the next leaf node
    pub fn get(&self, value : u128) -> u128 {
        let hnode = self.root_node_link.borrow().get_immutable(self.based_on);
        self.get_from_node(&hnode.read_lock(), value)
    }


    fn get_from_node(&self, node: &Node<K>, value: u128) -> u128 {
        if (node.is_leaf()) {
            // NYI this should use find_range_index and go to the next node when necessary
            node.values.find(value, u128::MAX)
        }
        else {
            let index = node.values.find_range_index(value);
            let child_hnode = self.get_immutable_child_hnode(node, index);
            self.get_from_node(&child_hnode.read_lock(), value)
        }
    }


    /// Creates an iterator for the view over a given range of values in the B+tree view.
    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        let root_hnode = self.root_node_link.borrow().get_immutable(self.based_on);
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
        let mutable_root_hnode = &self.root_node_link.borrow().get_mutable(self.based_on);
        if let SplitResult::Split(right_hnode) = insert_and_split(&mut mutable_root_hnode.write_lock(), value, self.based_on) {
           *self.root_node_link.borrow_mut() = NodeLink::mutable(
                create_branch_node(&mutable_root_hnode, right_hnode.clone()));
        }
    }


    /// Gets a handle to a child node, loading the child node if necessary. This should only be used for read operations.
    pub(super) fn get_immutable_child_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {
        let child_link = &node.children.as_ref().unwrap()[index];
        child_link.get_immutable(self.based_on)
    }


    /// Given a handle to a node, returns a handle to the next leaf node after it if there is one.
    pub(super) fn get_next_leaf_hnode(&self, hnode: &NodeHandle<K>) -> Option<NodeHandle<K>> {
        let mut node_read = hnode.read_lock();
        
        if node_read.next.is_empty() {
            Option::None
        } else {
            Option::Some(node_read.next.get_immutable(self.based_on))
        }
    }
}


/// Creates a new branch node from the specified left and right nodes
fn create_branch_node<const K:usize>(left_hnode: &NodeHandle<K>, right_hnode: NodeHandle<K>) -> NodeHandle<K> {

    // Make a new parent node that has the old node on its left and the new node on its right
    let right_node = &*right_hnode.read_lock();
    Node::new_branch(
        SortedArray::from_values(vec![right_node.values[0]]),   // NYI This is wrong for branch nodes -- need a node.first_value() method
        vec![
            NodeLink::mutable(left_hnode.clone()),
            NodeLink::mutable(right_hnode.clone()) 
        ])
}


/// Inserts a value into a node and splits it if necessary.
fn insert_and_split<const K:usize>(node: &mut Node<K>, value : u128, node_store: &dyn NodeStore<K>) -> SplitResult<K> {
    if node.is_leaf() {
        node.values.insert(value);

        if (node.values.len() > K) {
            SplitResult::Split(split_leaf_node(node))
        } else {
            SplitResult::NoSplit
        }
    }
    else {
        // Find the child this should go in and ask the child to insert the value
        let index = node.values.find_range_index(value);
        let mut mutable_child_hnode = &node.children.as_ref().unwrap()[index].get_mutable(node_store);

        // Handle the child splitting (which might force us to split the current node also)
        if let SplitResult::Split(right_hnode) = insert_and_split(&mut mutable_child_hnode.write_lock(), value, node_store) {

            let right_node = &*right_hnode.read_lock();
            let first_value_in_right_node = right_node.values[0];  // NYI This is wrong for branch nodes -- need a node.first_value() method
            node.values.insert(first_value_in_right_node);

            // NYI index is wrong here, I think? Perhaps just add one?
            node.children.as_mut().unwrap().insert(index, NodeLink::mutable(right_hnode.clone()));

            // Now see if we need to split
            if (node.values.len() > K) {
                SplitResult::Split(split_branch_node(node))
            } else {
                SplitResult::NoSplit
            }
        } else {
            SplitResult::NoSplit
        }
    }
}


/// Splits the right half of a leaf node off, and updates the leaf node linked list.
fn split_leaf_node<const K:usize>(node: &mut Node<K>) -> NodeHandle<K> {
    
    // Make a new right node
    let split_index = node.values.len() / 2;
    let right_values = node.values.split_off(split_index);
    let new_right_hnode = Node::new_leaf(right_values, node.next.clone());

    // Link the node we just split from to the new node in the leaf node linked list
    node.next = NodeLink::mutable(new_right_hnode.clone());
    new_right_hnode
}


/// Splits the right half of a node off into a new branch node and returns it.
fn split_branch_node<const K:usize>(node: &mut Node<K>) -> NodeHandle<K> {
    let split_index = node.values.len() / 2;
    let right_values = node.values.split_off(split_index);
    let right_children = node.children.as_mut().unwrap().split_off(split_index);
    Node::new_branch(right_values, right_children)
}
