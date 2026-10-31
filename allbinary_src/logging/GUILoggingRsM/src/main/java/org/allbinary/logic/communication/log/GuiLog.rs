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
// use crate::java::awt::*;
// use crate::javax::swing::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GuiLog {
    pub logFormatUtil: Option<Rc<RefCell<LogFormatUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

thread_local! { pub static GuiLog_instance: Option<Rc<RefCell<GuiLog>>> = GuiLog::new(); }

impl GuiLog {
    

    pub fn getInstance() -> Option<Rc<RefCell<GuiLog>>> {
        return GuiLog_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logFormatUtil: None,
            commonStrings: None,
        };
        this.logFormatUtil = LogFormatUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn showDialog(&mut self, mut msg: String) {
        let mut error: Option<Rc<RefCell<JDialog>>> = JDialog::new();
        let mut size: i32 = (msg.chars().count() as i32) * 9;
        let mut x: i32 = size.clone();
        if x > 550 {
            x = 550;
        }
        jcall!(jcall!(error).getContentPane()).setLayout(GridLayout::new(1, 1));
        if size > 255 {
            let mut y: i32 = 550;
            jcall!(error).setSize(x.clone(), y.clone());
            jcall!(jcall!(error).getContentPane()).add(JScrollPane::new(JTextArea::new(msg.clone())));
        } else if size > 0 {
            let mut y: i32 = 50;
            jcall!(error).setSize(x.clone(), y.clone());
            jcall!(jcall!(error).getContentPane()).add(JScrollPane::new(JLabel::new(msg.clone())));
        }
        jcall!(error).show();
    }
    

    // synchronized
    pub fn put(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String) -> String {
        return self.put4(specialMessage.clone(), anyType.clone(), functionName.clone(), None);
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put4(&mut self, mut specialMessage: String, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut functionName: String, mut exception: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut data: String = { let java_receiver = self.logFormatUtil.clone(); let java_arguments = (specialMessage.clone(), std::any::type_name_of_val(&anyType).to_string(), functionName.clone(), crate::java_into_object(exception.clone()),); let java_result = jcall!(java_receiver).get(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); java_result };
            self.showDialog(data.clone());
            println!("{}", data.clone());
            return data.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("put"), e.clone());
            return String::from("Logging Error");
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put(&mut self, mut specialMessage: String, mut className: String, mut functionName: String) -> String {
        return self.put4(specialMessage.clone(), className.clone(), functionName.clone(), None);
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn put4(&mut self, mut specialMessage: String, mut className: String, mut functionName: String, mut exception: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut data: String = jcall!(self.logFormatUtil).get(specialMessage.clone(), className.clone(), functionName.clone(), crate::java_into_object(exception.clone()));
            self.showDialog(data.clone());
            println!("{}", data.clone());
            return data.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            PreLogUtil::PreLogUtil::putOE(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("put"), e.clone());
            return String::from("Logging Error");
            }
        }
    }
}

