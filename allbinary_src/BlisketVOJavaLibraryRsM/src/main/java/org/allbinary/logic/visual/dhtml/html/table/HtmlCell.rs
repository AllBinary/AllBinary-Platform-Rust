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
// package org.allbinary.logic.visual.dhtml.html.table
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

pub struct HtmlCell {
    pub base: HtmlTag,
    pub END: String,
    pub START: String,
    pub ENDTAG: String,
    pub HEIGHT: String,
    pub WIDTH: String,
    pub height: String,
    pub width: String,
    pub before: String,
    pub body: String,
    pub after: String,
}


impl HtmlCell {
    

    pub fn new_struct(mut before: String, mut width: String, mut height: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlTag::new_struct(),
            END: String::new(),
            START: String::new(),
            ENDTAG: String::new(),
            HEIGHT: String::new(),
            WIDTH: String::new(),
            height: String::new(),
            width: String::new(),
            before: String::new(),
            body: String::new(),
            after: String::new(),
        };
        this.END = String::from(">");
        this.START = String::from("<td ");
        this.ENDTAG = String::from("</td>");
        this.HEIGHT = String::from("height");
        this.WIDTH = String::from("width");
        this.before = before.clone().clone();
        this.after = after.clone().clone();
        this.height = height.clone().clone();
        this.width = width.clone().clone();
        this.addAttribute(this.HEIGHT.clone(), this.height.clone());
        this.addAttribute(this.WIDTH.clone(), this.width.clone());
        this
    }

    pub fn new(mut before: String, mut width: String, mut height: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, width, height, after))))
    }
    

    pub fn addText(&mut self, mut body: String) {
        self.body = body.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut attributeKeys: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.otherAttributes).keySet()).toArray();
        let mut attributeSize: i32 = (attributeKeys.len() as i32).clone();
        result = self.before.clone();
        result += &self.START.clone();
        result += &String::from(" ");
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
        result += &self.body.clone();
        result += &self.ENDTAG.clone();
        result += &self.after.clone();
        return result.clone();
    }
}
impl std::ops::Deref for HtmlCell {
    type Target = HtmlTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlCell {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlCell {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base }
}


