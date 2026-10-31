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
// package org.allbinary.input.automation.module.actions.script.condition.processors.output
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ProfileActionScriptOutputInterface::ProfileActionScriptOutputInterface;
use super::ImageTypes::ImageTypes;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait ImageActionScriptOutputInterface: ProfileActionScriptOutputInterface {
    

    fn getImageTypes(&mut self) -> Option<Rc<RefCell<ImageTypes>>>;
    

    fn setImageTypes(&mut self, imageTypes: Option<Rc<RefCell<ImageTypes>>>);
    

    fn isSaved(&mut self) -> bool;
    

    fn setSaved(&mut self, saved: bool);
    

    fn isDisplay(&mut self) -> bool;
    

    fn setDisplay(&mut self, display: bool);
    

    fn log(&mut self);
}


