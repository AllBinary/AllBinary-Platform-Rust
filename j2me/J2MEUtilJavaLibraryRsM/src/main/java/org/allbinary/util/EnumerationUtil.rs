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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
use crate::jsinterop::annotations::JsType;
// use crate::java::util::Enumeration::Enumeration;
use crate::jsinterop::annotations::JsMethod;
use super::BasicArrayListD::BasicArrayListD;
use super::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//ArkTs does not have this.  So this is for the ArkTs build.
pub struct EnumerationUtil {
}

thread_local! { pub static EnumerationUtil_instance: Option<Rc<RefCell<EnumerationUtil>>> = EnumerationUtil::new(); }

impl EnumerationUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<EnumerationUtil>>> {
        return EnumerationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn hasMoreElements(&mut self, mut enumeration: Option<Rc<RefCell<dyn Enumeration>>>) -> bool {
        return jcall!(enumeration).hasMoreElements();
    }
    

    pub fn nextElement(&mut self, mut enumeration: Option<Rc<RefCell<dyn Enumeration>>>) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(enumeration).nextElement();
    }
    

    pub fn getAsArray(&mut self, mut enumeration: Option<Rc<RefCell<dyn Enumeration>>>) -> Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> {
        let mut basicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        while self.hasMoreElements(enumeration.clone()) {
            jcall!(basicArrayList).add(crate::java_into_object(self.nextElement(enumeration.clone())));
        }
        return jcall!(basicArrayList).toArray();
    }
}

