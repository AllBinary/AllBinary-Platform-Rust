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
// package org.allbinary.content
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::Intent::Intent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommandUriAction {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>,
}

thread_local! { pub static CommandUriAction_instance: Option<Rc<RefCell<CommandUriAction>>> = CommandUriAction::new(); }

impl CommandUriAction {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            hashtable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.hashtable = jcall!(StdUtil::getInstance()).createHashtable();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommandUriAction>>> {
        return CommandUriAction_instance.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut command: Option<Rc<RefCell<Command>>>, mut url: String) {
        jcall!(self.hashtable).put(crate::java_into_object(command.clone()), crate::java_into_object(url.clone()));
    }
    

    pub fn process(&mut self, mut command: Option<Rc<RefCell<Command>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).PROCESS.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getIntent(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> Option<Rc<RefCell<Intent>>> {
        let mut url: String = (jcall!(self.hashtable).get(crate::java_into_object(command.clone()))).clone() /* as String */;
        return self.base.intent.clone();
    }
}

