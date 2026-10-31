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
// package org.allbinary.logic.visual.dhtml.html.select
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::io::LineReader::LineReader;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlSelect {
    pub base: HtmlTag,
    pub END: String,
    pub STARTSELECT: String,
    pub ENDSELECT: String,
    pub SIZE: String,
    pub NAME: String,
    pub STARTOPTION: String,
    pub SELETED: String,
    pub ENDOPTION: String,
    pub before: String,
    pub name: String,
    pub after: String,
    pub size: String,
    pub selected: String,
    pub options: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub multiple: String,
    pub isMultipleSelect: bool,
}


impl HtmlSelect {
    

    pub fn new_struct(mut before: String, mut size: String, mut name: String, mut after: String) -> Self {
        let mut this = Self {
            base: HtmlTag::new_struct(),
            END: String::new(),
            STARTSELECT: String::new(),
            ENDSELECT: String::new(),
            SIZE: String::new(),
            NAME: String::new(),
            STARTOPTION: String::new(),
            SELETED: String::new(),
            ENDOPTION: String::new(),
            before: String::new(),
            name: String::new(),
            after: String::new(),
            size: String::new(),
            selected: String::new(),
            options: None,
            multiple: String::new(),
            isMultipleSelect: false,
        };
        this.END = String::from(">");
        this.STARTSELECT = String::from("<SELECT ");
        this.ENDSELECT = String::from("</SELECT>");
        this.SIZE = String::from("SIZE=\"");
        this.NAME = String::from("NAME=\"");
        this.STARTOPTION = String::from("<OPTION");
        this.SELETED = String::from("SELECTED");
        this.ENDOPTION = String::from("</OPTION>");
        this.multiple = String::from("multiple");
        this.isMultipleSelect = false;
        this.before = before.clone();
        this.size = size.clone();
        this.name = name.clone();
        this.after = after.clone();
        this.options = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut before: String, mut size: String, mut name: String, mut after: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(before, size, name, after))))
    }
    

    pub fn addOption(&mut self, mut key: String) {
        jcall!(self.options).add(crate::java_into_object(key.clone()));
    }
    

    pub fn addOptions(&mut self, mut fileName: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut lineReader: Option<Rc<RefCell<LineReader>>> = LineReader::new(fileName.clone());
            while jcall!(lineReader).hasNext() {
                let mut option: String = jcall!(lineReader).next();
                self.addOption(option.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            self.addOption(String::from("Error"));
            }
        }
    }
    

    pub fn setSelected(&mut self, mut selected: String) {
        self.selected = selected.clone();
    }
    

    pub fn isMultiple(&mut self) -> bool {
        return self.isMultipleSelect.clone();
    }
    

    pub fn setMultiple(&mut self, mut value: bool) {
        self.isMultipleSelect = value.clone();
    }
    

    pub fn getOptions(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut optionsArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.options).toArray();
        let mut optionsSize: i32 = (optionsArray.len() as i32).clone();
        for i in (0..optionsSize) {
            let mut value: String = (optionsArray[(i) as usize]).clone() /* as String */;
            if value.is_some() && jcall!(value).compareTo(jcall!(stringUtil).EMPTY_STRING.clone()) != 0 {
                result += &self.STARTOPTION.clone();
                result += &self.END.clone();
                result += &value.clone();
                result += &self.ENDOPTION.clone();
            }
        }
        return result.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut attributeKeys: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(jcall!(self.otherAttributes).keySet()).toArray();
        let mut attributeSize: i32 = (attributeKeys.len() as i32).clone();
        result = self.before.clone();
        result += &self.STARTSELECT.clone();
        result += &self.NAME.clone();
        result += &self.name.clone();
        result += &String::from("\" ");
        result += &self.SIZE.clone();
        result += &self.size.clone();
        result += &String::from("\" ");
        for i in (0..attributeSize) {
            let mut key: String = (attributeKeys[(i) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(self.base.otherAttributes).get(key.clone())).clone() /* as String */;
            if value.is_some() && jcall!(value).compareTo(jcall!(stringUtil).EMPTY_STRING.clone()) != 0 {
                result += &key.clone();
                result += &String::from("=\"");
                result += &value.clone();
                result += &String::from("\" ");
            }
        }
        if self.isMultiple() {
            result += &format!("{}{}", format!("{}{}", String::from(" "), self.multiple), String::from(" "));
        }
        result += &self.END.clone();
        result += &self.getOptions();
        result += &self.ENDSELECT.clone();
        result += &self.after.clone();
        return result.clone();
    }
}
impl std::ops::Deref for HtmlSelect {
    type Target = HtmlTag;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HtmlSelect {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTagDyn for HtmlSelect {
    fn as_HtmlTag(&self) -> &crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &self.base }
    fn as_HtmlTag_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::dhtml::html::HtmlTag::HtmlTag { &mut self.base }
}


