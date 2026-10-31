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
// package org.allbinary.input.automation.actions.script.condition
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;
use crate::org::allbinary::media::image::analysis::ColorRangeInterface::ColorRangeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ColorAtActionScriptConditionInterface {
    

    fn getPoint(&mut self) -> Option<Rc<RefCell<Point>>>;
    

    fn setPoint(&mut self, point: Option<Rc<RefCell<Point>>>);
    

    fn getColorRangeInterface(&mut self) -> Option<Rc<RefCell<dyn ColorRangeInterface>>>;
    

    fn setColorRangeInterface(&mut self, colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>);
    

    fn log(&mut self);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


