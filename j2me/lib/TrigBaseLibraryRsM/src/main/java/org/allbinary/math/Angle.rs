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
use crate::jsinterop::annotations::JsConstructor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Angle {
    pub value: i16,
}


impl Angle {
    

    pub fn new_struct(mut angle: i16) -> Self {
        let mut this = Self {
            value: 0,
        };
        this.value = angle.clone();
        this
    }

    pub fn new(mut angle: i16) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angle))))
    }
    

    pub fn getValue(&mut self) -> i16 {
        return self.value.clone();
    }
}
pub trait AngleDyn {
    fn as_Angle(&self) -> &Angle;
    fn as_Angle_mut(&mut self) -> &mut Angle;
    fn getValue(&mut self) -> i16 { Angle::getValue(self.as_Angle_mut()) }
}
impl AngleDyn for Angle {
    fn as_Angle(&self) -> &Angle { self }
    fn as_Angle_mut(&mut self) -> &mut Angle { self }
}


