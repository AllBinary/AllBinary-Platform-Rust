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
// package org.allbinary.business.category.hierarchy
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::category::CategoryInterface::CategoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait CategoryHierarchyInterface {
    

    fn getLevel(&mut self) -> i32;
    

    fn getLevelString(&mut self) -> String;
    

    fn getRoot(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    fn getParent(&mut self) -> Option<Rc<RefCell<dyn CategoryInterface>>>;
    

    fn setLevel(&mut self, level: i32);
    

    fn setRoot(&mut self, categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>);
    

    fn setParent(&mut self, categoryInterface: Option<Rc<RefCell<dyn CategoryInterface>>>);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


