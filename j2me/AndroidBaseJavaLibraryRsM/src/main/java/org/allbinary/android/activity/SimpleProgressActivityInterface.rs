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
// package org.allbinary.android.activity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait SimpleProgressActivityInterface {
    

    fn runOnUiThread(&mut self, action: Option<Rc<RefCell<dyn Runnable>>>);
    

    // throws Exception
    fn onSetProgress(&mut self, value: i32, text: String);
    

    // throws Exception
    fn onDismissProgress(&mut self, animationSequence: Vec<i32>);
    

    // throws Exception
    fn onShowProgress(&mut self, indeterminate: bool, animationSequence: Vec<i32>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


