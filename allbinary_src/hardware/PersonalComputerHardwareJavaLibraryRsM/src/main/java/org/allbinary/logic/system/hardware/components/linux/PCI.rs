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
// package org.allbinary.logic.system.hardware.components.linux
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::hardware::components::interfaces::HardwareComponentInterface::HardwareComponentInterface;
use crate::org::allbinary::logic::system::hardware::components::interfaces::VideoInterface::VideoInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PCI {
    pub data: String,
}

impl HardwareComponentInterface for PCI {
    fn compareTo(&mut self, mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>>) -> bool { Self::compareTo(self, componentInterface) }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl VideoInterface for PCI {
}

impl PCI {
    

    pub fn new_struct(mut data: String) -> Self {
        let mut this = Self {
            data: String::new(),
        };
        this.data = data.clone();
        this
    }

    pub fn new(mut data: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(data))))
    }
    

    pub fn compareTo(&mut self, mut componentInterface: Option<Rc<RefCell<dyn HardwareComponentInterface>>>) -> bool {
        return true;
    }
    

    pub fn toString(&mut self) -> String {
        return self.data.clone();
    }
}

