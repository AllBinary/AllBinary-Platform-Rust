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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32) -> Self {
        let mut this = Self {
            source: None,
            id: 0,
        };
        this.source = source.clone();
        this.id = id.clone();
        unsafe { *MotionEventFactory_index = jcall!({ let java_field = jcall!(TouchMotionGestureFactory::getInstance()).LAST_MOTION.clone(); java_field }).getId() };
        this
    }

    pub fn new(mut source: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut id: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(source, id))))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>> {
        let mut inputFactory: Option<Rc<RefCell<InputFactory>>> = InputFactory::getInstance();
        let mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>> = jcall!(inputFactory).getInstanceById(unsafe { let value = MotionEventFactory_index.clone(); *MotionEventFactory_index += 1; value });
        if match (&input, &{ let java_field = jcall!(inputFactory).NO_INPUT.clone(); java_field }) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return MotionGestureEvent::new(crate::java_into_object(self.source.clone()), self.id.clone(), ({ let java_field = jcall!(TouchMotionGestureFactory::getInstance()).NO_MOTION.clone(); java_field }).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        } else {
            return MotionGestureEvent::new(crate::java_into_object(self.source.clone()), self.id.clone(), (input).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<MotionGestureInput>() }, "ClassCastException: not a MotionGestureInput"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<MotionGestureInput>) } })).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        }
    }
}

