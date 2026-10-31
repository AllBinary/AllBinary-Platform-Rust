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
// package org.allbinary.game.graphics.hud
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

pub struct BasicHudFactory {
    pub DIRECTION_EXCEPTION: String,
    pub VERTICAL: i32,
    pub HORIZONTAL: i32,
    pub BOTTOMLEFT: i32,
    pub BOTTOMRIGHT: i32,
    pub TOPLEFT: i32,
    pub TOPRIGHT: i32,
    pub BOTTOMCENTER: i32,
    pub TOPCENTER: i32,
    pub ABSOLUTE: i32,
}

thread_local! { pub static BasicHudFactory_instance: Option<Rc<RefCell<BasicHudFactory>>> = BasicHudFactory::new(); }

impl BasicHudFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicHudFactory>>> {
        return BasicHudFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            DIRECTION_EXCEPTION: String::new(),
            VERTICAL: 0,
            HORIZONTAL: 0,
            BOTTOMLEFT: 0,
            BOTTOMRIGHT: 0,
            TOPLEFT: 0,
            TOPRIGHT: 0,
            BOTTOMCENTER: 0,
            TOPCENTER: 0,
            ABSOLUTE: 0,
        };
        this.DIRECTION_EXCEPTION = String::from("Only Horizontal Direction is Allowed Currently");
        this.VERTICAL = 0;
        this.HORIZONTAL = 1;
        this.BOTTOMLEFT = 0;
        this.BOTTOMRIGHT = 1;
        this.TOPLEFT = 2;
        this.TOPRIGHT = 3;
        this.BOTTOMCENTER = 4;
        this.TOPCENTER = 5;
        this.ABSOLUTE = 6;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

