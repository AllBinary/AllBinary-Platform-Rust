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
// package org.allbinary.input.automation.robot.osgi
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::InputAutomationRobotInterfaceWrapper::InputAutomationRobotInterfaceWrapper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationRobotChangeEvent {
    pub inputAutomationRobotInterfaceWrapper: Option<Rc<RefCell<InputAutomationRobotInterfaceWrapper>>>,
}


impl InputAutomationRobotChangeEvent {
    

    pub fn new_struct(mut inputAutomationRobotInterfaceWrapper: Option<Rc<RefCell<InputAutomationRobotInterfaceWrapper>>>) -> Self {
        let mut this = Self {
            inputAutomationRobotInterfaceWrapper: None,
        };
        this.setInputAutomationRobotInterfaceWrapper(inputAutomationRobotInterfaceWrapper.clone());
        this
    }

    pub fn new(mut inputAutomationRobotInterfaceWrapper: Option<Rc<RefCell<InputAutomationRobotInterfaceWrapper>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputAutomationRobotInterfaceWrapper))))
    }
    

    pub fn getInputAutomationRobotInterfaceWrapper(&mut self) -> Option<Rc<RefCell<InputAutomationRobotInterfaceWrapper>>> {
        return self.inputAutomationRobotInterfaceWrapper.clone();
    }
    

    pub fn setInputAutomationRobotInterfaceWrapper(&mut self, mut inputAutomationRobotInterfaceWrapper: Option<Rc<RefCell<InputAutomationRobotInterfaceWrapper>>>) {
        self.inputAutomationRobotInterfaceWrapper = inputAutomationRobotInterfaceWrapper.clone();
    }
}

