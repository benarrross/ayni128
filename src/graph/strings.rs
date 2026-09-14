use crate::PersistedSortedList;
use super::graph::*;
use xxhash_rust::const_xxh3::xxh3_64 as const_xxh3;
use xxhash_rust::xxh3::xxh3_64;

/*

OPERATIONS:
set(&[u8]) -> StringId
get(StringId) -> &[u8]

should I maintain a vec[u8] in memory for the strong contents, and serialize it into a blob when commit happens?
I would also need a HashMap from StringId to the right slice of that

For serialization, I could keep a list of strings awaiting serialization. Or I could add them to the blobstore immediately
when the set happens. Then I also need a table that maps from StringId to blob offset.



*/

pub struct StringsTable {
    inner_table: PersistedSortedList<TREE_NODE_SIZE>
} 


impl<'a> StringsTable {

    pub fn new(table: PersistedSortedList<TREE_NODE_SIZE>) -> Self {
        StringsTable {
            inner_table: table
        }
    }

    pub fn get_view(&'a self) -> StringsView<'a> {
        StringsView::new(self.inner_table.get_view())
    }
}


pub struct StringsView<'a> {
    pub inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
}


impl<'a> StringsView<'a> {
    pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
        StringsView {
            inner_view: view
        }
    }

    pub fn put(&self, node: &NodeId) {
        self.inner_view.put(Self::encode(&node));
    }

    
    fn encode(node: &NodeId) -> u128 {
        node.0 as u128
    }

    
    fn decode(encoded: &u128) -> NodeId {
        NodeId(*encoded as u32)
    }
}

