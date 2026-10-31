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
use crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlInput {
    pub base: HtmlTag,
    pub END: String,
    pub STARTINPUT: String,
    pub ENDINPUT: String,
    pub TYPE: String,
    pub NAME: String,
    pub before: String,
    pub r#type: String,
    pub name: String,
    pub after: String,
}


impl HtmlInput {
    

    pub fn new_struct(mut before: String, mut r#type: String, mut name: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlTag::new_struct(),
            END: String::new(),
            STARTINPUT: String::new(),
            ENDINPUT: String::new(),
            TYPE: String::new(),
            NAME: String::new(),
            before: String::new(),
            r#type: String::new(),
            name: String::new(),
            after: String::new(),
        };
        this.END = String::from(">");
        this.STARTINPUT = String::from("<INPUT ");
        this.ENDINPUT = String::from("</INPUT>");
        this.TYPE = String::from("TYPE=\"");
        this.NAME = String::from("NAME=\"");
        this.before = before.clone();
        this.r#type = r#type.clone();
        this.name = name.clone();
        this.after = after.clone();
        this
    }

    pub fn new(mut before: String, mut r#type: String, mut name: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, r#type, name, after))))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut attributeKeys: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.otherAttributes).keySet()).toArray();
        let mut attributeSize: i32 = (attributeKeys.len() as i32).clone();
        result = self.before.clone();
        result += &self.STARTINPUT.clone();
        result += &self.TYPE.clone();
        result += &self.r#type.clone();
        result += &String::from("\" ");
        result += &self.NAME.clone();
        result += &self.name.clone();
        result += &String::from("\" ");
        for i in (0..attributeSize) {
            let mut key: String = (attributeKeys[(i) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(self.base.otherAttributes).get(key.clone())).clone() /* as String */;
            if true && (value.cmp(&jcall!(stringUtil).EMPTY_STRING.clone()) as i32) != 0 {
                result += &key.clone();
                result += &String::from("=\"");
                result += &value.clone();
                result += &String::from("\" ");
            }
        }
        result += &self.END.clone();
        result += &self.ENDINPUT.clone();
        result += &self.after.clone();
        return result.clone();
    }
}
impl std::ops::Deref for HtmlInput {
    type Target = HtmlTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlInput {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HtmlInputDyn: crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn {
    fn as_HtmlInput(&self) -> &HtmlInput;
    fn as_HtmlInput_mut(&mut self) -> &mut HtmlInput;
    fn toString(&mut self) -> String { HtmlInput::toString(self.as_HtmlInput_mut()) }
}
impl HtmlInputDyn for HtmlInput {
    fn as_HtmlInput(&self) -> &HtmlInput { self }
    fn as_HtmlInput_mut(&mut self) -> &mut HtmlInput { self }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlInput {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base }
}


