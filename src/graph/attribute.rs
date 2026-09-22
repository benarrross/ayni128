use crate::pslist::*;
use super::graph::*;
use super::node::*;
use super::strings::StringId;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeName (StringId);

impl AttributeName {
    pub(crate) fn as_u32(&self) -> u32 { 
        self.0.as_u32()
    }
}

impl From<AttributeName> for u32 { fn from(item: AttributeName) -> u32 { item.0.0 } }
impl From<AttributeName> for StringId { fn from(item: AttributeName) -> StringId { item.0 } }
impl From<StringId> for AttributeName { fn from(item: StringId) -> AttributeName { AttributeName(item) } }
impl From<u32> for AttributeName { fn from(item: u32) -> AttributeName { AttributeName(StringId(item)) } }


pub struct Attribute {
    pub node : NodeId,
    pub name : AttributeName,
    pub value : StringId
}


pub mod by_node {

    use crate::Table;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;

    static NODE_BIT_INDEX : usize = 96;
    static NAME_BIT_INDEX : usize = 64;
    static VALUE_BIT_INDEX : usize = 32;

    static NODE_MASK : u128 = 0xFFFFFFFFu128 << NODE_BIT_INDEX;
    static NAME_MASK : u128 = 0xFFFFFFFFu128 << NAME_BIT_INDEX;
    static VALUE_MASK : u128 = 0xFFFFFFFFu128 << VALUE_BIT_INDEX;


    fn encode(node: NodeId, name: AttributeName, value: StringId) -> u128 {
        (node.as_u32() as u128) << NODE_BIT_INDEX |
        (name.as_u32() as u128) << NAME_BIT_INDEX |
        (value.as_u32() as u128) << VALUE_BIT_INDEX
    }

    fn encode_for_enum_min(node: NodeId) -> u128 {
        (node.as_u32() as u128) << NODE_BIT_INDEX
    }

    fn encode_for_enum_mac(node: NodeId) -> u128 {
        ((node.as_u32() + 1) as u128) << NODE_BIT_INDEX
    }

    fn encode_for_get(node: NodeId, name: AttributeName) -> u128 {
        (node.as_u32() as u128) << NODE_BIT_INDEX |
        (name.as_u32() as u128) << NAME_BIT_INDEX
    }

