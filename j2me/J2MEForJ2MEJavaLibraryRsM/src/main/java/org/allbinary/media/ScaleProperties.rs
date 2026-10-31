// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.media
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScaleProperties {
    pub scaleX: f32,
    pub scaleY: f32,
    pub scaleWidth: i32,
    pub scaleHeight: i32,
    pub shouldScale: bool,
}

thread_local! { pub static ScaleProperties_instance: Option<Rc<RefCell<ScaleProperties>>> = ScaleProperties::new(); }

impl ScaleProperties {

    pub fn new_struct() -> Self {
        let mut this = Self {
            scaleX: 0.0_f32,
            scaleY: 0.0_f32,
            scaleWidth: 0,
            scaleHeight: 0,
            shouldScale: false,
        };
        this.scaleX = 1.0_f32;
        this.scaleY = 1.0_f32;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).COLON_SEP.clone()).appendboolean(self.shouldScale.clone()).append(String::from("scaleX: ")).appendfloat(self.scaleX.clone()).append(String::from("scaleY: ")).appendfloat(self.scaleY.clone()).append(String::from("scaleWidth: ")).appendint(self.scaleWidth.clone()).append(String::from("scaleHeight: ")).appendint(self.scaleHeight.clone()).toString();
    }
}

