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
// package org.allbinary.business.advertisement.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementSearchData {
    pub TITLE: String,
    pub PRICE: String,
    pub RELEVANCY: String,
    pub POPULARITY: String,
}

thread_local! { pub static AdvertisementSearchData_instance: Option<Rc<RefCell<AdvertisementSearchData>>> = AdvertisementSearchData::new(); }

impl AdvertisementSearchData {
    

    pub fn getInstance() -> Option<Rc<RefCell<AdvertisementSearchData>>> {
        return AdvertisementSearchData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            TITLE: String::new(),
            PRICE: String::new(),
            RELEVANCY: String::new(),
            POPULARITY: String::new(),
        };
        this.TITLE = String::from("AFFILIATE_TITLE");
        this.PRICE = String::from("AFFILIATE_PRICE");
        this.RELEVANCY = String::from("AFFILIATE_RELEVANCY");
        this.POPULARITY = String::from("AFFILIATE_POPULARITY");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

