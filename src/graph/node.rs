use crate::BPlusTree;
use super::graph::*;


pub struct NodesTable (pub BPlusTree<TREE_NODE_SIZE>); // NYI make this private


pub fn encode(node: &NodeId) -> u128 {
    node.0 as u128
}

pub fn decode(encoded: &u128) -> NodeId {
    NodeId(*encoded as u32)
}
