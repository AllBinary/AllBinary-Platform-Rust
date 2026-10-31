// Generated Code Do Not Modify
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::PoolType::PoolType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PoolTypeFactory {
    pub VECTOR_POOL: Option<Rc<RefCell<PoolType>>>,
}

thread_local! { pub static PoolTypeFactory_instance: Option<Rc<RefCell<PoolTypeFactory>>> = PoolTypeFactory::new(); }

impl PoolTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            VECTOR_POOL: None,
        };
        this.VECTOR_POOL = PoolType::new(jcall!(CacheTypeFactory::CacheTypeFactory::getInstance()).NO_CACHE.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PoolTypeFactory>>> {
        return PoolTypeFactory_instance.with(Clone::clone).clone();
    }
}

