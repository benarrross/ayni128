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


    pub fn get(&self, value : u128) -> u128 {
        let hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        self.get_from_node(&hnode.read_lock(), value)
    }


    fn get_from_node(&self, node: &Node<K>, value: u128) -> u128 {
        if (node.is_leaf()) {
            node.values.find(value)
        }
        else {
            let index = node.values.find_index_before(value) + 1;
            let child_hnode = self.get_child_hnode(node, index);
            self.get_from_node(&child_hnode.read_lock(), value)
        }
    }

    pub fn iter(&'a self, min: u128, mac: u128) -> RangeCollection<'a, K> {
        let root_hnode = self.get_hnode(&mut self.root_node_link.borrow_mut());
        RangeCollection::new(self, root_hnode, min, mac)
    }   


    pub fn put(&self, value : u128) {

        // Update our list of added/deleted values
        if self.deletes.borrow().exists(value) {
            self.deletes.borrow_mut().remove(value);
        } else {
            self.puts.borrow_mut().insert(value)
        };

        // Update our b+tree and store the new root if necessary
        let mutable_root_hnode = self.get_mutable_hnode(&mut self.root_node_link.borrow_mut());
        self.put_in_mutable_node(&mutable_root_hnode, value).
            map(|new_root_hnode| *self.root_node_link.borrow_mut() = NodeLink::Edited(new_root_hnode));
    }


    pub fn put_in_mutable_node(&self, hnode: &NodeHandle<K>, value: u128) -> Option<NodeHandle<K>> {

        let mut mutable_node = &mut hnode.write_lock();
        let split_result = self.put_and_split(mutable_node, value);
        match &split_result {
            SplitResult::Split(right_hnode) => Some(Self::create_parent_for_split_nodes(hnode, &mut mutable_node, right_hnode.clone())),
            SplitResult::NoSplit => Option::None
        }
    }


    fn create_parent_for_split_nodes(hnode:&NodeHandle<K>, mutable_node: &mut Node<K>, right_hnode: NodeHandle<K>) -> NodeHandle<K> {

        // Fix up the next link on the old node to point to the new one we just created
        let old_next = mem::replace(&mut mutable_node.next, NodeLink::Edited(right_hnode.clone()));

        // Make a new parent node that has the old node on its left and the new node on its right
        let right_node = &*right_hnode.read_lock();
        NodeHandle::new(Node::new_branch(
            vec![right_node.values[0]],
            vec![
                RefCell::new(NodeLink::Edited(hnode.clone())),
                RefCell::new(NodeLink::Edited(right_hnode.clone())) 
            ],
            old_next))
    }


    fn put_and_split(&self, node: &mut Node<K>, value : u128) -> SplitResult<K> {

        if node.is_leaf() {
            node.values.insert(value);

            if (node.values.len() > K) {
                let split_index = node.values.len() / 2;
                let right_values = node.values.split_off(split_index);
                let new_right_node = Node { 
                    id: None,
                    values: right_values,
                    children: None,
                    next: node.next.clone(),
                };
                SplitResult::Split(NodeHandle::new(new_right_node))
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
                node.children.as_mut().unwrap().insert(index + 1, RefCell::new(NodeLink::Edited(right_hnode.clone())));

                if (node.values.len() > K) {
                    let split_index = node.values.len() / 2;
                    let right_values = node.values.split_off(split_index);
                    let right_children = node.children.as_mut().unwrap().split_off(split_index + 1);
                    let new_node = Node {   // NYI THIS IS WRONG... use the same logic as above
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


    fn get_child_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {

        let child_link = &node.children.as_ref().unwrap()[index];
        self.get_hnode(&mut child_link.borrow_mut())
    }


    fn get_child_mutable_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {

        let child_link = &node.children.as_ref().unwrap()[index];
        self.get_mutable_hnode(&mut child_link.borrow_mut())
    }


    fn get_hnode(&self, node_link: &mut NodeLink<K>) -> NodeHandle<K> {
        match node_link {
            NodeLink::Unloaded(id) => {
                let hnode = self.based_on.get_hnode(node_link);
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
                let hnode = self.based_on.get_hnode(node_link);
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

