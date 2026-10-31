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
// package org.allbinary.game.input.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::input::Input::Input;
use crate::org::allbinary::game::input::InputFactory::InputFactory;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::GameKeyEvent::GameKeyEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyEventFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub TOUCH_BUTTON_SOURCE_ID: i32,
    pub MOTION_GESTURE_SOURCE_ID: i32,
    pub MAX_SOURCES: i32,
    pub ARRAY: Vec<Vec<Option<Rc<RefCell<GameKeyEvent>>>>>,
}

pub static mut GameKeyEventFactory_instance: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());

impl GameKeyEventFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyEventFactory>>> {
        if match (&unsafe { GameKeyEventFactory_instance.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *GameKeyEventFactory_instance = GameKeyEventFactory::new() };
        }
        return (unsafe { GameKeyEventFactory_instance.clone() }).clone() /* as GameKeyEventFactory */;
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            TOUCH_BUTTON_SOURCE_ID: 0,
            MOTION_GESTURE_SOURCE_ID: 0,
            MAX_SOURCES: 0,
            ARRAY: Vec::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.TOUCH_BUTTON_SOURCE_ID = 2;
        this.MOTION_GESTURE_SOURCE_ID = 3;
        this.MAX_SOURCES = 4;
        this.ARRAY = vec![vec![None; (jcall!(InputFactory::getInstance()).MAX) as usize]; (this.MAX_SOURCES) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        let mut nullUtil: Option<Rc<RefCell<NullUtil>>> = NullUtil::getInstance();
        let mut size: i32 = jcall!(InputFactory::getInstance()).MAX.clone();
        {
            let mut index: i32 = self.MAX_SOURCES.clone();
            while { index -= 1; index } >= 0 {
            {
                let mut index2: i32 = size.clone();
                while { index2 -= 1; index2 } >= 0 {
                self.ARRAY[(index) as usize][(index2) as usize] = GameKeyEvent::createEvent(jcall!(nullUtil).NULL_OBJECT.clone(), index.clone(), index2.clone());
                }
            }
            }
        }
    }
    

    // throws Exception
    pub fn getInstanceForKey(&mut self, mut anyType: Option<Rc<RefCell<dyn GameKeyEventSourceInterface>>>, mut key: i32) -> Option<Rc<RefCell<GameKeyEvent>>> {
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = self.ARRAY[(jcall!(anyType).getSourceId()) as usize][(key) as usize].clone();
        return gameKeyEvent.clone();
    }
    

    // throws Exception
    pub fn getInstanceForInput(&mut self, mut anyType: Option<Rc<RefCell<dyn GameKeyEventSourceInterface>>>, mut input: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::Input::InputDyn>>>) -> Option<Rc<RefCell<GameKeyEvent>>> {
        let mut gameKeyEvent: Option<Rc<RefCell<GameKeyEvent>>> = self.ARRAY[(jcall!(anyType).getSourceId()) as usize][(jcall!(input).getId()) as usize].clone();
        return gameKeyEvent.clone();
    }
}

