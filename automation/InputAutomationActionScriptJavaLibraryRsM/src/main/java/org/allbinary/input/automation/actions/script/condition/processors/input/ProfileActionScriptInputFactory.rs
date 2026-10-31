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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::data::tree::dom::DomSearchHelper::DomSearchHelper;
use crate::org::w3c::dom::Node::Node;
use super::ProfileActionScriptInputInterface::ProfileActionScriptInputInterface;
use super::MouseActionScriptInput::MouseActionScriptInput;
use super::KeyboardActionScriptInput::KeyboardActionScriptInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ProfileActionScriptInputFactory {
}


impl ProfileActionScriptInputFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(mut node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<dyn ProfileActionScriptInputInterface>>> {
        let mut profileActionScriptInputInterface: Option<Rc<RefCell<dyn ProfileActionScriptInputInterface>>> = None;
        if DomSearchHelper::getNodeNoThrow(unsafe { KeyboardActionScriptInputData_NAME.clone() }.clone(), jcall!(node).getChildNodes()).is_some() {
            profileActionScriptInputInterface = KeyboardActionScriptInput::new1(node.clone()).map(|reference| reference as Rc<RefCell<dyn ProfileActionScriptInputInterface>>);
        } else if DomSearchHelper::getNodeNoThrow(unsafe { MouseActionScriptInputData_NAME.clone() }.clone(), jcall!(node).getChildNodes()).is_some() {
            profileActionScriptInputInterface = MouseActionScriptInput::new1(node.clone()).map(|reference| reference as Rc<RefCell<dyn ProfileActionScriptInputInterface>>);
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such ActionScript Input"))));
        }
        return profileActionScriptInputInterface.clone();
    }
}

