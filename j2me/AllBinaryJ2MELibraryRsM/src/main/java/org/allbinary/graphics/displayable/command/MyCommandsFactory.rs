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
use crate::javax::microedition::lcdui::Command::Command;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyCommandsFactory {
    pub NO_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub GAUGE_CHANGE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub SET_DISPLAYABLE: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub PAUSE_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
    pub RESUME_COMMAND: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>>,
}

pub static MyCommandsFactory_instance: crate::JavaStatic<Option<Rc<RefCell<MyCommandsFactory>>>> = crate::JavaStatic::new(|| MyCommandsFactory::new());

impl MyCommandsFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
        this.NO_COMMAND = Command::new4({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), -1, -1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.GAUGE_CHANGE = Command::new4(String::from("GAUGE_CHANGE"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_ITEM.with(Clone::clone).clone(), 1).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.SET_DISPLAYABLE = Command::new4(String::from("Display"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 2).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.PAUSE_COMMAND = Command::new4(String::from("Pause"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 0).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this.RESUME_COMMAND = Command::new4(String::from("Resume"), { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone(), crate::javax::microedition::lcdui::Command::Command_SCREEN.with(Clone::clone).clone(), 0).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Command::CommandDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

