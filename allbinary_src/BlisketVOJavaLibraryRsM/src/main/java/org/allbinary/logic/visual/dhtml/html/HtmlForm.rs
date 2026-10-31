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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInput;
use crate::org::allbinary::logic::visual::dhtml::html::table::HtmlTable::HtmlTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HtmlForm {
    pub FORMBEGIN: String,
    pub END: String,
    pub FORMEND: String,
    pub ACTION: String,
    pub GET: String,
    pub POST: String,
    pub METHODS: Vec<String>,
    pub METHOD: String,
    pub method: String,
    pub action: String,
    pub inputs: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl HtmlForm {
    

    pub fn new_struct(mut action: String) -> Self {
        let mut this = Self {
            FORMBEGIN: String::new(),
            END: String::new(),
            FORMEND: String::new(),
            ACTION: String::new(),
            GET: String::new(),
            POST: String::new(),
            METHODS: Vec::new(),
            METHOD: String::new(),
            method: String::new(),
            action: String::new(),
            inputs: None,
        };
        this.FORMBEGIN = String::from("<form ");
        this.END = String::from(">");
        this.FORMEND = String::from("</form>");
        this.ACTION = String::from("ACTION=\"");
        this.GET = String::from("GET\"");
        this.POST = String::from("POST\"");
        this.METHODS = vec![this.GET.clone(), this.POST.clone()];
        this.METHOD = String::from("METHOD=\"");
        this.action = action.clone();
        this.inputs = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new(mut action: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(action))))
    }
    

    pub fn setPost(&mut self) {
        self.method = self.POST.clone();
    }
    

    pub fn setGet(&mut self) {
        self.method = self.GET.clone();
    }
    

    pub fn addInput(&mut self, mut htmlInput: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInputDyn>>>) {
        jcall!(self.inputs).add(crate::java_into_object(htmlInput.clone()));
    }
    

    pub fn addTable(&mut self, mut htmlTable: Option<Rc<RefCell<HtmlTable>>>) {
        jcall!(self.inputs).add(crate::java_into_object(htmlTable.clone()));
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut result: String = jcall!(stringUtil).EMPTY_STRING.clone();
        let mut inputArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(self.inputs).toArray();
        let mut inputSize: i32 = (inputArray.len() as i32).clone();
        result = self.FORMBEGIN.clone();
        result += &self.METHOD.clone();
        result += &self.method.clone();
        result += &String::from("\" ");
        result += &self.ACTION.clone();
        result += &self.action.clone();
        result += &String::from("\" ");
        result += &self.END.clone();
        for i in (0..inputSize) {
            result += &{ let value = jcall!(inputArray[(i) as usize]); value.as_ref().downcast_ref::<String>().cloned().unwrap_or_else(|| format!("{:?}", value.as_ref().type_id())) };
            result += &String::from(" ");
        }
        result += &self.FORMEND.clone();
        return result.clone();
    }
}
pub trait HtmlFormDyn {
    fn as_HtmlForm(&self) -> &HtmlForm;
    fn as_HtmlForm_mut(&mut self) -> &mut HtmlForm;
    fn setPost(&mut self) { HtmlForm::setPost(self.as_HtmlForm_mut()) }
    fn setGet(&mut self) { HtmlForm::setGet(self.as_HtmlForm_mut()) }
    fn addInput(&mut self, mut htmlInput: Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::dhtml::html::input::HtmlInput::HtmlInputDyn>>>) { HtmlForm::addInput(self.as_HtmlForm_mut(), htmlInput) }
    fn addTable(&mut self, mut htmlTable: Option<Rc<RefCell<HtmlTable>>>) { HtmlForm::addTable(self.as_HtmlForm_mut(), htmlTable) }
    fn toString(&mut self) -> String { HtmlForm::toString(self.as_HtmlForm_mut()) }
}
impl HtmlFormDyn for HtmlForm {
    fn as_HtmlForm(&self) -> &HtmlForm { self }
    fn as_HtmlForm_mut(&mut self) -> &mut HtmlForm { self }
}


