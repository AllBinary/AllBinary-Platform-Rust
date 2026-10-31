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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ProfileActionScriptInputInterface::ProfileActionScriptInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait KeyboardActionScriptInputInterface: ProfileActionScriptInputInterface {
    

    fn getDelayBetweenKeys(&mut self) -> i32;
    

    fn getKeyArray(&mut self) -> Vec<Option<i32>>;
    

    fn isPress(&mut self) -> bool;
    

    fn isRelease(&mut self) -> bool;
    

    fn isNormal(&mut self) -> bool;
    

    fn setPress(&mut self, press: bool);
    

    fn setRelease(&mut self, release: bool);
    

    fn setNormal(&mut self);
    

    fn getText(&mut self) -> String;
    

    fn setText(&mut self, text: String);
    

    fn log(&mut self);
}


