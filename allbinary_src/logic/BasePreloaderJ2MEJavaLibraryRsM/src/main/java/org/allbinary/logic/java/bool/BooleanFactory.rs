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
// package org.allbinary.logic.java.bool
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

pub struct BooleanFactory {
    pub TRUE: Option<bool>,
    pub FALSE: Option<bool>,
    pub TRUE_STRING: String,
    pub FALSE_STRING: String,
    pub YES: String,
    pub NO: String,
}

thread_local! { pub static BooleanFactory_instance: Option<Rc<RefCell<BooleanFactory>>> = BooleanFactory::new(); }

impl BooleanFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TRUE: None,
            FALSE: None,
            TRUE_STRING: String::new(),
            FALSE_STRING: String::new(),
            YES: String::new(),
            NO: String::new(),
        };
        this.TRUE = Some(true);
        this.FALSE = Some(false);
        this.TRUE_STRING = (this.TRUE).expect("null Boolean").to_string();
        this.FALSE_STRING = (this.FALSE).expect("null Boolean").to_string();
        this.YES = String::from("yes");
        this.NO = String::from("no");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BooleanFactory>>> {
        return BooleanFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn toStringb(&mut self, mut bool: bool) -> String {
        if bool {
            return self.TRUE_STRING.clone();
        } else {
            return self.FALSE_STRING.clone();
        }
    }
}

