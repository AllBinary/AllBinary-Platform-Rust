// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.ag.layer.hud.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::NumberStringHud::NumberStringHud;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberStringHudFactory {
    pub NULL_NUMBER_STRING_HUD: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>>,
}

pub static mut NumberStringHudFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| NullUtil::getInstance());

impl NumberStringHudFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_NUMBER_STRING_HUD: None,
        };
        this.NULL_NUMBER_STRING_HUD = this.createHud();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<NumberStringHudFactory>>> {
        if match (&unsafe { NumberStringHudFactory_instance.clone() }, &NullUtil::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *NumberStringHudFactory_instance = NumberStringHudFactory::new() };
        }
        return (unsafe { NumberStringHudFactory_instance.clone() }).clone() /* as NumberStringHudFactory */;
    }
    

    pub fn createHud(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return NumberStringHud::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), 9, jcall!(BasicHudFactory::getInstance()).ABSOLUTE.clone(), 1, 0, jcall!(BasicColorFactory::getInstance()).NULL_COLOR.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn>>);
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(e.clone()));
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
            }
        }
    }
}

