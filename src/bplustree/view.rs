use std::cell::RefCell;
use crate::sortedarray::*;
use super::bplustree::*;
use super::editor::*;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;


pub struct View<'a, const K: usize> {
    based_on: &'a BPlusTree<'a, K>,
    root_node_link: RefCell<NodeLink<K>>,   // NYI consider making a set method on NodeLink and getting rid of the refcell here
    pub(super) puts: RefCell<SortedArray<u128>>,
    pub(super) deletes: RefCell<SortedArray<u128>>
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
            let value_in_node = node.values.find(value, u128::MAX);
            if (value_in_node < u128::MAX) {
                value_in_node
            }
            else {
                match self.get_next_leaf_from_node(node) {
                    Some(hnode_next) => hnode_next.read_lock().values[0],
                    None => u128::MAX
                }
            }
        }
        else {
            let index = node.values.find_range_index(value);
            let child_hnode = self.get_immutable_child_hnode(node, index);
            self.get_from_node(&child_hnode.read_lock(), value)
        }
    }


    /// Creates an iterator for the view over a given range of values in the B+tree view.
    pub fn iter(&'a self, min: u128, mac: u128) -> ViewIterator<'a, K> {
        let root_hnode = self.root_node_link.borrow().get_immutable(self.based_on);
        ViewIterator::new(self, root_hnode, min, mac)
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
    pub(super) fn get_next_leaf_from_hnode(&self, hnode: &NodeHandle<K>) -> Option<NodeHandle<K>> {
        self.get_next_leaf_from_node(&hnode.read_lock())
    }

    pub(super) fn get_next_leaf_from_node(&self, node: &Node<K>) -> Option<NodeHandle<K>> {
        if node.next_link.is_empty() {
            Option::None
        } else {
            Option::Some(node.next_link.get_immutable(self.based_on))
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


pub struct ViewIterator<'a, const K: usize> {
    based_on_view: &'a View<'a, K>,
    root_hnode: NodeHandle<K>,
    min: u128,
    mac: u128,
    current_hnode: Option<NodeHandle<K>>,
    current_index: usize
}


impl<'a, const K: usize> ViewIterator<'a,  K> {

    pub fn new(based_on_view: &'a View<'a, K>, root_node: NodeHandle<K>, min: u128, mac: u128) -> Self {
        ViewIterator { 
            based_on_view, 
            root_hnode: root_node.clone(), 
            min: min, 
            mac: mac,
            current_hnode: None,
            current_index: 0  }
    }


    fn find_first(&mut self) -> Option<u128> {

        // Find the leaf node
        let mut hnode = self.root_hnode.clone();
        loop {
            let hnode_cur = hnode.clone();
            let node_read_lock = hnode_cur.read_lock();
            if (node_read_lock.is_leaf()) {
                break;
            }

            let child_index = node_read_lock.values.find_range_index(self.min);
            hnode = self.based_on_view.get_immutable_child_hnode(&node_read_lock, child_index);
        }

        // The leaf node we are pointing at might be the one before the one we want, if the caller asks for a value 
        // between two leaf nodes. If this is the case, advance to the next one.
        let mut go_to_next_leaf = false;
        let mut index = 0;
        {
            let leaf_node_read_lock = hnode.read_lock();
            index = leaf_node_read_lock.values.find_index(self.min);
            if (index >= leaf_node_read_lock.values.len()) {
                go_to_next_leaf = true;
            }
        }
        if (go_to_next_leaf) {
            hnode = match self.based_on_view.get_next_leaf_from_hnode(&hnode) {
                Some(hnode_next) => hnode_next,
                None => { return Option::None; }
            };
            index = 0;
        }

        // Now that we have the correct node and index, start enumerating
        self.current_hnode = Option::Some(hnode.clone());
        self.current_index = index;

        // We could be enumerating an empty list
        let node_read_lock = hnode.read_lock();
        if index >= node_read_lock.values.len() {
            Option::None
        } else {
            Option::Some(node_read_lock.values[index])
        }
    }


    fn find_next(&mut self) -> Option<u128> {

        // Advance to the next value in this node
        let mut hnode = self.current_hnode.as_ref().unwrap().clone();
        let mut index = self.current_index + 1;

        // Advance to the next leaf node if necessary
        let mut go_to_next_leaf = false;
        {
            let node_read_lock = hnode.read_lock();
            if index >= node_read_lock.values.len() {
                go_to_next_leaf = true;
            }
        }
        if go_to_next_leaf {
            hnode = match self.based_on_view.get_next_leaf_from_hnode(&hnode) {
                Some(next_hnode) => next_hnode,
                None => { return None; }
            };
            index = 0;
        }

        self.current_hnode = Some(hnode.clone());
        self.current_index = index;

        let node_read_lock = hnode.read_lock();
        if self.current_index >= node_read_lock.values.len() {
            Option::None
        } else {
            Option::Some(node_read_lock.values[self.current_index])
        }
    }
}


impl<'a, const K: usize> Iterator for ViewIterator<'a, K> {

    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {

         match &self.current_hnode {
            None => self.find_first(),
            Some(node) => self.find_next()
        }
    }   
}
