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

pub static mut CancelTouchButtonInputFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl CancelTouchButtonInputFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<CancelTouchButtonInputFactory>>> {
        if match (&unsafe { CancelTouchButtonInputFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CancelTouchButtonInputFactory_instance = CancelTouchButtonInputFactory::new() };
        }
        return (unsafe { CancelTouchButtonInputFactory_instance.clone() }).clone() /* as CancelTouchButtonInputFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            cancelInputArray: Vec::new(),
        };
        this.cancelInputArray = vec![None; (jcall!(InputFactory::getInstance()).MAX) as usize];
        let mut basicTouchInputFactory: Option<Rc<RefCell<BasicTouchInputFactory>>> = BasicTouchInputFactory::getInstance();
        for index in (0..=(this.cancelInputArray.len() as i32) - 1).rev() {
            this.cancelInputArray[(index) as usize] = jcall!(basicTouchInputFactory).NONE.clone();
        }
        this.cancelInputArray[(jcall!(jcall!(basicTouchInputFactory).UP).getId()) as usize] = jcall!(basicTouchInputFactory).DOWN.clone();
        this.cancelInputArray[(jcall!(jcall!(basicTouchInputFactory).DOWN).getId()) as usize] = jcall!(basicTouchInputFactory).UP.clone();
        this.cancelInputArray[(jcall!(jcall!(basicTouchInputFactory).LEFT).getId()) as usize] = jcall!(basicTouchInputFactory).RIGHT.clone();
        this.cancelInputArray[(jcall!(jcall!(basicTouchInputFactory).RIGHT).getId()) as usize] = jcall!(basicTouchInputFactory).LEFT.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCancel(&mut self, mut touchButtonInput: Option<Rc<RefCell<TouchButtonInput>>>) -> Option<Rc<RefCell<TouchButtonInput>>> {
        return self.cancelInputArray[(jcall!(touchButtonInput).getId()) as usize].clone();
    }
}

