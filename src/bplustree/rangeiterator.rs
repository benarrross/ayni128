#![allow(unused)]
use std::iter::Map;
use std::io::{SeekFrom, prelude::*};
use std::iter::{*};
use std::sync::{Arc, RwLock};
use std::rc::Rc;

use crate::BlobId;

use super::bplustree::*;
use super::view::*;
use super::node::*;


pub struct RangeCollection<'a, const K: usize> {
    based_on: &'a View<'a, K>,
    root_node: LoadedNodeRef<K>,
    min: u128,
    mac: u128
}


impl<'a, const K: usize> RangeCollection<'a,  K> {

    pub fn new(based_on: &'a View<'a, K>, root_node: LoadedNodeRef<K>, min: u128, mac: u128) -> Self {
        RangeCollection { 
            based_on, 
            root_node: root_node, 
            min: min, 
            mac: mac  }
    }
}


impl<'a, const K: usize> IntoIterator for RangeCollection<'a, K> {
    type Item = u128;
    type IntoIter = RangeIterator<'a, K>;
    fn into_iter(self) -> Self::IntoIter { RangeIterator::new(self.based_on, self.root_node, self.min, self.mac) }
}


pub struct RangeIterator<'a, const K: usize> {
    based_on: &'a View<'a, K>,
    root_node: LoadedNodeRef<K>,
    min: u128,
    mac: u128,
    current_node: Option<LoadedNodeRef<K>>,
    index: usize
}

impl<'a, const K: usize> RangeIterator<'a,  K> {

    pub fn new(based_on: &'a View<'a, K>, root_node: LoadedNodeRef<K>, min: u128, mac: u128) -> Self {
        RangeIterator { 
            based_on, 
            root_node: root_node.clone(), 
            min: min, 
            mac: mac,
            current_node: None,
            index: 0  }
    }


    fn find_first(&mut self) -> Option<u128> {

        self.current_node = Option::Some(self.root_node.clone());
        let nodelink = self.current_node.as_ref().map(|nodelink| nodelink);
        let node = Option::expect(nodelink, "foo").read().unwrap();
        
        self.index = node.values.find(self.min);
        
        if self.index < node.values.len() {
            Option::Some(node.values[self.index])
        }
        else {
            Option::None
        }
    }

    fn find_next(&mut self) -> Option<u128> {

        let nodelink = self.current_node.as_ref().map(|nodelink| nodelink);
        let node = Option::expect(nodelink, "foo").read().unwrap();

        self.index = self.index + 1;        
        if self.index < node.values.len() {
            Option::Some(node.values[self.index])
        }
        else {
            Option::None
        }
    }


}


impl<'a, const K: usize> Iterator for RangeIterator<'a, K> {

    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {

         match &self.current_node {
            None => self.find_first(),
            Some(node) => self.find_next()
        }
    }
}

