use std::sync::{Arc, RwLock};
use std::cell::*;
use super::page::*;


#[derive(Debug, Clone)]
pub(super) struct PageHandle<const K: usize> {
    pub node_debug_id: usize,               // NYI limit this to only exist when running tests
    node_lock: RefCell<Box<Page<K>>>         // NYI would this be faster as a Mutex? Or could this be a Box or RefCell?
}


impl<const K: usize> PageHandle<K> {
    
    pub fn new(node: Page<K>) -> Self {

        PageHandle {
            node_debug_id: node.debug_id,
            node_lock: RefCell::new(Box::new(node))
        }
    }

    pub fn read_lock_deprecated(&self) -> Ref<'_, Page<K>> {
        // Rust is an absolutely insane language. I have no idea why you need to use map just to get 
        // a reference to something on the heap. shoot me now.
        let borrow = self.node_lock.borrow();
        Ref::map(borrow, |boxed| boxed.as_ref())
    }

    pub fn write_lock_deprecated(&self) -> RefMut<'_, Page<K>> {
        // Rust is an absolutely insane language. I have no idea why you need to use map just to get 
        // a reference to something on the heap. shoot me now.
        let borrow = self.node_lock.borrow_mut();
        RefMut::map(borrow, |boxed| boxed.as_mut())
    }
}


