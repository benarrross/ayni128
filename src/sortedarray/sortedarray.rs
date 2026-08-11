#![allow(unused)]
use std::ops::{Index, IndexMut};


#[derive(Debug, Clone)]
pub struct SortedArray<T: Copy + Ord> {
    values : Vec<T>
}


impl<T: Copy + Ord> SortedArray<T> {

    pub fn new() -> Self {
        SortedArray { values: Vec::new() }
    }


    pub fn from_values(values: Vec<T>) -> Self {
        SortedArray { values: values }
    }

    
    pub fn insert(&mut self, value: T) {
        match self.values.binary_search(&value) {
            Ok(index) => {},
            Err(index) => {
                self.values.insert(index, value);
            },
        };        
    }


    pub fn remove(&mut self, value: T) {
        match self.values.binary_search(&value) {
            Ok(index) => self.values.remove(index),
            Err(index) => panic!("Attempting to remove value that does not exist"),
        };        
    }


    pub fn find_index(&self, value: T) -> usize {
        match self.values.binary_search(&value) {
            Ok(index) => index,
            Err(index) => index,
        }
    }


    pub fn get(&self, value: T) -> Option<T> {
        match self.values.binary_search(&value) {
            Ok(index) => Some(self.values[index]),
            Err(index) => None,
        }
    }


    pub fn split_off(&mut self, index: usize) -> SortedArray<T> {
        Self::from_values(self.values.split_off(index))
    }


    pub fn len(&self) -> usize { self.values.len() }
}


impl<T: Copy + Ord> Index<usize> for SortedArray<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { &self.values[index] }
}

