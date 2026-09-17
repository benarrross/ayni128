use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use crate::BlobStore;
use crate::pslist::editor::create_branch_node;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;
use super::TableView;


pub struct Table<const K: usize> {
    root_node_link: Mutex<NodeLink<K>>,
    loaded_hnodes: RefCell<HashMap<BlobId, NodeHandle<K>>>,
    backing_store: Arc<Mutex<BlobStore>>
}


impl<'a, const K: usize> Table<K> {

    pub fn new(backing_store: Arc<Mutex<BlobStore>>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_node = Node::<K>::empty_leaf();
        let root_id = root_node.store(backing_store.lock().as_mut().unwrap());

        // Start off with one node
        let mut nodes : HashMap<BlobId, NodeHandle<K>> = HashMap::new();
        let root_node_handle = NodeHandle::new(root_node); 
        nodes.insert(root_id, root_node_handle.clone());

        Table { 
            root_node_link: Mutex::new(NodeLink::<K>::immutable(&root_node_handle)),
            loaded_hnodes: RefCell::new(nodes), 
            backing_store: backing_store 
        }
    }


    pub fn open(store: &mut BlobStore) -> Self {
        unimplemented!();
    }


    pub fn get_view(&'a self) -> TableView<'a, K> {
        TableView::new(self, &*self.root_node_link.lock().unwrap())
    }


    pub fn commit(&self, view: &TableView<'a, K>) {

        // Get a write lock on our root node that will persist through the whole commit.
        // This will ensure only one commit happens at a time.
        let root_node_write_lock = &mut self.root_node_link.lock().unwrap();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {

            let mutable_root_hnode = root_node_write_lock.get_mutable_hnode(&self);
            match super::editor::insert_and_split(&mut mutable_root_hnode.write_lock(), *value, self) {
                SplitResult::Split(right_hnode) => {
                    let branch_node = create_branch_node(&mutable_root_hnode, right_hnode.clone());
                    **root_node_write_lock = NodeLink::mutable(&branch_node);
                },
                SplitResult::NoSplit => {}
            };

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
