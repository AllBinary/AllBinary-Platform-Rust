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
// package org.allbinary.business.entry
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

pub struct EntryData {
    pub ID: String,
    pub ENCRYPTION: String,
    pub TIMECREATED: String,
    pub LASTMODIFIED: String,
    pub DEFAULT: String,
    pub SPECIAL: String,
    pub ENABLE: String,
    pub YES: String,
    pub NO: String,
}

thread_local! { pub static EntryData_instance: Option<Rc<RefCell<EntryData>>> = EntryData::new(); }

impl EntryData {
    

    pub fn getInstance() -> Option<Rc<RefCell<EntryData>>> {
        return EntryData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ID: String::new(),
            ENCRYPTION: String::new(),
            TIMECREATED: String::new(),
            LASTMODIFIED: String::new(),
            DEFAULT: String::new(),
            SPECIAL: String::new(),
            ENABLE: String::new(),
            YES: String::new(),
            NO: String::new(),
        };
        this.ID = String::from("ENTRY_ID");
        this.ENCRYPTION = String::from("ENTRY_ENCRYPTION");
        this.TIMECREATED = String::from("ENTRY_TIMECREATED");
        this.LASTMODIFIED = String::from("ENTRY_LASTMODIFIED");
        this.DEFAULT = String::from("ENTRY_DEFAULT");
        this.SPECIAL = String::from("ENTRY_SPECIAL");
        this.ENABLE = String::from("ENTRY_ENABLE");
        this.YES = String::from("Yes");
        this.NO = String::from("No");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

