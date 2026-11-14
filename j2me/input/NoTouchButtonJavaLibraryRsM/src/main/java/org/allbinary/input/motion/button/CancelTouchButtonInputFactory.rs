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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use super::TouchButtonInput::TouchButtonInput;
use super::BasicTouchInputFactory::BasicTouchInputFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CancelTouchButtonInputFactory {
    pub cancelInputArray: Vec<Option<Rc<RefCell<TouchButtonInput>>>>,
}

pub static mut CancelTouchButtonInputFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone());

impl CancelTouchButtonInputFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CancelTouchButtonInputFactory>>> {
        if match (&unsafe { CancelTouchButtonInputFactory_instance.clone() }, &{ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CancelTouchButtonInputFactory_instance = crate::java_into_object(CancelTouchButtonInputFactory::new()) };
        }
        return { let java_object = (unsafe { CancelTouchButtonInputFactory_instance.clone() }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<CancelTouchButtonInputFactory>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            cancelInputArray: Vec::new(),
        };
        this.cancelInputArray = vec![None; ({ let java_field = jcall!(InputFactory::getInstance()).MAX.clone(); java_field }) as usize];
        let mut basicTouchInputFactory: Option<Rc<RefCell<BasicTouchInputFactory>>> = BasicTouchInputFactory::getInstance();
        for index in (0..=(this.cancelInputArray.len() as i32) - 1).rev() {
            { let java_index = (index) as usize; this.cancelInputArray[java_index] = { let java_field = jcall!(basicTouchInputFactory).NONE.clone(); java_field }.clone(); };
        }
        { let java_index = (jcall!({ let java_field = jcall!(basicTouchInputFactory).UP.clone(); java_field }).getId()) as usize; this.cancelInputArray[java_index] = { let java_field = jcall!(basicTouchInputFactory).DOWN.clone(); java_field }.clone(); };
        { let java_index = (jcall!({ let java_field = jcall!(basicTouchInputFactory).DOWN.clone(); java_field }).getId()) as usize; this.cancelInputArray[java_index] = { let java_field = jcall!(basicTouchInputFactory).UP.clone(); java_field }.clone(); };
        { let java_index = (jcall!({ let java_field = jcall!(basicTouchInputFactory).LEFT.clone(); java_field }).getId()) as usize; this.cancelInputArray[java_index] = { let java_field = jcall!(basicTouchInputFactory).RIGHT.clone(); java_field }.clone(); };
        { let java_index = (jcall!({ let java_field = jcall!(basicTouchInputFactory).RIGHT.clone(); java_field }).getId()) as usize; this.cancelInputArray[java_index] = { let java_field = jcall!(basicTouchInputFactory).LEFT.clone(); java_field }.clone(); };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCancel(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return self.cancelInputArray[(jcall!(touchButtonInput).getId()) as usize].clone();
    }
}

