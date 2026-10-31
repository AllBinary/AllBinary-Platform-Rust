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
// package org.allbinary.logic.system.hardware
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::NoHardwareComponent::NoHardwareComponent;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::HardwareInterface::HardwareInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoHardware {
}

thread_local! { pub static NoHardware_instance: Option<Rc<RefCell<dyn HardwareInterface>>> = NoHardware::new().map(|reference| reference as Rc<RefCell<dyn HardwareInterface>>); }
impl HardwareInterface for NoHardware {
    fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> { Self::getComponent(self, index) }
    fn toString(&mut self) -> String { Self::toString(self) }
    fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool { Self::compareTo(self, hardwareInterface) }
    fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::difference(self, hardwareInterface) }
}

impl NoHardware {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn HardwareInterface>>> {
        return NoHardware_instance.with(Clone::clone).clone();
    }
    

    pub fn getComponent(&mut self, mut index: i32) -> Option<Rc<RefCell<dyn HardwareComponentInterface>>> {
        return NoHardwareComponent::getInstance().map(|reference| reference as Rc<RefCell<dyn HardwareComponentInterface>>);
    }
    

    pub fn toString(&mut self) -> String {
        return String::from("No Hardware");
    }
    

    pub fn compareTo(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> bool {
        return true;
    }
    

    pub fn difference(&mut self, mut hardwareInterface: Option<Rc<RefCell<dyn HardwareInterface>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
}

