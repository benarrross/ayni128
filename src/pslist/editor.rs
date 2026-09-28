use crate::pslist::Table;
use crate::pslist::TableView;
use crate::sortedarray::*;
use super::page::*;
use super::pagehandle::*;
use super::pagelink::*;


/// Inserts a value into a node and splits it if necessary.
pub fn insert_and_split<'a, const K:usize>(page: &mut Page<K>, value: u128, view: &TableView<'a, K>) -> SplitResult<K> {

    if page.is_leaf() {
        page.values.insert(value);

        if (page.values.len() > K) {
            SplitResult::Split(split_leaf_node(page))
        } else {
            SplitResult::NoSplit
        }
    }
    else {
        // Find the child this should go in and ask the child to insert the value
        let index = page.values.find_range_index(value);
        let mut mutable_child_hpage = page.get_mutable_child_hpage(index, view);

        // Handle the child splitting (which might force us to split the current node also)
        if let SplitResult::Split(right_hnode) = 
            insert_and_split(&mut &mut view.get_mutable_page_deprecated(&mutable_child_hpage), value, view) {

            let right_page = view.get_page_deprecated(&right_hnode);
            let first_value_in_right_page = right_page.first_value(view);

            page.values.insert(first_value_in_right_page);
            let new_child_index = page.values.find_range_index(first_value_in_right_page);
            page.children.as_mut().unwrap().insert(new_child_index, PageLink::new_mutable(&right_hnode));

            // Now see if we need to split
            if (page.values.len() > K) {
                SplitResult::Split(split_branch_node(page))
            } else {
                SplitResult::NoSplit
            }
        } else {
            SplitResult::NoSplit
        }
    }
}


/// Splits the right half of a leaf node off, and updates the leaf node linked list.
fn split_leaf_node<const K:usize>(page: &mut Page<K>) -> PageHandle<K> {
    
    // Make a new right node
    let split_index = page.values.len() / 2;
    let right_values = page.values.split_off(split_index);
    let new_right_hpage = PageHandle::new(Page::new_leaf(right_values, page.next_link.clone()));

    // Link the node we just split from to the new node in the leaf node linked list
    page.next_link = PageLink::new_mutable(&new_right_hpage);
    new_right_hpage
}


/// Splits the right half of a node off into a new branch node and returns it.
fn split_branch_node<const K:usize>(page: &mut Page<K>) -> PageHandle<K> {

    let split_index = page.values.len() / 2;
    let right_values = page.values.split_off(split_index + 1);
    page.values.pop();
    let right_children = page.children.as_mut().unwrap().split_off(split_index + 1);
    PageHandle::new(Page::new_branch(right_values, right_children))
}


/// Creates a new branch node from the specified left and right nodes
pub fn create_branch_page<'a, const K:usize>(
    left_hpage: &PageHandle<K>, right_hpage: PageHandle<K>, view: &TableView<'a, K>) -> PageHandle<K> {

    // Make a new parent node that has the old node on its left and the new node on its right
    let right_page = view.get_page_deprecated(&right_hpage);
    PageHandle::new(Page::new_branch(
        SortedArray::from_values(vec![right_page.first_value(view)]),
        vec![
            PageLink::new_mutable(&left_hpage),
            PageLink::new_mutable(&right_hpage) 
        ]))
}

