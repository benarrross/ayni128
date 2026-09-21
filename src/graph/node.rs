use crate::Table;
use super::graph::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NodeId (u32);

impl NodeId {
    pub(crate) fn new(value: u32) -> Self {
        NodeId { 0: value }
    }

    pub(crate) fn as_u32(&self) -> u32 { 
        self.0
    }
}


impl From<NodeId> for u32 {
    fn from(item: NodeId) -> u32 {
        item.0
    }
}


impl From<u32> for NodeId {
    fn from(item: u32) -> NodeId {
        NodeId(item)
    }
}


pub struct NodesTable {
    inner_table: Table<TREE_NODE_SIZE>
} 


impl<'a> NodesTable {

    pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
        NodesTable {
            inner_table: table
        }
    }

    pub fn get_view(&'a self) -> NodesView<'a> {
        NodesView::new(self.inner_table.get_view())
    }


    pub fn commit(&self, view: &'a NodesView) {
        self.inner_table.commit(&view.inner_view);
    }
}


pub struct NodesView<'a> {
    pub inner_view: crate::pslist::TableView<'a, TREE_NODE_SIZE>
}


impl<'a> NodesView<'a> {
    pub fn new(view: crate::pslist::TableView<'a, TREE_NODE_SIZE>) -> Self {
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

