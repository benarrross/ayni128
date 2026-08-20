use std::cell::RefCell;
use std::collections::HashMap;
use crate::BlobId;
use crate::BlobStore;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;
use super::View;


pub struct BPlusTree<'a, const K: usize> {
    root_id: BlobId,    // NYI change this to root: NodeLink<K>, perhaps in a mutex
    loaded_hnodes: RefCell<HashMap<BlobId, NodeHandle<K>>>,
    backing_store: &'a mut BlobStore<'a>
}


impl <'a, const K: usize> BPlusTree<'a, K> {

    pub fn new(backing_store: &'a mut BlobStore<'a>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_node = Node::<K>::empty_leaf();
        let root_id = root_node.store(backing_store);

        // Start off with one node
        let mut nodes : HashMap<BlobId, NodeHandle<K>> = HashMap::new();
        nodes.insert(root_id, NodeHandle::new(root_node));

        BPlusTree { root_id, loaded_hnodes: RefCell::new(nodes), backing_store }
    }


    pub fn open(store: &mut BlobStore<'a>) -> Self {
        panic!("NYI");
    }


    pub fn get_view(&'a self) -> View<'a, K> {
        View::new(self, self.create_link_to_loaded_node(self.root_id))
    }


    pub fn commit(&self, view: &View<'a, K>) {

        // NYI wrap this in a lock so only one instance can run at a time
        // Perhaps put a mutex around root_id? Or is the write lock on root
        // good enough?

        // Get a write lock on our root node since we are going to modify it
        let root_link = self.create_link_to_loaded_node(self.root_id);
        let mutable_root_hnode = root_link.get_mutable(self);
        let root_node_write_lock = &mut mutable_root_hnode.write_lock();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {
            // NYI handle splits
            super::editor::insert_and_split(root_node_write_lock, *value, self);

        // if let SplitResult::Split(right_hnode) = insert_and_split(&mut mutable_root_hnode.write_lock(), value, self.based_on) {
        //    *self.root_node_link.borrow_mut() = NodeLink::mutable(
        //         create_branch_node(&mutable_root_hnode, right_hnode.clone()));
        }

        // Remove all deleted values from the committed b+tree
        // NYI

        // Write the edited nodes to storage
        // NYI
        
    }


    fn create_link_to_loaded_node(&self, blobid: BlobId) -> NodeLink<K> {
        match self.loaded_hnodes.borrow().get(&blobid) {
            Some(loaded_hnode) => NodeLink::immutable(loaded_hnode.clone()),
            None => NodeLink::unloaded(blobid) // NYI need to actually load the node
        }
    }
}


impl<'a, const K:usize> NodeStore<K> for BPlusTree<'a, K> {

    fn load(&self, node_link: &NodeLink<K>) -> NodeHandle<K> {
        unimplemented!()
    }
}
