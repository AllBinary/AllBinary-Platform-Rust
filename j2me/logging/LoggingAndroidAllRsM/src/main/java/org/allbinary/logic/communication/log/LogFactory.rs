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
// package org.allbinary.logic.communication.log
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::Log::Log;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ActualPlatform
pub struct LogFactory {
}


impl LogFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstanceD() -> Option<Rc<RefCell<Log>>> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut nullUtil: Option<Rc<RefCell<NullUtil>>> = NullUtil::getInstance();
        return Log::new(jcall!(stringUtil).EMPTY_STRING.clone(), jcall!(nullUtil).NULL_OBJECT.clone(), jcall!(stringUtil).EMPTY_STRING.clone(), jcall!(nullUtil).NULL_OBJECT.clone());
    }
    

    pub fn getInstanceNoF(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Log>>> {
        return Log::new(specialMessage.clone(), anyType.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), exception.clone());
    }
    

    pub fn getInstance(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Log>>> {
        return Log::new(specialMessage.clone(), anyType.clone(), functionName.clone(), exception.clone());
    }
    

    pub fn getInstanceF(mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) -> Option<Rc<RefCell<Log>>> {
        return Log::new(specialMessage.clone(), anyType.clone(), functionName.clone(), jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
    }
}

