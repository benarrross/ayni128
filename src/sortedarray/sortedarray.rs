#![allow(unused)]
use std::ops::{Index, IndexMut};


#[derive(Debug, Clone)]
pub struct SortedArray<T: Ord> {
    values : Vec<T>
}


impl<T: Ord> SortedArray<T> {

    pub fn new() -> Self {
        SortedArray { values: Vec::new() }
    }


    pub fn insert(&mut self, value: T) {

        match self.values.binary_search(&value) {
            Ok(index) => {},
            Err(index) => {
                self.values.insert(index, value);
            },
        };        
    }


    pub fn find(&self, value: T) -> usize {

        match self.values.binary_search(&value) {
            Ok(index) => index,
            Err(index) => index,
        }
    }


    pub fn len(&self) -> usize { self.values.len() }
}


impl<T: Ord> Index<usize> for SortedArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.values[index] }
}

