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
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PointFactory {
    pub ZERO_ZERO: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}

thread_local! { pub static PointFactory_instance: Option<Rc<RefCell<PointFactory>>> = PointFactory::new(); }

impl PointFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<PointFactory>>> {
        return PointFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ZERO_ZERO: None,
        };
        this.ZERO_ZERO = this.createXYZ(0, 0, 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn createXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return GPoint::new(x.clone(), y.clone(), 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>);
    }
    

    pub fn createXYZ(&mut self, mut x: i32, mut y: i32, mut z: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return GPoint::new(x.clone(), y.clone(), z.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>);
    }
}

