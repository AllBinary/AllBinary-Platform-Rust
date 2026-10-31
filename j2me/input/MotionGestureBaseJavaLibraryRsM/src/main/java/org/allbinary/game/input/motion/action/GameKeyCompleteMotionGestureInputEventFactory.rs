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
// package org.allbinary.game.input.motion.action
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GameKeyCompleteMotionGestureInputEventFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub eventList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static GameKeyCompleteMotionGestureInputEventFactory_SINGLETON: Option<Rc<RefCell<GameKeyCompleteMotionGestureInputEventFactory>>> = GameKeyCompleteMotionGestureInputEventFactory::new(); }

impl GameKeyCompleteMotionGestureInputEventFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            eventList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.eventList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GameKeyCompleteMotionGestureInputEventFactory>>> {
        return GameKeyCompleteMotionGestureInputEventFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut gameKeyCompleteMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>>) {
        jcall!(self.eventList).add(crate::java_into_object(gameKeyCompleteMotionGestureInputEvent.clone()));
    }
    

    pub fn updateAll(&mut self) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).START_LABEL.clone()).appendint(jcall!(self.eventList).size()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("updateAll"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        for index in (0..=jcall!(self.eventList).size() - 1).rev() {
            let mut gameKeyCompleteMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> = { let java_object = (jcall!(self.eventList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>>>().expect("ClassCastException").clone()) };
            jcall!(gameKeyCompleteMotionGestureInputEvent).update();
        }
    }
}

