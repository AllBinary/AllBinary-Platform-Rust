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
// package org.allbinary.graphics.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::FormType::FormType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FormTypeFactory {
    pub UNK: String,
    pub NULL_FORM_TYPE: Option<Rc<RefCell<FormType>>>,
    pub TEMP_HORIZONTAL_FORM: Option<Rc<RefCell<FormType>>>,
    pub HORIZONTAL_FORM: Option<Rc<RefCell<FormType>>>,
    pub VERTICAL_CENTER_FORM: Option<Rc<RefCell<FormType>>>,
}

thread_local! { pub static FormTypeFactory_instance: Option<Rc<RefCell<FormTypeFactory>>> = FormTypeFactory::new(); }

impl FormTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            UNK: String::new(),
            NULL_FORM_TYPE: None,
            TEMP_HORIZONTAL_FORM: None,
            HORIZONTAL_FORM: None,
            VERTICAL_CENTER_FORM: None,
        };
        this.UNK = String::from("Unknown FormType");
        this.NULL_FORM_TYPE = FormType::new(jcall!(StringUtil::getInstance()).NULL_STRING.clone());
        this.TEMP_HORIZONTAL_FORM = FormType::new(String::from("TempHorizontal"));
        this.HORIZONTAL_FORM = FormType::new(String::from("Horizontal"));
        this.VERTICAL_CENTER_FORM = FormType::new(String::from("Vertical"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FormTypeFactory>>> {
        return FormTypeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getFormType(&mut self) -> Option<Rc<RefCell<FormType>>> {
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut formType: Option<Rc<RefCell<FormType>>> = None;
        if jcall!(displayInfo).isPortrait() {
            formType = self.VERTICAL_CENTER_FORM.clone();
        } else {
            formType = self.HORIZONTAL_FORM.clone();
        }
        return formType.clone();
    }
}

