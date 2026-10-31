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
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::java::lang::Thread::*;
use super::KeyboardActionScriptInputInterface::KeyboardActionScriptInputInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyboardInputAutomationProcessor {
}


impl KeyboardInputAutomationProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(mut keyActionScriptInputInterface: Option<Rc<RefCell<dyn KeyboardActionScriptInputInterface>>>) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(format!("{}{}", jcall!(CommonLabels::getInstance()).START, jcall!(keyActionScriptInputInterface).toString()), crate::java_into_object(String::from("KeyInputAutomationProcessor")), jcall!(commonStrings).PROCESS.clone());
        let mut inputRobotInterface: Option<Rc<RefCell<dyn InputRobotInterface>>> = jcall!(keyActionScriptInputInterface).getInputRobotInterface();
        let mut integer: Vec<Option<i32>> = jcall!(keyActionScriptInputInterface).getKeyArray();
        if jcall!(keyActionScriptInputInterface).getDelayBetweenKeys() == 0 {
            for index in (0..(jcall!(keyActionScriptInputInterface).getKeyArray().len() as i32)) {
                if jcall!(keyActionScriptInputInterface).isPress() {
                    jcall!(inputRobotInterface).keyPress(integer[(index) as usize].clone());
                }
            }
            std::thread::sleep(std::time::Duration::from_millis((jcall!(keyActionScriptInputInterface).getTime()) as u64));
            for index in (0..=(jcall!(keyActionScriptInputInterface).getKeyArray().len() as i32) - 1).rev() {
                if jcall!(keyActionScriptInputInterface).isRelease() {
                    jcall!(inputRobotInterface).keyRelease(integer[(index) as usize].clone());
                }
            }
        } else {
            for index in (0..(jcall!(keyActionScriptInputInterface).getKeyArray().len() as i32)) {
                if jcall!(keyActionScriptInputInterface).isPress() {
                    jcall!(inputRobotInterface).keyPress(integer[(index) as usize].clone());
                }
                std::thread::sleep(std::time::Duration::from_millis((jcall!(keyActionScriptInputInterface).getTime()) as u64));
                if jcall!(keyActionScriptInputInterface).isRelease() {
                    jcall!(inputRobotInterface).keyRelease(integer[(index) as usize].clone());
                }
                let mut timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> = TimeDelayHelper::new(jcall!(keyActionScriptInputInterface).getDelayBetweenKeys()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
                let mut toLong: i32 = 0;
                while !jcall!(timeHelper).isTimeTNT() || toLong > 6000 {
                    toLong += 1;
                    std::thread::sleep(std::time::Duration::from_millis((50) as u64));
                }
            }
        }
    }
}

