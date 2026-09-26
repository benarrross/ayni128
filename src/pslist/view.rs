use std::cell::RefCell;
use crate::sortedarray::*;
use super::table::*;
use super::editor::*;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;


pub struct TableView<'a, const K: usize> {
    based_on: &'a Table<K>,
    root_node_link: RefCell<NodeLink<K>>,
    pub(super) puts: RefCell<SortedArray<u128>>,
    pub(super) deletes: RefCell<SortedArray<u128>>
}


impl<'a, const K: usize> TableView<'a, K> {

    /// Creates a new read/write view on the B+tree. Each view should only be used by one thread.
    /// You must commit the view for your changes to be saved.
    pub(super) fn new(based_on: &'a Table<K>, root_node_link: &NodeLink<K>) -> Self {
        TableView { 
            based_on: based_on,
            root_node_link: RefCell::new(root_node_link.clone()),
            puts: RefCell::new(SortedArray::new()),
            deletes: RefCell::new(SortedArray::new())
        }
    }


    /// Gets the next value greater than or equal to the specified value.
    /// NYI this needs the same logic as the enumerator to go to the next leaf node
    pub fn get(&self, value : u128) -> u128 {
        let hnode = self.get_immutable_hnode(&self.root_node_link.borrow());
        self.get_from_node(&hnode.read_lock(), value)
    }


    fn get_from_node(&self, node: &Node<K>, value: u128) -> u128 {
        if (node.is_leaf()) {
            let value_in_node = node.values.find(value, u128::MAX);
            if (value_in_node < u128::MAX) {
                value_in_node
            } else {
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
    pub fn iter(&'a self, min: u128, mac: u128) -> TableIterator<'a, K> {
        let root_hnode = self.get_immutable_hnode(&self.root_node_link.borrow());
        TableIterator::new(self, root_hnode, min, mac)
    }   


    /// Inserts a value into the B+tree.
    pub fn insert(&self, value: u128) {

        self.check();

        // Update our list of added/deleted values. This is used to commit the transaction later.
        if self.deletes.borrow().exists(value) {
            self.deletes.borrow_mut().remove(value);
        } else {
            self.puts.borrow_mut().insert(value)
        };

        // Update our b+tree and store the new root if necessary
        let mutable_root_hnode = &self.root_node_link.borrow().get_mutable_hnode(self.based_on);
        if let SplitResult::Split(right_hnode) = insert_and_split(&mut mutable_root_hnode.write_lock(), value, self.based_on) {
           *self.root_node_link.borrow_mut() = NodeLink::new_mutable(
                &create_branch_node(&mutable_root_hnode, right_hnode.clone(), self.based_on));
        }

        self.check();
    }


    pub(super) fn check(&self) {
        let root_node = self.get_immutable_hnode(&self.root_node_link.borrow());
        root_node.read_lock().check(&self.based_on);

    }

    /// Gets a handle to a child node, loading the child node if necessary. This should only be used for read operations.
    pub(super) fn get_immutable_child_hnode(&self, node: &Node<K>, index: usize) -> NodeHandle<K> {
        let child_link = &node.children.as_ref().unwrap()[index];
        self.get_immutable_hnode(&child_link)
    }


    pub(super) fn get_immutable_hnode(&self, node_link: &NodeLink<K>) -> NodeHandle<K> {
        let mut new_inner = NodeLinkKind::Empty;

        let loaded_hnode = match &*node_link.inner.read().unwrap() {
            NodeLinkKind::Unloaded(id) => {
                let hnode = self.based_on.load(&node_link);
                new_inner = NodeLinkKind::Mutable(hnode.clone());
                hnode
            },
            NodeLinkKind::Loaded(hnode) => hnode.clone(),
            NodeLinkKind::Mutable(hnode) => hnode.clone(),
            NodeLinkKind::Empty => panic!("Can't get an empty node link")
        };

        if !matches!(&new_inner, NodeLinkKind::Empty) {
            *node_link.inner.write().unwrap() = new_inner;
        }

        loaded_hnode
    }


    /// Given a handle to a node, returns a handle to the next leaf node after it if there is one.
    pub(super) fn get_next_leaf_from_hnode(&self, hnode: &NodeHandle<K>) -> Option<NodeHandle<K>> {
        self.get_next_leaf_from_node(&hnode.read_lock())
    }


    pub(super) fn get_next_leaf_from_node(&self, node: &Node<K>) -> Option<NodeHandle<K>> {
        if node.next_link.is_empty() {
            Option::None
        } else {
            Option::Some(self.get_immutable_hnode(&node.next_link))
        }
    }
}


pub struct TableIterator<'a, const K: usize> {
    based_on_view: &'a TableView<'a, K>,
    root_hnode: NodeHandle<K>,
    min: u128,
    mac: u128,
    hnode: Option<NodeHandle<K>>,
    index: usize
}


impl<'a, const K: usize> TableIterator<'a,  K> {

    pub(super) fn new(based_on_view: &'a TableView<'a, K>, root_node: NodeHandle<K>, min: u128, mac: u128) -> Self {
        TableIterator { 
            based_on_view, 
            root_hnode: root_node.clone(), 
            min: min, 
            mac: mac,
            hnode: None,
            index: 0  }
    }


    fn find_first(&mut self) -> Option<u128> {

        // Find the leaf node
        let mut hnode = self.root_hnode.clone();
        loop {
            let hnode_cur = hnode.clone();
            let node = hnode_cur.read_lock();
            if (node.is_leaf()) {
                break;
            }

            let child_index = node.values.find_range_index(self.min);
            hnode = self.based_on_view.get_immutable_child_hnode(&node, child_index);
        }

        // The leaf node we are pointing at might be the one before the one we want, if the caller asks for a value 
        // between two leaf nodes. If this is the case, advance to the next one.
        let mut go_to_next_leaf = false;
        let mut index = 0;
        {
            let leaf_node = hnode.read_lock();
            index = leaf_node.values.find_index(self.min);
            if (index >= leaf_node.values.len()) {
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
        self.hnode = Option::Some(hnode.clone());
        self.index = index;

        // We could be enumerating an empty list
        let node = hnode.read_lock();
        if index >= node.values.len() || node.values[index] >= self.mac {
            Option::None
        } else {
            Option::Some(node.values[index])
        }
    }


    fn find_next(&mut self) -> Option<u128> {

        // Advance to the next value in this node
        let mut hnode = self.hnode.as_ref().unwrap().clone();
        let mut index = self.index + 1;

        // Advance to the next leaf node if necessary
        let mut go_to_next_leaf = false;
        {
            let node = hnode.read_lock();
            if index >= node.values.len() {
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

        self.hnode = Some(hnode.clone());
        self.index = index;

        let node = hnode.read_lock();
        if self.index >= node.values.len() || node.values[self.index] >= self.mac {
            Option::None
        } else {
            Option::Some(node.values[self.index])
        }
    }
}


impl<'a, const K: usize> Iterator for TableIterator<'a, K> {

    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {

         match &self.hnode {
            None => self.find_first(),
            Some(node) => self.find_next()
        }
    }   
}
