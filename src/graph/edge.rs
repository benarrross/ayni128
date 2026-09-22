use crate::Table;
use super::graph::*;
use super::node::NodeId;
use super::strings::StringId;
use super::view::*;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdgeName (StringId);

impl EdgeName {
    pub(crate) fn as_u32(&self) -> u32 { 
        self.0.as_u32()
    }
}

impl From<EdgeName> for u32 { fn from(item: EdgeName) -> u32 { item.0.0 } }
impl From<EdgeName> for StringId { fn from(item: EdgeName) -> StringId { item.0 } }
impl From<StringId> for EdgeName { fn from(item: StringId) -> EdgeName { EdgeName(item) } }
impl From<u32> for EdgeName { fn from(item: u32) -> EdgeName { EdgeName(StringId(item)) } }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct EdgeOrder (u32);

impl EdgeOrder {
    pub(crate) fn as_u32(&self) -> u32 { 
        self.0
    }
}
impl From<EdgeOrder> for u32 { fn from(item: EdgeOrder) -> u32 { item.0 } }
impl From<u32> for EdgeOrder { fn from(item: u32) -> EdgeOrder { EdgeOrder(item) } }


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Child = 0,
    Reference = 1,
}

//impl From<EdgeType> for u32 { fn from(item: EdgeType) -> u32 { item.0.0 } }
impl From<u32> for EdgeType { 
    fn from(item: u32) -> EdgeType {
        if (item == 0) {
            EdgeType::Child 
        } else  {
            EdgeType::Reference 
        } 
    }
}


pub struct Edge {
    pub from: NodeId,
    pub edge_type: EdgeType,
    pub name: EdgeName,
    pub order: EdgeOrder,
    pub to: NodeId
}


static FROM_BIT_INDEX : usize = 96;
static TYPE_BIT_INDEX : usize = 64;
static NAME_BIT_INDEX : usize = 63;
static ORDER_BIT_INDEX: usize = 31;
static TO_BIT_INDEX: usize = 0;

static FROM_MASK : u128 = 0xFFFFFFFFu128 << FROM_BIT_INDEX;
static NAME_MASK : u128 = 0xFFFFFFFFu128 << NAME_BIT_INDEX;
static TYPE_MASK : u128 = 0x1u128 << TYPE_BIT_INDEX;
static ORDER_MASK : u128 = 0x7FFFFFFFu128 << ORDER_BIT_INDEX;
static TO_MASK : u128 = 0xFFFFFFFFu128 << TO_BIT_INDEX;


fn encode(from: NodeId, name: EdgeName, edge_type: EdgeType, to: NodeId, order: EdgeOrder) -> u128 {
    (from.as_u32() as u128) << FROM_MASK |
    (edge_type as u32 as u128) << TYPE_BIT_INDEX |
    (name.as_u32() as u128) << NAME_BIT_INDEX |
    (order.as_u32() as u128) << ORDER_BIT_INDEX |
    (to.as_u32() as u128) << TO_BIT_INDEX
}

// fn encode_for_enum_min(node: NodeId) -> u128 {
//     (node.as_u32() as u128) << NODE_BIT_INDEX
// }

// fn encode_for_enum_mac(node: NodeId) -> u128 {
//     ((node.as_u32() + 1) as u128) << NODE_BIT_INDEX
// }

fn encode_for_get(from: NodeId, edge_type: EdgeType, name: EdgeName) -> u128 {
    (from.as_u32() as u128) << FROM_MASK |
    (edge_type as u32 as u128) << TYPE_BIT_INDEX |
    (name.as_u32() as u128) << NAME_BIT_INDEX
}


fn decode(encoded: u128) -> Edge {
    Edge {
        from: ((((encoded & FROM_MASK) >> FROM_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
        edge_type: ((((encoded & TYPE_MASK) >> TYPE_BIT_INDEX) & 0x1) as u32).into(),
        name: ((((encoded & NAME_MASK) >> NAME_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
        order: ((((encoded & ORDER_MASK) >> ORDER_BIT_INDEX) & 0x7FFFFFFF) as u32).into(),
        to: ((((encoded & TO_MASK) >> TO_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
    }
}


pub struct EdgesTable {
    inner_table: Table<TREE_NODE_SIZE>
} 


impl<'a> EdgesTable {

    pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
        EdgesTable {
            inner_table: table
        }
    }

    pub fn get_view(&'a self) -> EdgesFromView<'a> {
        EdgesFromView::new(self.inner_table.get_view())
    }


    pub fn commit(&self, view: &'a EdgesFromView) {
        self.inner_table.commit(&view.inner_view);
    }
}


pub struct EdgesFromView<'a> {
    inner_view: crate::pslist::TableView<'a, TREE_NODE_SIZE>
}


impl<'a> EdgesFromView<'a> {
    pub fn new(view: crate::pslist::TableView<'a, TREE_NODE_SIZE>) -> Self {
        EdgesFromView {
            inner_view: view
        }
    }

    pub fn insert(&self, from: NodeId, edge_type: EdgeType, name: EdgeName, to: NodeId, order: EdgeOrder) {
        self.inner_view.insert(encode(from, name, edge_type, to, order));
    }


    pub fn get(&self, from: NodeId, edge_type: EdgeType, name: EdgeName) -> Option<Edge> {
            let found = decode(self.inner_view.get(encode_for_get(from, edge_type, name)));
            if (found.from == from && found.edge_type == edge_type && found.name == name) {
            Some(found)
            } else {
                None
            }
    }


    pub fn iter_attributes(&'a self, node: NodeId) -> EdgeIterator<'a> {
        unimplemented!();
    }
}


pub struct EdgeIterator<'a> {
    based_on_view: &'a GraphView<'a>,
}


impl<'a> Iterator for EdgeIterator<'a> {

    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!();
    }   
}
