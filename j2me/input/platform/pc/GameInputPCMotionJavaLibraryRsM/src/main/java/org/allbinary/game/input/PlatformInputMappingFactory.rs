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
use crate::org::allbinary::game::input::mapping::PersistentInputMapping::PersistentInputMapping;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::MotionLaptopGameInputMapping::MotionLaptopGameInputMapping;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformInputMappingFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub inputToGameKeyMapping: Option<Rc<RefCell<PersistentInputMapping>>>,
}

thread_local! { pub static PlatformInputMappingFactory_instance: Option<Rc<RefCell<PlatformInputMappingFactory>>> = PlatformInputMappingFactory::new(); }

impl PlatformInputMappingFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            inputToGameKeyMapping: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.inputToGameKeyMapping = PersistentInputMapping::getNullInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlatformInputMappingFactory>>> {
        return PlatformInputMappingFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getPersistentInputMappingInstance(&mut self) -> Option<Rc<RefCell<PersistentInputMapping>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if match (&self.inputToGameKeyMapping, &PersistentInputMapping::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.inputToGameKeyMapping = MotionLaptopGameInputMapping::new().map(|reference| Rc::new(RefCell::new(Rc::try_unwrap(reference).ok().expect("new object is not shared").into_inner().base.base)));
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

