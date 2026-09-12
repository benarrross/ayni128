use crate::BPlusTree;
use super::graph::*;


pub struct NodesTable {
    inner_table: BPlusTree<TREE_NODE_SIZE>
} 


impl<'a> NodesTable {

    pub fn new(table: BPlusTree<TREE_NODE_SIZE>) -> Self {
        NodesTable {
            inner_table: table
        }
    }

    pub fn get_view(&'a self) -> NodesView<'a> {
        NodesView::new(self.inner_table.get_view())
    }
}


pub struct NodesView<'a> {
    pub inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
}


impl<'a> NodesView<'a> {
    pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
        NodesView {
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

