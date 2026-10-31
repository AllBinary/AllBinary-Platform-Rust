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
// package org.allbinary.graphics
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

pub struct ItemColorFactory {
    pub INVERT_PAINT: i32,
    pub PAINT: i32,
    pub TEXT_FIELD_NO_FOCUS: i32,
}

thread_local! { pub static ItemColorFactory_instance: Option<Rc<RefCell<ItemColorFactory>>> = ItemColorFactory::new(); }

impl ItemColorFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            INVERT_PAINT: 0,
            PAINT: 0,
            TEXT_FIELD_NO_FOCUS: 0,
        };
        this.INVERT_PAINT = 0xFF000000_u32 as i32;
        this.PAINT = 0xFFe07718_u32 as i32;
        this.TEXT_FIELD_NO_FOCUS = 0x7F7F7F_u32 as i32;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ItemColorFactory>>> {
        return ItemColorFactory_instance.with(Clone::clone).clone();
    }
}

