use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use crate::BlobStore;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;
use super::View;


pub struct BPlusTree<const K: usize> {
    root_node_link: RefCell<NodeLink<K>>,   // NYI consider making a set method on NodeLink and getting rid of the refcell here
    loaded_hnodes: RefCell<HashMap<BlobId, NodeHandle<K>>>,
    backing_store: Arc<Mutex<BlobStore>>
}


impl<'a, const K: usize> BPlusTree<K> {

    pub fn new(backing_store: Arc<Mutex<BlobStore>>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_node = Node::<K>::empty_leaf();
        let root_id = root_node.store(backing_store.lock().as_mut().unwrap());

        // Start off with one node
        let mut nodes : HashMap<BlobId, NodeHandle<K>> = HashMap::new();
        let root_node_handle = NodeHandle::new(root_node); 
        nodes.insert(root_id, root_node_handle.clone());

        BPlusTree { 
            root_node_link: RefCell::new(NodeLink::<K>::immutable(&root_node_handle)),
            loaded_hnodes: RefCell::new(nodes), 
            backing_store: backing_store 
        }
    }


    pub fn open(store: &mut BlobStore) -> Self {
        panic!("NYI");
    }


    pub fn get_view(&'a self) -> View<'a, K> {
        View::new(self, &*self.root_node_link.borrow())
    }


    pub fn commit(&self, view: &View<'a, K>) {

        // Get a write lock on our root node that will persist through the whole commit.
        // This will ensure only one commit happens at a time.
        let mutable_root_hnode = self.root_node_link.borrow().get_mutable(self);
        let root_node_write_lock = &mut mutable_root_hnode.write_lock();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {

            // NYI handle splits
            super::editor::insert_and_split(root_node_write_lock, *value, self);

        // if let SplitResult::Split(right_hnode) = super::editor::insert_and_split(
        //     &mut mutable_root_hnode.write_lock(), *value, self) {
        //         *self.root_node_link.borrow_mut() = NodeLink::mutable(
        //             create_branch_node(&mutable_root_hnode, right_hnode.clone()));
        //     }
        }

        // Remove all deleted values from the committed b+tree
        // NYI

        // Write the edited nodes to storage
        // NYI

    }


    pub(crate) fn load(&self, node_link: &NodeLink<K>) -> NodeHandle<K> {
        unimplemented!()
    }
}
