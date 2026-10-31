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
// package org.allbinary.input.automation.actions.script.condition.processors.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::input::automation::robot::InputRobotInterface::InputRobotInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use super::MouseActionScriptInputInterface::MouseActionScriptInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MouseInputAutomationProcessor {
}


impl MouseInputAutomationProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(mut mouseActionScriptInputInterface: Option<Rc<RefCell<dyn MouseActionScriptInputInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(jcall!(StringMaker::new()).append(String::from("Start - Processing Mouse Input at: ")).append(jcall!(jcall!(mouseActionScriptInputInterface).getPoint()).toString()).append(String::from(" clicks: ")).appendint(jcall!(mouseActionScriptInputInterface).getButtonClicks()).toString(), crate::java_into_object(String::from("MouseInputAutomationProcessor")), jcall!(commonStrings).PROCESS.clone());
        let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = jcall!(mouseActionScriptInputInterface).getInputRobotInterface();
        if jcall!(jcall!(mouseActionScriptInputInterface).getPoint()).x != -1 && jcall!(jcall!(mouseActionScriptInputInterface).getPoint()).y != -1 {
            jcall!(inputRobotInterface).mouseMove(jcall!(mouseActionScriptInputInterface).getPoint());
        }
        std::thread::sleep(std::time::Duration::from_millis((jcall!(mouseActionScriptInputInterface).getTime()) as u64));
        if jcall!(mouseActionScriptInputInterface).getButtonClicks() != 0 {
            jcall!(inputRobotInterface).mousePress(jcall!(mouseActionScriptInputInterface).getButtonClicks());
            std::thread::sleep(std::time::Duration::from_millis((jcall!(mouseActionScriptInputInterface).getTime()) as u64));
            jcall!(inputRobotInterface).mouseRelease(jcall!(mouseActionScriptInputInterface).getButtonClicks());
        }
        std::thread::sleep(std::time::Duration::from_millis((jcall!(mouseActionScriptInputInterface).getTime()) as u64));
    }
}

