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
use super::Input::Input;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformKeyFactory {
}

pub static mut PlatformKeyFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl PlatformKeyFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlatformKeyFactory>>> {
        if match (&unsafe { PlatformKeyFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *PlatformKeyFactory_instance = crate::java_into_object(PlatformKeyFactory::new()) };
        }
        return { let java_object = (unsafe { PlatformKeyFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PlatformKeyFactory>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getString(&mut self, mut keyCode: i32) -> String {
        return PCGameKey::PCGameKey::getString(keyCode.clone());
    }
    

    pub fn isSubmission(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isSubmission(input.clone());
    }
    

    pub fn isDelete(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isDelete(input.clone());
    }
    

    pub fn isBackSpace(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isBackSpace(input.clone());
    }
    

    pub fn isUp(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isUp(input.clone());
    }
    

    pub fn isDown(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isDown(input.clone());
    }
    

    pub fn isEnter(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isEnter(input.clone());
    }
    

    pub fn isLeft(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isLeft(input.clone());
    }
    

    pub fn isRight(&mut self, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> bool {
        return jcall!(PCKeyFactory::PCKeyFactory::getInstance()).isRight(input.clone());
    }
}

