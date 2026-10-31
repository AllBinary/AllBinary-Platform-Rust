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
// package org.allbinary.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinarySensorManager {
    pub ORIENTATION_SENSOR_INPUT: String,
    pub ORIENTATION_TYPE: String,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

pub static mut AllBinarySensorManager_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl AllBinarySensorManager {
    

    pub fn getInstance() -> Option<Rc<RefCell<AllBinarySensorManager>>> {
        if match (&unsafe { AllBinarySensorManager_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *AllBinarySensorManager_instance = AllBinarySensorManager::new() };
        }
        return (unsafe { AllBinarySensorManager_instance.clone() }).clone() /* as AllBinarySensorManager */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ORIENTATION_SENSOR_INPUT: String::new(),
            ORIENTATION_TYPE: String::new(),
            list: None,
        };
        this.ORIENTATION_SENSOR_INPUT = String::from("Orientation Sensor Input");
        this.ORIENTATION_TYPE = String::from("Orientation Type");
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn getSensorNamesList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.list.clone();
    }
    

    pub fn shutdown(&mut self) {
    }
}