    fn decode(encoded: u128) -> Attribute {
        Attribute {
            node: ((((encoded & NODE_MASK) >> NODE_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
            name: ((((encoded & NAME_MASK) >> NAME_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
            value: StringId((((encoded & VALUE_MASK) >> VALUE_BIT_INDEX) & 0xFFFFFFFF) as u32),
        }
    }


    pub struct AttributesByNodeTable {
        inner_table: Table<TREE_NODE_SIZE>
    } 

    impl<'a> AttributesByNodeTable {

        pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
            AttributesByNodeTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> AttributesByNodeTableView<'a> {
            AttributesByNodeTableView::new(self.inner_table.get_view())
        }


        pub fn commit(&self, view: &'a AttributesByNodeTableView) {
            self.inner_table.commit(&view.inner_view);
        }
    }


    pub struct AttributesByNodeTableView<'a> {
        inner_view: crate::pslist::TableView<'a, TREE_NODE_SIZE>
    }

    impl<'a> AttributesByNodeTableView<'a> {
        pub fn new(view: crate::pslist::TableView<'a, TREE_NODE_SIZE>) -> Self {
            AttributesByNodeTableView {
                inner_view: view
            }
        }


        pub fn insert(&self, node: NodeId, name: AttributeName, value: StringId) {
            self.inner_view.insert(encode(node, name, value));
        }


        pub fn get(&self, node: NodeId, name: AttributeName) -> Option<Attribute> {
            let found = decode(self.inner_view.get(encode_for_get(node, name)));
            if (found.node == node && found.name == name) {
                Some(found)
            } else {
                None
            }
        }

        
        pub fn iter(&'a self, node: NodeId) -> AttributeByNodeIterator<'a> {
            AttributeByNodeIterator::new(&self.inner_view, node)   
        }
    }


    pub struct AttributeByNodeIterator<'a> {
        inner_iter: TableIterator<'a, TREE_NODE_SIZE>
    }


    impl <'a> AttributeByNodeIterator<'a> {
        pub fn new(based_on_view: &'a crate::pslist::TableView<TREE_NODE_SIZE>, node: NodeId) -> Self {
            let inner_iter = based_on_view.iter(
                encode_for_enum_min(node), encode_for_enum_mac(node));
            AttributeByNodeIterator { inner_iter }
        }
    }


    impl<'a> Iterator for AttributeByNodeIterator<'a> {

        type Item = Attribute;

        fn next(&mut self) -> Option<Self::Item> {
            match self.inner_iter.next() {
                Some(encoded) => Some(decode(encoded)),
                None => None
            }
        }   
    }
}


pub mod by_attr {

    use crate::Table;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;
    
    static NAME_BIT_INDEX : usize = 96;
    static VALUE_BIT_INDEX : usize = 64;
    static NODE_BIT_INDEX : usize = 32;

    static NAME_MASK : u128 = 0xFFFFFFFF << NAME_BIT_INDEX;
    static VALUE_MASK : u128 = 0xFFFFFFFF << VALUE_BIT_INDEX;
    static NODE_MASK : u128 = 0xFFFFFFFF << NODE_BIT_INDEX;


    fn encode(name: AttributeName, value: StringId, node: NodeId) -> u128 {
        (name.as_u32() as u128) << NAME_BIT_INDEX |
        (value.as_u32() as u128) << VALUE_BIT_INDEX |
        (node.as_u32() as u128) << NODE_BIT_INDEX
    }

    fn encode_for_enum_min(name: AttributeName, value: StringId, ) -> u128 {
        (name.as_u32() as u128) << NAME_BIT_INDEX |
        (value.as_u32() as u128) << VALUE_BIT_INDEX
    }

    fn encode_for_enum_mac(name: AttributeName, value: StringId, ) -> u128 {
        (name.as_u32() as u128) << NAME_BIT_INDEX |
        ((value.as_u32() + 1) as u128) << VALUE_BIT_INDEX
    }

    fn decode(encoded: u128) -> Attribute {
        Attribute {
            node: ((((encoded & NODE_MASK) >> NODE_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
            name: ((((encoded & NAME_MASK) >> NAME_BIT_INDEX) & 0xFFFFFFFF) as u32).into(),
            value: StringId((((encoded & VALUE_MASK) >> VALUE_BIT_INDEX) & 0xFFFFFFFF) as u32),
        }
    }

    fn decode_node(encoded: u128) -> NodeId {
        ((((encoded & NODE_MASK) >> NODE_BIT_INDEX) & 0xFFFFFFF) as u32).into()
    }

    
    pub struct NodesByAttributeTable {
        inner_table: Table<TREE_NODE_SIZE>
    } 


    impl<'a> NodesByAttributeTable {

        pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
            NodesByAttributeTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> NodesByAttributeTableView<'a> {
            NodesByAttributeTableView::new(self.inner_table.get_view())
        }


        pub fn commit(&self, view: &'a NodesByAttributeTableView) {
            self.inner_table.commit(&view.inner_view);
        }
    }


    pub struct NodesByAttributeTableView<'a> {
        inner_view: crate::pslist::TableView<'a, TREE_NODE_SIZE>
    }

    
    impl<'a> NodesByAttributeTableView<'a> {
        pub fn new(view: crate::pslist::TableView<'a, TREE_NODE_SIZE>) -> Self {
            NodesByAttributeTableView {
                inner_view: view
            }
        }

        pub fn insert(&self, name: AttributeName, value: StringId, node: NodeId) {
            self.inner_view.insert(encode(name, value, node));
        }


        pub fn iter(&'a self, name: AttributeName, value: StringId) -> NodeByAttributeIterator<'a> {
            NodeByAttributeIterator::new(&self.inner_view, name, value)   
        }
    }


    pub struct NodeByAttributeIterator<'a> {
        inner_iter: TableIterator<'a, TREE_NODE_SIZE>
    }


    impl <'a> NodeByAttributeIterator<'a> {
        pub fn new(based_on_view: &'a crate::pslist::TableView<TREE_NODE_SIZE>, name: AttributeName, value: StringId) -> Self {
            let inner_iter = based_on_view.iter(
                encode_for_enum_min(name, value), encode_for_enum_mac(name, value));
            NodeByAttributeIterator { inner_iter }
        }
    }


    impl<'a> Iterator for NodeByAttributeIterator<'a> {

        type Item = NodeId;

        fn next(&mut self) -> Option<Self::Item> {
            let x= self.inner_iter.next(); 
            match x {
                Some(encoded) => Some(decode_node(encoded)),
                None => None
            }
        }   
    }
}
