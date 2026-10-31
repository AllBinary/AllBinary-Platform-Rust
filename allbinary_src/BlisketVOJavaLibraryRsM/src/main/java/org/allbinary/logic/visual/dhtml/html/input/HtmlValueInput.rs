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
// package org.allbinary.logic.visual.dhtml.html.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::HtmlInput::HtmlInput;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlValueInput {
    pub base: HtmlInput,
    pub value: String,
    pub VALUE: String,
}


impl HtmlValueInput {
    

    pub fn new_struct(mut before: String, mut r#type: String, mut name: String, mut value: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlInput::new_struct(before.clone(), r#type.clone(), name.clone(), after.clone()),
            value: String::new(),
            VALUE: String::new(),
        };
        this.VALUE = String::from("VALUE");
        // super(...) is in the struct literal above
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        this.value = jcall!(stringUtil).EMPTY_STRING.clone();
        this.addAttribute(this.VALUE.clone(), value.clone());
        this
    }

    pub fn new(mut before: String, mut r#type: String, mut name: String, mut value: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, r#type, name, value, after))))
    }
}
impl std::ops::Deref for HtmlValueInput {
    type Target = HtmlInput;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlValueInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HtmlValueInputDyn: crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInputDyn {
    fn as_HtmlValueInput(&self) -> &HtmlValueInput;
    fn as_HtmlValueInput_mut(&mut self) -> &mut HtmlValueInput;
}
impl HtmlValueInputDyn for HtmlValueInput {
    fn as_HtmlValueInput(&self) -> &HtmlValueInput { self }
    fn as_HtmlValueInput_mut(&mut self) -> &mut HtmlValueInput { self }
}

impl crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInputDyn for HtmlValueInput {
    fn as_HtmlInput(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInput { &self.base }
    fn as_HtmlInput_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInput { &mut self.base }
}
impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlValueInput {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base.base }
}


