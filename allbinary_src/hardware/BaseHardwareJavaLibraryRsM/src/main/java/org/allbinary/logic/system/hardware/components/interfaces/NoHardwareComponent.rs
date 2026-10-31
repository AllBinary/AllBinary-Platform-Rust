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
// package org.allbinary.logic.system.hardware.components.interfaces
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::HardwareComponentInterface::HardwareComponentInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoHardwareComponent {
}

thread_local! { pub static NoHardwareComponent_instance: Option<Rc<RefCell<NoHardwareComponent>>> = NoHardwareComponent::new(); }
impl HardwareComponentInterface for NoHardwareComponent {
    fn compareTo(&mut self, mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>>) -> bool { Self::compareTo(self, componentInterface) }
    fn toString(&mut self) -> String { Self::toString(self) }
}

impl NoHardwareComponent {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NoHardwareComponent>>> {
        return NoHardwareComponent_instance.with(Clone::clone).clone();
    }
    

    pub fn compareTo(&mut self, mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>>) -> bool {
        if match &(componentInterface) { Some(reference) => std::ptr::addr_eq(&*self as *const Self, reference.as_ptr()), None => false } {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).NULL_STRING.clone();
    }
}

