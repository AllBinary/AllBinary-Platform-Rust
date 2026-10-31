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
// package org.allbinary.business.advertisement.search.website
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AdvertisementWebsiteInterface::AdvertisementWebsiteInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicAdvertisementWebsite {
    pub websiteId: String,
}

impl AdvertisementWebsiteInterface for BasicAdvertisementWebsite {
    fn toString(&mut self) -> String { Self::toString(self) }
}

impl BasicAdvertisementWebsite {
    

    pub fn new_struct(mut websiteId: String) -> Self {
        let mut this = Self {
            websiteId: String::new(),
        };
        this.websiteId = websiteId.clone();
        this
    }

    pub fn new(mut websiteId: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(websiteId))))
    }
    

    pub fn toString(&mut self) -> String {
        return self.websiteId.clone();
    }
}

