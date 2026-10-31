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
// package org.allbinary.animation.text
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TextChangeListener::TextChangeListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TextInterface {
    

    fn setTextWithOnMeasure(&mut self, text: String, textChangeListener: Option<Rc<RefCell<TextChangeListener>>>);
    

    fn getText(&mut self) -> String;
    

    fn setText(&mut self, text: String);
    

    fn getFontHeight(&mut self) -> i32;
    

    fn getWidth(&mut self) -> i32;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


