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
// package org.allbinary.logic.visual.dhtml.html
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlTag {
    pub otherAttributes: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>,
}


impl HtmlTag {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            otherAttributes: None,
        };
        this.otherAttributes = { let java_value = (jcall!(StdUtil::getInstance()).createHashMap()).clone(); java_value };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addAttribute(&mut self, mut key: String, mut value: String) {
        jcall!(self.otherAttributes).put(crate::java_into_object(key.clone()), crate::java_into_object(value.clone()));
    }
}
pub trait HtmlTagDyn: std::any::Any {
    fn as_HtmlTag(&self) -> &HtmlTag;
    fn as_HtmlTag_mut(&mut self) -> &mut HtmlTag;
    fn addAttribute(&mut self, mut key: String, mut value: String) { HtmlTag::addAttribute(self.as_HtmlTag_mut(), key, value) }
}
impl HtmlTagDyn for HtmlTag {
    fn as_HtmlTag(&self) -> &HtmlTag { self }
    fn as_HtmlTag_mut(&mut self) -> &mut HtmlTag { self }
}


