use crate::pslist::*;
use super::graph::*;
use super::node::*;
use super::strings::StringId;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeName (StringId);


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
        (name.0.0 as u128) << NAME_BIT_INDEX |
        (value.0 as u128) << VALUE_BIT_INDEX
    }

    fn encode_for_enum_min(node: NodeId) -> u128 {
        (node.as_u32() as u128) << NODE_BIT_INDEX
    }

    fn encode_for_enum_mac(node: NodeId) -> u128 {
        ((node.as_u32() + 1) as u128) << NODE_BIT_INDEX
    }

    fn encode_for_get(node: NodeId, name: AttributeName) -> u128 {
        (node.as_u32() as u128) << NODE_BIT_INDEX |
        (name.0.0 as u128) << NAME_BIT_INDEX
    }

    fn decode(encoded: u128) -> Attribute {
        Attribute {
            node: (((encoded & NODE_MASK) >> NODE_BIT_INDEX) as u32).into(),
            name: (((encoded & NAME_MASK) >> NAME_BIT_INDEX) as u32).into(),
            value: StringId(((encoded & VALUE_MASK) >> VALUE_BIT_INDEX) as u32),
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
    }


    pub struct AttributesByNodeTableView<'a> {
        inner_view: crate::pslist::ListView<'a, TREE_NODE_SIZE>
    }

    impl<'a> AttributesByNodeTableView<'a> {
        pub fn new(view: crate::pslist::ListView<'a, TREE_NODE_SIZE>) -> Self {
            AttributesByNodeTableView {
                inner_view: view
            }
        }

        pub fn put(&self, node: NodeId, name: AttributeName, value: StringId) {
            self.inner_view.put(encode(node, name, value));
        }


        pub fn get(&self, node: NodeId, name: AttributeName) -> Attribute {
            let found_encoded = self.inner_view.get(encode_for_get(node, name));
            decode(found_encoded)
        }

        pub fn iter_attributes(&'a self, node: NodeId) -> AttributeByNodeIterator<'a> {  
            AttributeByNodeIterator::new(&self.inner_view, node)   
        }
    }


    pub struct AttributeByNodeIterator<'a> {
        inner_iter: ViewIterator<'a, TREE_NODE_SIZE>
    }

    impl <'a> AttributeByNodeIterator<'a> {
        pub fn new(based_on_view: &'a crate::pslist::ListView<TREE_NODE_SIZE>, node: NodeId) -> Self {
            let inner_iter = based_on_view.iter(
                encode_for_enum_min(node), encode_for_enum_mac(node));
            AttributeByNodeIterator { 
                inner_iter: inner_iter }

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


pub mod by_name {

    use crate::Table;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;
    
    static NODE_BIT_INDEX : usize = 32;
    static NAME_BIT_INDEX : usize = 96;
    static VALUE_BIT_INDEX : usize = 64;

    static NAME_MASK : u128 = 0xFFFFFFFF << NAME_BIT_INDEX;
    static VALUE_MASK : u128 = 0xFFFFFFFF << VALUE_BIT_INDEX;
    static NODE_MASK : u128 = 0xFFFFFFFF << NODE_BIT_INDEX;


    pub struct AttributesByNameTable {
        inner_table: Table<TREE_NODE_SIZE>
    } 


    impl<'a> AttributesByNameTable {

        pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
            AttributesByNameTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> AttributesByNameView<'a> {
            AttributesByNameView::new(self.inner_table.get_view())
        }
    }


    pub struct AttributesByNameView<'a> {
        inner_view: crate::pslist::ListView<'a, TREE_NODE_SIZE>
    }

    
    impl<'a> AttributesByNameView<'a> {
        pub fn new(view: crate::pslist::ListView<'a, TREE_NODE_SIZE>) -> Self {
            AttributesByNameView {
                inner_view: view
            }
        }

        pub fn put(&self, name: AttributeName, value: StringId, node: NodeId) {
            self.inner_view.put(Self::encode(name, value, node));
        }


        // pub fn iter_nodes_with_attribute(&self, name: AttributeName, value: StringId) -> AttrByNameValueIterator<'a> {
        //     unimplemented!();
        // }
        

        fn encode(name: AttributeName, value: StringId, node: NodeId) -> u128 {
            (name.0.0 as u128) << NAME_BIT_INDEX |
            (value.0 as u128) << VALUE_BIT_INDEX |
            (node.as_u32() as u128) << NODE_BIT_INDEX
        }


        fn decode(encoded: u128) -> Attribute {
            Attribute {
                node: (((encoded & NODE_MASK) >> NODE_BIT_INDEX) as u32).into(),
                name: (((encoded & NAME_MASK) >> NAME_BIT_INDEX) as u32).into(),
                value: StringId(((encoded & VALUE_MASK) >> VALUE_BIT_INDEX) as u32),
            }
        }
    }


    pub struct AttributeByNameValueIterator<'a> {
        based_on_view: &'a GraphView<'a>,
    }


    impl<'a> Iterator for AttributeByNameValueIterator<'a> {

        type Item = Attribute;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!();
        }   
    }

    
}

