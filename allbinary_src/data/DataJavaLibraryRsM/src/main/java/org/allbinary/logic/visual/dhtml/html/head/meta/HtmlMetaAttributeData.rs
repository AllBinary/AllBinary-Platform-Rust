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
// package org.allbinary.logic.visual.dhtml.html.head.meta
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

pub struct HtmlMetaAttributeData {
    pub id: i32,
    pub name: String,
}


impl HtmlMetaAttributeData {
    

    pub fn new_struct(mut id: i32, mut value: String) -> Self {
        let mut this = Self {
            id: 0,
            name: String::new(),
        };
        this.name = value.clone();
        this.id = id.clone();
        this
    }

    pub fn new(mut id: i32, mut value: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, value))))
    }
    

    pub fn equals(&mut self, mut htmlMetaAttributeData: Option<Rc<RefCell<HtmlMetaAttributeData>>>) -> bool {
        if jcall!(htmlMetaAttributeData).id == self.id {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}

