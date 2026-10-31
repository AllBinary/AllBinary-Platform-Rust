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
// package org.allbinary.business.advertisement
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

pub struct AdvertisementData {
    pub NAME: String,
    pub DESCRIPTION: String,
}

thread_local! { pub static AdvertisementData_instance: Option<Rc<RefCell<AdvertisementData>>> = AdvertisementData::new(); }

impl AdvertisementData {
    

    pub fn getInstance() -> Option<Rc<RefCell<AdvertisementData>>> {
        return AdvertisementData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NAME: String::new(),
            DESCRIPTION: String::new(),
        };
        this.NAME = String::from("ADVERTISEMENT_NAME");
        this.DESCRIPTION = String::from("ADVERTISEMENT_DESCRIPTION");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

