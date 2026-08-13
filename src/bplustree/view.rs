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

    pub fn new(based_on: &'a BPlusTree<'a, K>, root_node_link: NodeLink<K>) -> Self {

        View { 
            based_on, 
            root_node_link: RefCell::new(root_node_link.clone()),
            puts: RefCell::new(SortedArray::new()),
            deletes: RefCell::new(SortedArray::new())
        }
    }


    pub fn put(&self, value : u128) {

        // Update our list of added/deleted values
        match self.deletes.borrow().get(value) {
            Some(value) => self.deletes.borrow_mut().remove(value),
            None => self.puts.borrow_mut().insert(value)
        };

        // Update our b+tree and store the new root if necessary
        let mut mutable_root_hnode = self.get_mutable_hnode(&mut self.root_node_link.borrow_mut());
        self.put_in_mutable_node(&mut mutable_root_hnode, value).
            map(|new_root_hnode| *self.root_node_link.borrow_mut() = NodeLink::Edited(new_root_hnode));
    }


    pub fn put_in_mutable_node(&self, hnode: &mut NodeHandle<K>, value: u128) -> Option<NodeHandle<K>> {

        let original_hnode = &hnode.clone();
        let mut mutable_node = &mut hnode.write_lock(); // NYI use get_mutable_node here
       
        // Tell the node to insert the value. If it splits, create a new root with the two nodes as children.
        let split_result = self.put_and_split(mutable_node, value);
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


    fn put_and_split(&self, node: &mut Node<K>, value : u128) -> SplitResult<K> {

        if node.is_leaf() {
            node.values.insert(value);

            if (node.values.len() > K) {
                let split_index = node.values.len() / 2;
                let right_values = node.values.split_off(split_index);
                let new_node = Node {   // NYI ask the bplustree for a new node instead
                    id: None,
                    values: right_values,
                    children: None,
                    next: node.next.clone(),
                };
                SplitResult::Split(NodeHandle::new(new_node))
            }
            else {
                SplitResult::NoSplit
            }
        }
        else {
            // binary search values to see which child it should go in. Then handle the child splitting.
            // Then see if this node needs to be split as well.
            let index = node.values.find_index_before(value) + 1;
            let mut mutable_child_hnode = self.get_child_mutable_hnode(node, index);
            let mut mutable_child_node = &mut mutable_child_hnode.write_lock();

            let split_result = self.put_and_split(mutable_child_node, value);
            if let SplitResult::Split(right_hnode) = split_result {
                
                // Insert the new node right after the one we just split
                let right_node = &*right_hnode.read_lock();
                node.values.insert(right_node.values[0]);
                node.children.as_mut().unwrap().insert(index + 1, NodeLink::Edited(right_hnode.clone()));

                if (node.values.len() > K) {
                    let split_index = node.values.len() / 2;
                    let right_values = node.values.split_off(split_index);
                    let right_children = node.children.as_mut().unwrap().split_off(split_index + 1);
                    let new_node = Node {
                        id: None,
                        values: right_values,
                        children: Some(right_children),
                        next: node.next.clone(),
                    };
                    SplitResult::Split(NodeHandle::new(new_node))
                }
                else {
                    SplitResult::NoSplit
                }
            }
            else {
                SplitResult::NoSplit
            }
        }
    }


    fn get_child_hnode(&self, node: &mut Node<K>, index: usize) -> NodeHandle<K> {

        let child_link = &mut node.children.as_mut().unwrap()[index];
        self.get_hnode(child_link)
    }


    fn get_child_mutable_hnode(&self, node: &mut Node<K>, index: usize) -> NodeHandle<K> {

        let child_link = &mut node.children.as_mut().unwrap()[index];
        self.get_mutable_hnode(child_link)
    }


    pub fn delete(&self, value : u128) {
        panic!("NYI");
    }


    pub fn get(&self, value : u128) -> u128 {
        let root_hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        //root_hnode.read_lock().get(value)
        unimplemented!();
    }


    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        let root_hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        RangeCollection::new(self, root_hnode, min, mac)
    }   


    fn get_hnode(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {
        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode(node_link.clone());
                *node_link = NodeLink::Loaded(hnode.clone());
                hnode
            },
            NodeLink::Loaded(hnode) => hnode.clone(),
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("NYI")
        }
    }

    /// Gets a mutable version of the node and updates the passed in link if necesssary
    fn get_mutable_hnode(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {

        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode(node_link.clone());
                let node_copy = hnode.read_lock().clone();
                let new_hnode = NodeHandle::new(node_copy);
                *node_link = NodeLink::Edited(new_hnode.clone());
                new_hnode
            },
            NodeLink::Loaded(hnode) => {
                let node_copy = hnode.read_lock().clone();
                let new_hnode = NodeHandle::new(node_copy);
                *node_link = NodeLink::Edited(new_hnode.clone());
                new_hnode
            },
            NodeLink::Edited(hnode) => hnode.clone(),
            NodeLink::Empty => panic!("Can't make an empty node link mutable")
        }
    }
}

