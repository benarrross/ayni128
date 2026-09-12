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

    use crate::BPlusTree;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesFromTable {
        pub inner_table: BPlusTree<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesFromTable {

        pub fn new(table: BPlusTree<TREE_NODE_SIZE>) -> Self {
            EdgesFromTable {
                inner_table: table
            }
        }

        // pub fn get_view(&'a self) -> NodesView<'a> {
        //     NodesView::new(self.inner_table.get_view())
        // }
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

    use crate::BPlusTree;
    use crate::graph::graph::*;
    use crate::graph::view::*;
    use super::*;


    pub struct EdgesToTable {
        pub inner_table: BPlusTree<TREE_NODE_SIZE>
    } 


    impl<'a> EdgesToTable {

        pub fn new(table: BPlusTree<TREE_NODE_SIZE>) -> Self {
            EdgesToTable {
                inner_table: table
            }
        }

        // pub fn get_view(&'a self) -> NodesView<'a> {
        //     NodesView::new(self.inner_table.get_view())
        // }
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
