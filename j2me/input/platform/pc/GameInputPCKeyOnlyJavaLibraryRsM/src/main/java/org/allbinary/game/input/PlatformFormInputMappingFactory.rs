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
use crate::org::allbinary::game::input::mapping::InputToGameKeyMapping::InputToGameKeyMapping;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;
use super::PCKeyFactory::PCKeyFactory;
use super::GameKeyFactory::GameKeyFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformFormInputMappingFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>>,
}

pub static mut PlatformFormInputMappingFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl PlatformFormInputMappingFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            inputToGameKeyMapping: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.inputToGameKeyMapping = InputToGameKeyMapping::getNullInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlatformFormInputMappingFactory>>> {
        if match (&unsafe { PlatformFormInputMappingFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PlatformFormInputMappingFactory_instance = crate::java_into_object(PlatformFormInputMappingFactory::new()) };
        }
        return { let java_object = (unsafe { PlatformFormInputMappingFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PlatformFormInputMappingFactory>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getOrCreate(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.inputToGameKeyMapping, &InputToGameKeyMapping::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut pcKeyFactory: Option<Rc<RefCell<PCKeyFactory>>> = PCKeyFactory::getInstance();
                let mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>> = InputToGameKeyMapping::new();
                let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
                jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(pcKeyFactory).DPAD_UP.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(pcKeyFactory).DPAD_DOWN.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(pcKeyFactory).DPAD_LEFT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(pcKeyFactory).DPAD_RIGHT.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                jcall!(inputToGameKeyMapping).add({ let java_field = jcall!(gameKeyFactory).KEY_NUM1.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>), { let java_field = jcall!(pcKeyFactory).ENTER.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>));
                self.inputToGameKeyMapping = inputToGameKeyMapping.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).GET_INSTANCE.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
        }
        return self.inputToGameKeyMapping.clone();
    }
}

