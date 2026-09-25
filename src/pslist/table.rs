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
    root_node_link: Mutex<NodeLink<K>>, // NYI do we need this mutex, given that NodeLink has a RwLock inside it?
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

        // Lock the backing store so we don't commit at the same time
        let backing_store_lock = self.backing_store.lock();

        TableView::new(self, &*self.root_node_link.lock().unwrap())
    }


    pub fn commit(&self, view: &TableView<'a, K>) {

        // Lock the backing store at the top of commit so we only commit one view (transaction) at a time
        let mut blob_store = self.backing_store.lock().unwrap();

        // Get a write lock on our root node that will persist through the whole commit.
        // This will ensure only one commit happens at a time.
        // NYI get rid of the mutex on root_node_link, and lock backing_store instead
        let root_node_write_lock = &mut self.root_node_link.lock().unwrap();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {

            let mutable_root_hnode = root_node_write_lock.get_mutable_hnode(&self);
            match super::editor::insert_and_split(&mut mutable_root_hnode.write_lock(), *value, self) {
                SplitResult::Split(right_hnode) => {
                    let branch_node = create_branch_node(&mutable_root_hnode, right_hnode.clone(), self);
                    **root_node_write_lock = NodeLink::mutable(&branch_node);
                },
                SplitResult::NoSplit => {}
            };
        }

        // Remove all deleted values from the committed b+tree
        // NYI

        // Write the edited nodes to storage (if there are any)
        if root_node_write_lock.is_mutable() {
            let root_hnode = root_node_write_lock.get_mutable_hnode(&self);
            let root_node = root_hnode.write_lock();
            let root_blobid = root_node.store(&mut blob_store);

            // Rewrite the root node link
            root_node_write_lock.set_unloaded(root_blobid);
        }
    }


    pub(super) fn load(&self, node_link: &NodeLink<K>) -> NodeHandle<K> {
        unimplemented!()
    }
}
