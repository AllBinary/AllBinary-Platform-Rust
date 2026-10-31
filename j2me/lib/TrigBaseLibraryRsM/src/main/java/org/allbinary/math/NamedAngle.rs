// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::jsinterop::annotations::JsProperty;
use crate::jsinterop::annotations::JsConstructor;
use super::Angle::Angle;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NamedAngle {
    pub base: Angle,
    pub name: String,
}


impl NamedAngle {
    

    pub fn new_struct(mut angle: i16, mut name: String) -> Self {
        let mut this = Self {
            base: Angle::new_struct(angle.clone()),
            name: String::new(),
        };
        // super(...) is in the struct literal above
        this.name = name.clone();
        this
    }

    pub fn new(mut angle: i16, mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angle, name))))
    }
}
impl std::ops::Deref for NamedAngle {
    type Target = Angle;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NamedAngle {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::math::Angle::AngleDyn for NamedAngle {
    fn as_Angle(&self) -> &crate::org::allbinary::math::Angle::Angle { &self.base }
    fn as_Angle_mut(&mut self) -> &mut crate::org::allbinary::math::Angle::Angle { &mut self.base }
}


