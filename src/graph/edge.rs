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


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    Child = 0,
    Reference = 1,
}


pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub name: EdgeName,
    pub edge_type: EdgeType,
    pub order: EdgeOrder
}


pub mod edge_from {

    use crate::Table;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesFromTable {
        inner_table: Table<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesFromTable {

        pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
            EdgesFromTable {
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
            unimplemented!();
            //self.inner_view.insert(encode(node, name, value));
        }


        pub fn get(&self, node: NodeId, name: EdgeName) -> Option<Edge> {
            unimplemented!();
        //     let found_encoded = self.inner_view.get(encode_for_get(node, name));
        //     decode(found_encoded)
        }


        pub fn iter_attributes(&'a self, node: NodeId) -> EdgeFromIterator<'a> {
            unimplemented!();
//            AttributeByNodeIterator::new(&self.inner_view, node)   
        }
    }


    pub struct EdgeFromIterator<'a> {
        based_on_view: &'a GraphView<'a>,
    }


    impl<'a> Iterator for EdgeFromIterator<'a> {

        type Item = Edge;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!();
        }   
    }
}


pub mod edge_to {

    use crate::Table;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesToTable {
        inner_table: Table<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesToTable {

        pub fn new(table: Table<TREE_NODE_SIZE>) -> Self {
            EdgesToTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> EdgesToView<'a> {
            EdgesToView::new(self.inner_table.get_view())
        }


        pub fn commit(&self, view: &'a EdgesToView) {
            self.inner_table.commit(&view.inner_view);
        }
    }
    

    pub struct EdgesToView<'a> {
        inner_view: crate::pslist::TableView<'a, TREE_NODE_SIZE>
    }


    impl<'a> EdgesToView<'a> {
        pub fn new(view: crate::pslist::TableView<'a, TREE_NODE_SIZE>) -> Self {
            EdgesToView {
                inner_view: view
            }
        }
    }


    pub struct EdgeToIterator<'a> {
        based_on_view: &'a GraphView<'a>,
    }


    impl<'a> Iterator for EdgeToIterator<'a> {

        type Item = Edge;

        fn next(&mut self) -> Option<Self::Item> {
            unimplemented!();
        }   
    }
}
