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
// use crate::java::awt::event::KeyEvent::KeyEvent;
// use crate::javax::swing::*;
use crate::org::allbinary::input::KeySingletonFactory::KeySingletonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyNameDefaultComboBoxModelFactory {
}


impl KeyNameDefaultComboBoxModelFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DefaultComboBoxModel>>> {
        let mut defaultComboBoxModel: Option<Rc<RefCell<DefaultComboBoxModel>>> = DefaultComboBoxModel::new();
        let mut keyArray: Vec<Option<i32>> = KeySingletonFactory::getArray();
        for index in (0..(keyArray.len() as i32)) {
            let mut keyAsString: String = KeyEvent::getKeyText(keyArray[(index) as usize].clone());
            jcall!(defaultComboBoxModel).addElement(keyAsString.clone());
        }
        return defaultComboBoxModel.clone();
    }
}

