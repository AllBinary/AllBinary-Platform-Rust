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
// package org.allbinary.logic.control.contraints.display.browser
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisplayInBrowserContraintsData {
}

thread_local! { pub static DisplayInBrowserContraintsData_NAME: String = String::from("DISPLAYBROWSERCONSTRAINTS_NAME"); }
thread_local! { pub static DisplayInBrowserContraintsData_FRAMEABLE: String = String::from("DISPLAYBROWSERCONSTRAINTS_FRAMEABLE"); }
thread_local! { pub static DisplayInBrowserContraintsData_MOUNTABLE: String = String::from("DISPLAYBROWSERCONSTRAINTS_MOUNTABLE"); }
thread_local! { pub static DisplayInBrowserContraintsData_QUICKFRAMEABLE: String = String::from("DISPLAYBROWSERCONSTRAINTS_QUICKFRAMEABLE"); }
thread_local! { pub static DisplayInBrowserContraintsData_MOUNTABLELINK: String = String::from("DISPLAYBROWSERCONSTRAINTS_MOUNTABLELINK"); }

impl DisplayInBrowserContraintsData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

