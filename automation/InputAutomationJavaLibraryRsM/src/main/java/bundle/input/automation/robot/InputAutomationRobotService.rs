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
// package bundle.input.automation.robot
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use super::InputAutomationRobotServiceInterface::InputAutomationRobotServiceInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputAutomationRobotService {
    pub inputRobotInterfaceArray: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>,
}

impl InputAutomationRobotServiceInterface for InputAutomationRobotService {
    fn getInputRobotInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>> { Self::getInputRobotInterfaceArray(self) }
}
impl crate::org::allbinary::osgi::OSGIServiceInterface::OSGIServiceInterface for InputAutomationRobotService {
}

impl InputAutomationRobotService {
    

    pub fn new_struct(mut inputRobotInterfaceArray: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>) -> Self {
        let mut this = Self {
            inputRobotInterfaceArray: Vec::new(),
        };
        this.setInputRobotInterfaceArray(inputRobotInterfaceArray.clone());
        this
    }

    pub fn new(mut inputRobotInterfaceArray: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(inputRobotInterfaceArray))))
    }
    

    pub fn getInputRobotInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>> {
        return self.inputRobotInterfaceArray.clone();
    }
    

    pub fn setInputRobotInterfaceArray(&mut self, mut inputRobotInterfaceArray: Vec<Option<Rc<RefCell<dyn InputRobotInterface>>>>) {
        self.inputRobotInterfaceArray = inputRobotInterfaceArray.clone();
    }
}

