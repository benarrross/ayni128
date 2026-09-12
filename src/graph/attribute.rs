use crate::BPlusTree;
use super::graph::*;


pub struct Attribute {
    pub node : NodeId,
    pub name : AttributeName,
    pub value : StringId
}


pub mod by_node {

    use crate::BPlusTree;
    use crate::graph::graph::*;
    use super::*;

    static NODE_BIT_INDEX : usize = 96;
    static NAME_BIT_INDEX : usize = 64;
    static VALUE_BIT_INDEX : usize = 32;

    static NODE_MASK : u128 = 0xFFFFFFFF << NODE_BIT_INDEX;
    static NAME_MASK : u128 = 0xFFFFFFFF << NAME_BIT_INDEX;
    static VALUE_MASK : u128 = 0xFFFFFFFF << VALUE_BIT_INDEX;


    pub struct AttributeByNodeTable {
        inner_table: BPlusTree<TREE_NODE_SIZE>
    } 


    impl<'a> AttributeByNodeTable {

        pub fn new(table: BPlusTree<TREE_NODE_SIZE>) -> Self {
            AttributeByNodeTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> AttributeByNodeView<'a> {
            AttributeByNodeView::new(self.inner_table.get_view())
        }
    }


    pub struct AttributeByNodeView<'a> {
        inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
    }


    impl<'a> AttributeByNodeView<'a> {
        pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
            AttributeByNodeView {
                inner_view: view
            }
        }

        pub fn put(&self, node: &NodeId, name: &AttributeName, value: &StringId) {
            self.inner_view.put(Self::encode(&node, &name, &value));
        }


        pub fn get(&self, node: &NodeId, name: &AttributeName) -> Attribute {
            let found_encoded = self.inner_view.get(Self::encode_for_get(&node, &name));
            Self::decode(&found_encoded)
        }


        fn encode(node: &NodeId, name: &AttributeName, value: &StringId) -> u128 {
            (node.0 as u128) << NODE_BIT_INDEX |
            (name.0 as u128) << NAME_BIT_INDEX |
            (value.0 as u128) << VALUE_BIT_INDEX
        }


        fn encode_for_get(node: &NodeId, name: &AttributeName) -> u128 {
            (node.0 as u128) << NODE_BIT_INDEX |
            (name.0 as u128) << NAME_BIT_INDEX
        }


        fn decode(encoded: &u128) -> Attribute {
            Attribute {
                node: NodeId(((*encoded & NODE_MASK) >> NODE_BIT_INDEX) as u32),
                name: AttributeName(((*encoded & NAME_MASK) >> NAME_BIT_INDEX) as u32),
                value: StringId(((*encoded & VALUE_MASK) >> VALUE_BIT_INDEX) as u32),
            }
        }

    }
}


pub mod by_name {

    use crate::BPlusTree;
    use crate::graph::graph::*;
    use super::*;
    
    static NODE_BIT_INDEX : usize = 32;
    static NAME_BIT_INDEX : usize = 96;
    static VALUE_BIT_INDEX : usize = 64;

    static NAME_MASK : u128 = 0xFFFFFFFF << NAME_BIT_INDEX;
    static VALUE_MASK : u128 = 0xFFFFFFFF << VALUE_BIT_INDEX;
    static NODE_MASK : u128 = 0xFFFFFFFF << NODE_BIT_INDEX;


    pub struct AttributeByNameTable {
        inner_table: BPlusTree<TREE_NODE_SIZE>
    } 


    impl<'a> AttributeByNameTable {

        pub fn new(table: BPlusTree<TREE_NODE_SIZE>) -> Self {
            AttributeByNameTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> AttributeByNameView<'a> {
            AttributeByNameView::new(self.inner_table.get_view())
        }
    }


    pub struct AttributeByNameView<'a> {
        inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
    }

    
    impl<'a> AttributeByNameView<'a> {
        pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
            AttributeByNameView {
                inner_view: view
            }
        }

        pub fn put(&self, name: &AttributeName, value: &StringId, node: &NodeId) {
            self.inner_view.put(Self::encode(&name, &value, &node));
        }


        // pub fn iter_nodes_with_attribute(&self, name: AttributeName, value: StringId) -> AttrByNameValueIterator<'a> {
        //     unimplemented!();
        // }
        

        fn encode(name: &AttributeName, value: &StringId, node: &NodeId) -> u128 {
            (name.0 as u128) << NAME_BIT_INDEX |
            (value.0 as u128) << VALUE_BIT_INDEX |
            (node.0 as u128) << NODE_BIT_INDEX
        }


        fn decode(encoded: &u128) -> Attribute {
            Attribute {
                node: NodeId(((*encoded & NODE_MASK) >> NODE_BIT_INDEX) as u32),
                name: AttributeName(((*encoded & NAME_MASK) >> NAME_BIT_INDEX) as u32),
                value: StringId(((*encoded & VALUE_MASK) >> VALUE_BIT_INDEX) as u32),
            }
        }
    }
}

