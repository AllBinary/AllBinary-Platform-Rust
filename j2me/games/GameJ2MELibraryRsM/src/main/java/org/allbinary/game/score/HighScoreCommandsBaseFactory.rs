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
use crate::org::allbinary::J2MEUtil::J2MEUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HighScoreCommandsBaseFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub HIGH_SCORE_COMMANDS: Vec<Option<Rc<RefCell<Command>>>>,
}


impl HighScoreCommandsBaseFactory {
    

    pub fn new_struct(mut HIGH_SCORE_COMMANDS: Vec<Option<Rc<RefCell<Command>>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            HIGH_SCORE_COMMANDS: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.HIGH_SCORE_COMMANDS = HIGH_SCORE_COMMANDS.clone();
        this
    }

    pub fn new(mut HIGH_SCORE_COMMANDS: Vec<Option<Rc<RefCell<Command>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(HIGH_SCORE_COMMANDS))))
    }
    

    pub fn isHighScoreCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> bool {
        for index in (0..=(self.HIGH_SCORE_COMMANDS.len() as i32) - 1).rev() {
            if match (&command, &self.HIGH_SCORE_COMMANDS[(index) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn getIndex(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> i32 {
        for index in (0..=(self.HIGH_SCORE_COMMANDS.len() as i32) - 1).rev() {
            if match (&command, &self.HIGH_SCORE_COMMANDS[(index) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return index.clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Such Command"))));
    }
}
pub trait HighScoreCommandsBaseFactoryDyn {
    fn as_HighScoreCommandsBaseFactory(&self) -> &HighScoreCommandsBaseFactory;
    fn as_HighScoreCommandsBaseFactory_mut(&mut self) -> &mut HighScoreCommandsBaseFactory;
    fn isHighScoreCommand(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> bool { HighScoreCommandsBaseFactory::isHighScoreCommand(self.as_HighScoreCommandsBaseFactory_mut(), command) }
    fn getIndex(&mut self, mut command: Option<Rc<RefCell<Command>>>) -> i32 { HighScoreCommandsBaseFactory::getIndex(self.as_HighScoreCommandsBaseFactory_mut(), command) }
}
impl HighScoreCommandsBaseFactoryDyn for HighScoreCommandsBaseFactory {
    fn as_HighScoreCommandsBaseFactory(&self) -> &HighScoreCommandsBaseFactory { self }
    fn as_HighScoreCommandsBaseFactory_mut(&mut self) -> &mut HighScoreCommandsBaseFactory { self }
}


