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
// package org.allbinary.input.motion.gesture.observer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::AllBinaryEventObjectFactoryInterface::AllBinaryEventObjectFactoryInterface;
use super::MotionGestureEvent::MotionGestureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionEventFactory {
    pub source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub id: i32,
}

pub static mut MotionEventFactory_index: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
impl AllBinaryEventObjectFactoryInterface for MotionEventFactory {
    fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>> { Self::getInstance(self) }
}

impl MotionEventFactory {
    

    pub fn new_struct(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32) -> Self {
        let mut this = Self {
            source: None,
            id: 0,
        };
        this.source = source.clone();
        this.id = id.clone();
        unsafe { *MotionEventFactory_index = jcall!(jcall!(TouchMotionGestureFactory::getInstance()).LAST_MOTION).getId() };
        this
    }

    pub fn new(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(source, id))))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>> {
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(inputFactory).getInstanceById(unsafe { let value = MotionEventFactory_index.clone(); *MotionEventFactory_index += 1; value });
        if match (&input, &jcall!(inputFactory).NO_INPUT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return MotionGestureEvent::new(self.source.clone(), self.id.clone(), (jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION).clone() /* as MotionGestureInput */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        } else {
            return MotionGestureEvent::new(self.source.clone(), self.id.clone(), (input).clone() /* as MotionGestureInput */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        }
    }
}

