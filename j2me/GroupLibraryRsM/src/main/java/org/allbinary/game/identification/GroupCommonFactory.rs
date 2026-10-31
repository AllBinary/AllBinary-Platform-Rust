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
// package org.allbinary.game.identification
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GroupCommonFactory {
    pub ID: Option<i32>,
    pub GROUP_NAME_LABEL: String,
    pub ID_LABEL: String,
    pub ADDING_LABEL: String,
    pub TO_LABEL: String,
}

pub static mut GroupCommonFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GroupCommonFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ID: None,
            GROUP_NAME_LABEL: String::new(),
            ID_LABEL: String::new(),
            ADDING_LABEL: String::new(),
            TO_LABEL: String::new(),
        };
        this.ID = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(10);
        this.GROUP_NAME_LABEL = String::from("Group Name: ");
        this.ID_LABEL = String::from(" Id: ");
        this.ADDING_LABEL = String::from("Adding: ");
        this.TO_LABEL = String::from(" to: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GroupCommonFactory>>> {
        if match (&unsafe { GroupCommonFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GroupCommonFactory_instance = GroupCommonFactory::new() };
        }
        return (unsafe { GroupCommonFactory_instance.clone() }).clone() /* as GroupCommonFactory */;
    }
}

