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
// package org.allbinary.game.score
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

pub struct HighScoreCommands {
    pub DISPLAY: Option<Rc<RefCell<Command>>>,
    pub PERSONAL: Option<Rc<RefCell<Command>>>,
    pub WORLD: Option<Rc<RefCell<Command>>>,
}

thread_local! { pub static HighScoreCommands_instance: Option<Rc<RefCell<HighScoreCommands>>> = HighScoreCommands::new(); }

impl HighScoreCommands {

    pub fn new_struct() -> Self {
        let mut this = Self {
            DISPLAY: None,
            PERSONAL: None,
            WORLD: None,
        };
        this.DISPLAY = Command::new(String::from("Scores"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.PERSONAL = Command::new(String::from("Personal"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this.WORLD = Command::new(String::from("World"), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), unsafe { crate::javax::microedition::lcdui::Command::Command_SCREEN.clone() }.clone(), 1);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<HighScoreCommands>>> {
        return HighScoreCommands_instance.with(Clone::clone).clone();
    }
}

