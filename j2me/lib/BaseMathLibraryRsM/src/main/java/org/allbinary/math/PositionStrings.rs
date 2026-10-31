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
// package org.allbinary.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PositionStrings {
    pub X: String,
    pub Y: String,
    pub X_LABEL: String,
    pub Y_LABEL: String,
    pub Z_LABEL: String,
    pub DX_LABEL: String,
    pub DY_LABEL: String,
    pub DZ_LABEL: String,
}

thread_local! { pub static PositionStrings_instance: Option<Rc<RefCell<PositionStrings>>> = PositionStrings::new(); }

impl PositionStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            X: String::new(),
            Y: String::new(),
            X_LABEL: String::new(),
            Y_LABEL: String::new(),
            Z_LABEL: String::new(),
            DX_LABEL: String::new(),
            DY_LABEL: String::new(),
            DZ_LABEL: String::new(),
        };
        this.X = String::from("x");
        this.Y = String::from("y");
        this.X_LABEL = String::from("x: ");
        this.Y_LABEL = String::from("y: ");
        this.Z_LABEL = String::from("z: ");
        this.DX_LABEL = String::from("dx: ");
        this.DY_LABEL = String::from("dy: ");
        this.DZ_LABEL = String::from("dz: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PositionStrings>>> {
        return PositionStrings_instance.with(Clone::clone).clone();
    }
}

