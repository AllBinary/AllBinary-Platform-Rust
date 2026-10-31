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

pub static mut PlatformFormInputMappingFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl PlatformFormInputMappingFactory {

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
        if match (&unsafe { PlatformFormInputMappingFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PlatformFormInputMappingFactory_instance = PlatformFormInputMappingFactory::new() };
        }
        return (unsafe { PlatformFormInputMappingFactory_instance.clone() }).clone() /* as PlatformFormInputMappingFactory */;
    }
    

    pub fn getOrCreate(&mut self) -> Option<Rc<RefCell<InputToGameKeyMapping>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.inputToGameKeyMapping, &InputToGameKeyMapping::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                let mut inputToGameKeyMapping: Option<Rc<RefCell<InputToGameKeyMapping>>> = InputToGameKeyMapping::new();
                let mut gameKeyFactory: Option<Rc<RefCell<GameKeyFactory>>> = GameKeyFactory::getInstance();
                jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).UP.clone(), jcall!(gameKeyFactory).UP.clone());
                jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).DOWN.clone(), jcall!(gameKeyFactory).DOWN.clone());
                jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).LEFT.clone(), jcall!(gameKeyFactory).LEFT.clone());
                jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).RIGHT.clone(), jcall!(gameKeyFactory).RIGHT.clone());
                jcall!(inputToGameKeyMapping).add(jcall!(gameKeyFactory).KEY_NUM1.clone(), jcall!(gameKeyFactory).GAME_A.clone());
                self.inputToGameKeyMapping = inputToGameKeyMapping.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), e.clone());
            }
        }
        return self.inputToGameKeyMapping.clone();
    }
}

