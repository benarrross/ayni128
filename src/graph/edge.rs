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

    use super::Edge;
    use crate::graph::view::*;


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

    use super::Edge;
    use crate::graph::view::*;

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
