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
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::input::motion::gesture::MotionGestureInput::MotionGestureInput;
use crate::org::allbinary::input::motion::gesture::TouchMotionGestureFactory::TouchMotionGestureFactory;
use crate::org::allbinary::logic::util::event::AllBinaryEventCircularPool::AllBinaryEventCircularPool;
use super::MotionEventFactory::MotionEventFactory;
use super::MotionGestureEvent::MotionGestureEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MotionEventCircularPool {
    pub MIN: i32,
    pub eventPool: Option<Rc<RefCell<AllBinaryEventCircularPool>>>,
}


impl MotionEventCircularPool {
    

    pub fn createPool(mut id: i32) -> Option<Rc<RefCell<MotionEventCircularPool>>> {
        return MotionEventCircularPool::new(id.clone());
    }
    

    pub fn new_struct(mut id: i32) -> Self {
        let mut this = Self {
            MIN: 0,
            eventPool: None,
        };
        this.MIN = jcall!(jcall!(TouchMotionGestureFactory::getInstance()).LAST_MOTION).getId();
        this.eventPool = AllBinaryEventCircularPool::new((jcall!(InputFactory::getInstance()).MAX - 1) - this.MIN);
        jcall!(this.eventPool).initAllBinaryEventCircularPool(MotionEventFactory::new(this.eventPool.clone(), id.clone()));
        this
    }

    pub fn new(mut id: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id))))
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(&mut self, mut motionGestureInput: Option<Rc<RefCell<MotionGestureInput>>>) -> Option<Rc<RefCell<MotionGestureEvent>>> {
        return { let java_object = ({ let java_receiver = self.eventPool.clone(); let java_arguments = (jcall!(motionGestureInput).getId() - self.MIN,); let java_result = jcall!(java_receiver).getInstance(java_arguments.0); java_result }).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionGestureEvent>>>>().expect("ClassCastException").clone()) };
    }
}

