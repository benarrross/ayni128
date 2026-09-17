use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use xxhash_rust::xxh3::xxh3_64;
use crate::BlobStore;
use crate::PersistedSortedList;


static TREE_NODE_SIZE : usize = 512;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct StringId (pub u32);


impl From<u64> for StringId {
    fn from(hash: u64) -> StringId {
        let high = (hash & 0xFFFFFFFFu64 ) as u32;
        let low = (hash >> u32::BITS) as u32;
        StringId(high ^ low)
    } 
}

pub struct StringTable {
    cache_by_id: HashMap<StringId, Vec<u8>>,
    unsaved: Vec<StringId>,
    saved_strings_by_id: StoredStringsTable,
    blob_store: Arc<Mutex<BlobStore>>
}


impl StringTable {

    pub fn new(blob_store: Arc<Mutex<BlobStore>>) -> Self {
        StringTable {
            cache_by_id: HashMap::new(), 
            unsaved: Vec::new(), 
            saved_strings_by_id: StoredStringsTable::new(PersistedSortedList::new(blob_store.clone())), 
            blob_store: blob_store.clone() 
        }
    }

    pub fn map_to_id(& mut self, value: &[u8]) -> StringId {

        // NYI need to account for strings with duplicate hashes
        let id : StringId = xxh3_64(value).into();
        match self.cache_by_id.get(&id) {
            Some(_) => id,
            None => {
                self.cache_by_id.insert(id, value.to_vec());
                self.unsaved.push(id);
                id
            }
        }
    }


    pub fn get(&self, id: &StringId) -> Vec<u8> {
        match self.cache_by_id.get(&id) {
            Some(value) => value.to_vec(),
            None => {
                // NYI look up the value in our stored_strings_by_id table, which gives us the hash
                // use the hash to look up the offset in the stored_strings_by_hash table
                unimplemented!()
            }
        }
    }


    pub fn save(&self) {
        unimplemented!();
    }
}



struct StoredStringsTable {
    inner_table: PersistedSortedList<TREE_NODE_SIZE>
} 


impl<'a> StoredStringsTable {

    pub fn new(table: PersistedSortedList<TREE_NODE_SIZE>) -> Self {
        StoredStringsTable {
            inner_table: table
        }
    }

    pub fn get_view(&'a self) -> StoredStringsView<'a> {
        StoredStringsView::new(self.inner_table.get_view())
    }
}


struct StoredStringsView<'a> {
    pub inner_view: crate::pslist::ListView<'a, TREE_NODE_SIZE>
}


impl<'a> StoredStringsView<'a> {
    pub fn new(view: crate::pslist::ListView<'a, TREE_NODE_SIZE>) -> Self {
        StoredStringsView {
            inner_view: view
        }
    }
}
