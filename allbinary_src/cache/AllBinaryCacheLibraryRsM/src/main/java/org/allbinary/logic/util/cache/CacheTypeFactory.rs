// Generated Code Do Not Modify
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::CacheType::CacheType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CacheTypeFactory {
    pub NO_CACHE: Option<Rc<RefCell<CacheType>>>,
    pub CACHE: Option<Rc<RefCell<CacheType>>>,
    pub SET: Option<Rc<RefCell<CacheType>>>,
    pub CACHE_VECTOR: Option<Rc<RefCell<CacheType>>>,
}

thread_local! { pub static CacheTypeFactory_instance: Option<Rc<RefCell<CacheTypeFactory>>> = CacheTypeFactory::new(); }

impl CacheTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_CACHE: None,
            CACHE: None,
            SET: None,
            CACHE_VECTOR: None,
        };
        this.NO_CACHE = CacheType::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        this.CACHE = CacheType::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        this.SET = CacheType::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), String::from("Set"));
        this.CACHE_VECTOR = CacheType::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CacheTypeFactory>>> {
        return CacheTypeFactory_instance.with(Clone::clone).clone();
    }
}

