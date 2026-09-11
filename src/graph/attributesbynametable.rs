use crate::BPlusTree;
use super::graph::*;
use super::attributebynodetable::Attribute;


pub struct AttributeByNameTable (pub BPlusTree<TREE_NODE_SIZE>); // NYI make this private



static NODE_BIT_INDEX : usize = 32;
static NAME_BIT_INDEX : usize = 96;
static VALUE_BIT_INDEX : usize = 64;

static NAME_MASK : u128 = 0xFFFFFFFF << NAME_BIT_INDEX;
static VALUE_MASK : u128 = 0xFFFFFFFF << VALUE_BIT_INDEX;
static NODE_MASK : u128 = 0xFFFFFFFF << NODE_BIT_INDEX;


pub fn encode(name: &AttributeName, value: &StringId, node: &NodeId) -> u128 {
    (name.0 as u128) << NAME_BIT_INDEX |
    (value.0 as u128) << VALUE_BIT_INDEX |
    (node.0 as u128) << NODE_BIT_INDEX
}


pub fn decode(encoded: &u128) -> Attribute {
    Attribute {
        node: NodeId(((*encoded & NODE_MASK) >> NODE_BIT_INDEX) as u32),
        name: AttributeName(((*encoded & NAME_MASK) >> NAME_BIT_INDEX) as u32),
        value: StringId(((*encoded & VALUE_MASK) >> VALUE_BIT_INDEX) as u32),
    }
}
