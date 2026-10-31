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
// package org.allbinary.logic.system.os
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::java::character::CharacterSet::CharacterSet;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait OperatingSystemInterface {
    

    fn getName(&mut self) -> String;
    

    fn getVersion(&mut self) -> String;
    

    fn getArch(&mut self) -> String;
    

    fn getCharacterSet(&mut self) -> Option<Rc<RefCell<CharacterSet>>>;
    

    fn isOverScan(&mut self) -> bool;
    

    fn getOverScanXPercent(&mut self) -> i32;
    

    fn getOverScanYPercent(&mut self) -> i32;
    

    fn isScalable(&mut self) -> bool;
    

    fn isAutoHide(&mut self) -> bool;
    

    fn toString(&mut self) -> String;
}


