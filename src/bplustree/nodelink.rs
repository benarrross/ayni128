#![allow(unused)]
use std::cell::RefCell;
use std::fmt;
use std::mem;
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::bplustree::node::SplitResult::NoSplit;
use crate::{blobstore::*, sortedarray::*};

use super::bplustree::*;
use super::nodehandle::*;


// Node that may or may not be loaded into memory yet
#[derive(Debug, Clone)]
pub enum NodeLink<const K: usize> {
    Empty,
    Unloaded(BlobId),
    Loaded(NodeHandle<K>),
    Edited(NodeHandle<K>)
}


