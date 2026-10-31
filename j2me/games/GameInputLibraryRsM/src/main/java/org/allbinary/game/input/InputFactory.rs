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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InputFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub KEY_CODE_LABEL: String,
    pub DEVICE_ID_LABEL: String,
    pub MAX: i32,
    pub inputIntegerArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>>,
    pub NO_INPUT: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>,
}

pub static mut InputFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl InputFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<InputFactory>>> {
        if match (&unsafe { InputFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *InputFactory_instance = InputFactory::new() };
        }
        return (unsafe { InputFactory_instance.clone() }).clone() /* as InputFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            KEY_CODE_LABEL: String::new(),
            DEVICE_ID_LABEL: String::new(),
            MAX: 0,
            inputIntegerArray: Vec::new(),
            NO_INPUT: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.KEY_CODE_LABEL = String::from("KeyCode: ");
        this.DEVICE_ID_LABEL = String::from("DeviceId: ");
        this.MAX = jcall!(SmallIntegerSingletonFactory::getInstance()).getMin();
        this.inputIntegerArray = vec![None; (this.MAX) as usize];
        this.NO_INPUT = Input::new(0, jcall!(this.commonStrings).UNKNOWN.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>);
        let mut size: i32 = (this.inputIntegerArray.len() as i32).clone();
        for index in (0..size) {
            this.inputIntegerArray[(index) as usize] = this.NO_INPUT.clone();
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut id: i32, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) {
        self.inputIntegerArray[(id) as usize] = input.clone();
    }
    

    pub fn getInstanceById(&mut self, mut id: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> {
        if id < 0 {
            id = -id;
        }
        if id > (self.inputIntegerArray.len() as i32) {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("Warning id: ")).appendint(id.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET_INSTANCE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            return self.NO_INPUT.clone();
        }
        return self.inputIntegerArray[(id) as usize].clone();
    }
}

