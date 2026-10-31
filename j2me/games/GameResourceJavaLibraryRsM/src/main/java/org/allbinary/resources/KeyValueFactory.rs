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
// package org.allbinary.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyValueFactory {
    pub MAP: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static KeyValueFactory_instance: Option<Rc<RefCell<KeyValueFactory>>> = KeyValueFactory::new(); }

impl KeyValueFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAP: None,
        };
        this.MAP = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<KeyValueFactory>>> {
        return KeyValueFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn get(&mut self, mut key: String) -> String {
        let mut value: String = (jcall!(self.MAP).get(crate::java_into_object((key).clone() /* as Any */))).clone() /* as String */;
        if false {
            return key.clone();
        }
        return value.clone();
    }
}

