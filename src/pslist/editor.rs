use crate::pslist::Table;
use crate::pslist::TableView;
use crate::sortedarray::*;
use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;


/// Inserts a value into a node and splits it if necessary.
pub fn insert_and_split<'a, const K:usize>(node: &mut Node<K>, value: u128, table: &Table<K>, view: &TableView<'a, K>) -> SplitResult<K> {

    if node.is_leaf() {
        node.values.insert(value);

        if (node.values.len() > K) {
            SplitResult::Split(split_leaf_node(node))
        } else {
            SplitResult::NoSplit
        }
    }
    else {
        // Find the child this should go in and ask the child to insert the value
        let index = node.values.find_range_index(value);
        let mut mutable_child_hnode = node.get_mutable_child_hnode(index, table, view);

        // Handle the child splitting (which might force us to split the current node also)
        if let SplitResult::Split(right_hnode) = insert_and_split(&mut &mut view.get_mutable_node(&mutable_child_hnode), value, table, view) {

            let right_node = view.get_node(&right_hnode);
            let first_value_in_right_node = right_node.first_value(table, view);

            node.values.insert(first_value_in_right_node);
            let new_child_index = node.values.find_range_index(first_value_in_right_node);
            node.children.as_mut().unwrap().insert(new_child_index, NodeLink::new_mutable(&right_hnode));

            // Now see if we need to split
            if (node.values.len() > K) {
                SplitResult::Split(split_branch_node(node))
            } else {
                SplitResult::NoSplit
            }
        } else {
            SplitResult::NoSplit
        }
    }
}


/// Splits the right half of a leaf node off, and updates the leaf node linked list.
fn split_leaf_node<const K:usize>(node: &mut Node<K>) -> NodeHandle<K> {
    
    // Make a new right node
    let split_index = node.values.len() / 2;
    let right_values = node.values.split_off(split_index);
    let new_right_hnode = NodeHandle::new(Node::new_leaf(right_values, node.next_link.clone()));

    // Link the node we just split from to the new node in the leaf node linked list
    node.next_link = NodeLink::new_mutable(&new_right_hnode);
    new_right_hnode
}


/// Splits the right half of a node off into a new branch node and returns it.
fn split_branch_node<const K:usize>(node: &mut Node<K>) -> NodeHandle<K> {

    let split_index = node.values.len() / 2;
    let right_values = node.values.split_off(split_index + 1);
    node.values.pop();
    let right_children = node.children.as_mut().unwrap().split_off(split_index + 1);
    NodeHandle::new(Node::new_branch(right_values, right_children))
}


/// Creates a new branch node from the specified left and right nodes
pub fn create_branch_node<'a, const K:usize>(left_hnode: &NodeHandle<K>, right_hnode: NodeHandle<K>, table: &Table<K>, view: &TableView<'a, K>) -> NodeHandle<K> {

    // Make a new parent node that has the old node on its left and the new node on its right
    let right_node = view.get_node(&right_hnode);
    NodeHandle::new(Node::new_branch(
        SortedArray::from_values(vec![right_node.first_value(table, view)]),
        vec![
            NodeLink::new_mutable(&left_hnode),
            NodeLink::new_mutable(&right_hnode) 
        ]))
}

