use super::graph::*;
use super::view::*;


pub struct Edge {
    from: NodeId,
    to: NodeId,
    name: EdgeName,
    edge_type: EdgeType,
    order: EdgeOrder
}


pub mod edge_from {

    use crate::PersistedSortedList;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesFromTable {
        inner_table: PersistedSortedList<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesFromTable {

        pub fn new(table: PersistedSortedList<TREE_NODE_SIZE>) -> Self {
            EdgesFromTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> EdgesFromView<'a> {
            EdgesFromView::new(self.inner_table.get_view())
        }
    }

    
    pub struct EdgesFromView<'a> {
        inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
    }


    impl<'a> EdgesFromView<'a> {
        pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
            EdgesFromView {
                inner_view: view
            }
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

    use crate::PersistedSortedList;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesToTable {
        inner_table: PersistedSortedList<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesToTable {

        pub fn new(table: PersistedSortedList<TREE_NODE_SIZE>) -> Self {
            EdgesToTable {
                inner_table: table
            }
        }

        pub fn get_view(&'a self) -> EdgesToView<'a> {
            EdgesToView::new(self.inner_table.get_view())
        }
    }
    

    pub struct EdgesToView<'a> {
        inner_view: crate::bplustree::View<'a, TREE_NODE_SIZE>
    }


    impl<'a> EdgesToView<'a> {
        pub fn new(view: crate::bplustree::View<'a, TREE_NODE_SIZE>) -> Self {
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
