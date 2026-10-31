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
// package org.allbinary.graphics.displayable.command
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCommandsFactory {
    pub NO_COMMAND: Option<Rc<RefCell<Command>>>,
    pub GAUGE_CHANGE: Option<Rc<RefCell<Command>>>,
    pub SET_DISPLAYABLE: Option<Rc<RefCell<Command>>>,
    pub PAUSE_COMMAND: Option<Rc<RefCell<Command>>>,
    pub RESUME_COMMAND: Option<Rc<RefCell<Command>>>,
}

thread_local! { pub static MyCommandsFactory_instance: Option<Rc<RefCell<MyCommandsFactory>>> = MyCommandsFactory::new(); }

impl MyCommandsFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<MyCommandsFactory>>> {
        return MyCommandsFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NO_COMMAND: None,
            GAUGE_CHANGE: None,
            SET_DISPLAYABLE: None,
            PAUSE_COMMAND: None,
            RESUME_COMMAND: None,
        };
        this.NO_COMMAND = Command::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), -1, -1);
        this.GAUGE_CHANGE = Command::new(String::from("GAUGE_CHANGE"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_ITEM.clone() }.clone(), 1);
        this.SET_DISPLAYABLE = Command::new(String::from("Display"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 2);
        this.PAUSE_COMMAND = Command::new(String::from("Pause"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 0);
        this.RESUME_COMMAND = Command::new(String::from("Resume"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

