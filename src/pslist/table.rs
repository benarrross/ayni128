use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::BlobId;
use crate::BlobStore;
use crate::pslist::editor::create_branch_page;
use super::page::*;
use super::pagehandle::*;
use super::pagelink::*;
use super::pagemap::*;
use super::TableView;


pub struct Table<const K: usize> {
    root_page_link: PageLink<K>,
    root_blobid: RefCell<BlobId>,
    backing_store: Arc<Mutex<BlobStore>>,
    loaded_pages: Arc<Mutex<PageMap<K>>>,
}


impl<'a, const K: usize> Table<K> {

    pub fn new(backing_store: Arc<Mutex<BlobStore>>) -> Self {

        // Make a new, empty node for our root, store it, and add it to  our blobs map
        let root_page = Page::<K>::empty_leaf();
        let root_id = root_page.store(backing_store.lock().as_mut().unwrap());

        // Start off with one node
        let mut pages : HashMap<BlobId, PageHandle<K>> = HashMap::new();
        let root_hpage = PageHandle::new(root_page); 
        pages.insert(root_id, root_hpage.clone());

        Table { 
            root_page_link: PageLink::<K>::new_loaded(&root_hpage),
            root_blobid: RefCell::new(BlobId::new_empty()),
            backing_store: backing_store,
            loaded_pages: Arc::new(Mutex::new(PageMap::new()))
        }
    }


    pub fn open(store: &mut BlobStore) -> Self {
        unimplemented!();
    }


    pub fn get_view(&'a self) -> TableView<'a, K> {

        // Lock the backing store so we don't commit at the same time
        let backing_store_lock = self.backing_store.lock();

        TableView::new(self, &self.root_page_link)
    }


    pub fn commit(&self, view: &TableView<'a, K>) -> BlobId {

        // Lock the backing store at the top of commit so we only commit one view (transaction) at a time
        let mut blob_store = self.backing_store.lock().unwrap();

        // Insert all new values into the committed b+tree
        let inserted_values = view.puts.borrow();
        for value in inserted_values.iter() {

            // NYI it's strange and wrong that we call view to get the mutable node... need to get it from ourselves
            let mutable_root_hpage = view.get_mutable_hpage(&self.root_page_link);
            match super::editor::insert_and_split(&mut view.get_mutable_page_deprecated(&mutable_root_hpage), *value, view) {
                SplitResult::Split(right_hpage) => {
                    let branch_page = create_branch_page(&mutable_root_hpage, right_hpage.clone(), view);
                    self.root_page_link.set_mutable(&branch_page);
                },
                SplitResult::NoSplit => {}
            };
        }

        // Remove all deleted values from the committed b+tree
        // NYI

        // Write the edited nodes to storage (if there are any)
        if self.root_page_link.is_mutable() {
            // NYI it's strange and wrong that we call view to get the mutable node... need to get it from ourselves
            let root_hpage = view.get_mutable_hpage(&self.root_page_link);
            let root_page = view.get_mutable_page_deprecated(&root_hpage);
            let root_blobid = root_page.store(&mut blob_store);

            // Rewrite the root node link
            // NYI bring back this commented out line
//            self.root_node_link.set_unloaded(root_blobid);
            self.root_blobid.replace(root_blobid);
        }

        // Return our root blobid, regardless of whether it changed or not
        *self.root_blobid.borrow()
    }


    pub(super) fn load(&self, node_link: &PageLink<K>) -> PageHandle<K> {
        unimplemented!()
    }
}
