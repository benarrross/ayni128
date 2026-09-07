use super::graph::NodeId;


pub fn encode(node: &NodeId) -> u128 {
    node.0 as u128
}

pub fn decode(encoded: &u128) -> NodeId {
    NodeId(*encoded as u32)
}
