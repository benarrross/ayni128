use super::node::*;
use super::nodehandle::*;
use super::nodelink::*;


/// Inserts a value into a node and splits it if necessary.
pub fn insert_and_split<const K:usize>(node: &mut Node<K>, value : u128, node_store: &dyn NodeStore<K>) -> SplitResult<K> {
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
        let mut mutable_child_hnode = &node.children.as_ref().unwrap()[index].get_mutable(node_store);

        // Handle the child splitting (which might force us to split the current node also)
        if let SplitResult::Split(right_hnode) = insert_and_split(&mut mutable_child_hnode.write_lock(), value, node_store) {

            let right_node = &*right_hnode.read_lock();
            let first_value_in_right_node = right_node.values[0];  // NYI This is wrong for branch nodes -- need a node.first_value() method

            node.values.insert(first_value_in_right_node);
            let new_child_index = node.values.find_range_index(first_value_in_right_node);
            node.children.as_mut().unwrap().insert(new_child_index, NodeLink::mutable(right_hnode.clone()));

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
    let new_right_hnode = Node::new_leaf(right_values, node.next_link.clone());

    // Link the node we just split from to the new node in the leaf node linked list
    node.next_link = NodeLink::mutable(new_right_hnode.clone());
    new_right_hnode
}


/// Splits the right half of a node off into a new branch node and returns it.
fn split_branch_node<const K:usize>(node: &mut Node<K>) -> NodeHandle<K> {
    let split_index = node.values.len() / 2;
    let right_values = node.values.split_off(split_index);
    let right_children = node.children.as_mut().unwrap().split_off(split_index);
    Node::new_branch(right_values, right_children)
}
