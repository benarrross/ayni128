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
    root_node_link: NodeLink<K>,
    root_blobid: RefCell<BlobId>,
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
            root_node_link: NodeLink::<K>::new_loaded(&root_node_handle),
            root_blobid: RefCell::new(BlobId::new_empty()),
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

        TableView::new(self, &self.root_node_link)
    }


    pub fn commit(&self, view: &TableView<'a, K>) -> BlobId {

        // Lock the backing store at the top of commit so we only commit one view (transaction) at a time
        let mut blob_store = self.backing_store.lock().unwrap();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {

            // NYI it's strange and wrong that we call view to get the mutable node... need to get it from ourselves
            let mutable_root_hnode = view.get_mutable_hnode(&self.root_node_link);
            match super::editor::insert_and_split(&mut view.get_mutable_node(&mutable_root_hnode), *value, self, view) {
                SplitResult::Split(right_hnode) => {
                    let branch_node = create_branch_node(&mutable_root_hnode, right_hnode.clone(), self, view);
                    self.root_node_link.set_mutable(&branch_node);
                },
                SplitResult::NoSplit => {}
            };
        }

        // Remove all deleted values from the committed b+tree
        // NYI

        // Write the edited nodes to storage (if there are any)
        if self.root_node_link.is_mutable() {
            // NYI it's strange and wrong that we call view to get the mutable node... need to get it from ourselves
            let root_hnode = view.get_mutable_hnode(&self.root_node_link);
            let root_node = view.get_mutable_node(&root_hnode);
            let root_blobid = root_node.store(&mut blob_store);

            // Rewrite the root node link
            // NYI bring back this commented out line
//            self.root_node_link.set_unloaded(root_blobid);
            self.root_blobid.replace(root_blobid);
        }

        // Return our root blobid, regardless of whether it changed or not
        *self.root_blobid.borrow()
    }


    pub(super) fn load(&self, node_link: &NodeLink<K>) -> NodeHandle<K> {
        unimplemented!()
    }
}
