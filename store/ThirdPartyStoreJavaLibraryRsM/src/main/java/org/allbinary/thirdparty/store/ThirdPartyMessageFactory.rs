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
// package org.allbinary.thirdparty.store
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

pub struct ThirdPartyMessageFactory {
    pub AUTHORIZED: String,
    pub NOT_AUTHORIZED: String,
    pub LINKAGE_ERROR: String,
}

thread_local! { pub static ThirdPartyMessageFactory_instance: Option<Rc<RefCell<ThirdPartyMessageFactory>>> = ThirdPartyMessageFactory::new(); }

impl ThirdPartyMessageFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            AUTHORIZED: String::new(),
            NOT_AUTHORIZED: String::new(),
            LINKAGE_ERROR: String::new(),
        };
        this.AUTHORIZED = String::from("Authorized");
        this.NOT_AUTHORIZED = String::from("Not Authorized");
        this.LINKAGE_ERROR = String::from("Linkage Error");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThirdPartyMessageFactory>>> {
        return ThirdPartyMessageFactory_instance.with(Clone::clone).clone();
    }
}

