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
// package org.allbinary.media.image.analysis
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ColorRangeInterface {
    

    fn getMinRed(&mut self) -> i32;
    

    fn setMinRed(&mut self, minRed: i32);
    

    fn getMaxRed(&mut self) -> i32;
    

    fn setMaxRed(&mut self, maxRed: i32);
    

    fn getMinGreen(&mut self) -> i32;
    

    fn setMinGreen(&mut self, minGreen: i32);
    

    fn getMaxGreen(&mut self) -> i32;
    

    fn setMaxGreen(&mut self, maxGreen: i32);
    

    fn getMinBlue(&mut self) -> i32;
    

    fn setMinBlue(&mut self, minBlue: i32);
    

    fn getMaxBlue(&mut self) -> i32;
    

    fn setMaxBlue(&mut self, maxBlue: i32);
    

    fn isInRange(&mut self, color: Option<Rc<RefCell<Color>>>) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


