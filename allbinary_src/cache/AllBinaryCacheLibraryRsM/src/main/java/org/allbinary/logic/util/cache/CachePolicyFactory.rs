//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
// 
// By agreeing to this license you and any business entity you represent are
// legally bound to the AllBinary Open License Version 1 legal agreement.
// 
// You may obtain the AllBinary Open License Version 1 legal agreement from
// AllBinary or the root directory of AllBinary's AllBinary Platform repository.
// 
// Created By: Travis Berthelot
//  
// Generated Code Do Not Modify
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CachePolicy::CachePolicy;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CachePolicyFactory {
    pub NAME: String,
    pub MAX_TIME_THOUSAND_MAX: Option<Rc<RefCell<CachePolicy>>>,
    pub THIRTY_MINUTES_TEN_THOUSAND_MAX: Option<Rc<RefCell<CachePolicy>>>,
    pub THIRTY_MINUTES_FIFTY_THOUSAND_MAX: Option<Rc<RefCell<CachePolicy>>>,
    pub ONE_MINUTE_FIVE_MAX: Option<Rc<RefCell<CachePolicy>>>,
    pub ONE_MINUTE_TEN_MAX: Option<Rc<RefCell<CachePolicy>>>,
    pub ONE_MINUTE_ONE_HUNDRED_MAX: Option<Rc<RefCell<CachePolicy>>>,
}

thread_local! { pub static CachePolicyFactory_instance: Option<Rc<RefCell<CachePolicyFactory>>> = CachePolicyFactory::new(); }

impl CachePolicyFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            MAX_TIME_THOUSAND_MAX: None,
            THIRTY_MINUTES_TEN_THOUSAND_MAX: None,
            THIRTY_MINUTES_FIFTY_THOUSAND_MAX: None,
            ONE_MINUTE_FIVE_MAX: None,
            ONE_MINUTE_TEN_MAX: None,
            ONE_MINUTE_ONE_HUNDRED_MAX: None,
        };
        this.NAME = String::from("lru");
        this.MAX_TIME_THOUSAND_MAX = CachePolicy::new(this.NAME.clone(), i64::MAX.clone(), 1000);
        this.THIRTY_MINUTES_TEN_THOUSAND_MAX = CachePolicy::new(this.NAME.clone(), 1000 * 60 * 30, 10000);
        this.THIRTY_MINUTES_FIFTY_THOUSAND_MAX = CachePolicy::new(this.NAME.clone(), 1000 * 60 * 30, 100000);
        this.ONE_MINUTE_FIVE_MAX = CachePolicy::new(this.NAME.clone(), 1000 * 60 * 1, 5);
        this.ONE_MINUTE_TEN_MAX = CachePolicy::new(this.NAME.clone(), 1000 * 60 * 1, 10);
        this.ONE_MINUTE_ONE_HUNDRED_MAX = CachePolicy::new(this.NAME.clone(), 1000 * 60 * 1, 100);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CachePolicyFactory>>> {
        return CachePolicyFactory_instance.with(Clone::clone).clone();
    }
}

