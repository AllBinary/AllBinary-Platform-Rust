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
// package org.allbinary.logic.java.characters
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

pub struct CharacterSetData {
    pub US_ASCII: String,
    pub ISO_8859_1: String,
    pub UTF_8: String,
    pub UTF_16: String,
    pub UTF_16BE: String,
    pub UTF_16LE: String,
}

thread_local! { pub static CharacterSetData_instance: Option<Rc<RefCell<CharacterSetData>>> = CharacterSetData::new(); }

impl CharacterSetData {
    

    pub fn getInstance() -> Option<Rc<RefCell<CharacterSetData>>> {
        return CharacterSetData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            US_ASCII: String::new(),
            ISO_8859_1: String::new(),
            UTF_8: String::new(),
            UTF_16: String::new(),
            UTF_16BE: String::new(),
            UTF_16LE: String::new(),
        };
        this.US_ASCII = String::from("US-ASCII");
        this.ISO_8859_1 = String::from("ISO-8859-1");
        this.UTF_8 = String::from("UTF-8");
        this.UTF_16 = String::from("UTF-16");
        this.UTF_16BE = String::from("UTF-16BE");
        this.UTF_16LE = String::from("UTF-16LE");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

