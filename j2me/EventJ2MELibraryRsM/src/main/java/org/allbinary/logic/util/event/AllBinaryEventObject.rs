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
// package org.allbinary.logic.util.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryEventObject {
    pub source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
}

thread_local! { pub static AllBinaryEventObject_NULL_EVENT: Option<Rc<RefCell<dyn AllBinaryEventObjectDyn>>> = AllBinaryEventObject::new(jcall!(NullUtil::getInstance()).NULL_OBJECT.clone()).map(|reference| reference as Rc<RefCell<dyn AllBinaryEventObjectDyn>>); }

impl AllBinaryEventObject {
    

    pub fn new_struct(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            source: None,
        };
        this.source = anyType.clone();
        this
    }

    pub fn new(mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(anyType))))
    }
    

    pub fn getSource(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return self.source.clone();
    }
    

    pub fn setSource(&mut self, mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.source = source.clone();
    }
}
pub trait AllBinaryEventObjectDyn {
    fn as_AllBinaryEventObject(&self) -> &AllBinaryEventObject;
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut AllBinaryEventObject;
    fn getSource(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { AllBinaryEventObject::getSource(self.as_AllBinaryEventObject_mut()) }
    fn setSource(&mut self, mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { AllBinaryEventObject::setSource(self.as_AllBinaryEventObject_mut(), source) }
}
impl AllBinaryEventObjectDyn for AllBinaryEventObject {
    fn as_AllBinaryEventObject(&self) -> &AllBinaryEventObject { self }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut AllBinaryEventObject { self }
}


