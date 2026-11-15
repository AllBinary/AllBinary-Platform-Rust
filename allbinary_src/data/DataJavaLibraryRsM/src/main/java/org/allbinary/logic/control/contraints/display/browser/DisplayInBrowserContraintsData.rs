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

pub static DisplayInBrowserContraintsData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DISPLAYBROWSERCONSTRAINTS_NAME"));
pub static DisplayInBrowserContraintsData_FRAMEABLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DISPLAYBROWSERCONSTRAINTS_FRAMEABLE"));
pub static DisplayInBrowserContraintsData_MOUNTABLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DISPLAYBROWSERCONSTRAINTS_MOUNTABLE"));
pub static DisplayInBrowserContraintsData_QUICKFRAMEABLE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DISPLAYBROWSERCONSTRAINTS_QUICKFRAMEABLE"));
pub static DisplayInBrowserContraintsData_MOUNTABLELINK: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DISPLAYBROWSERCONSTRAINTS_MOUNTABLELINK"));

impl DisplayInBrowserContraintsData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

