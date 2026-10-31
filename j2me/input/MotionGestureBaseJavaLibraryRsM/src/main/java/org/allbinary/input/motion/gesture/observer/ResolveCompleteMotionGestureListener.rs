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
use crate::org::allbinary::game::input::CompleteMotionGestureInputEventHandler::CompleteMotionGestureInputEventHandler;
use crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEvent;
use crate::org::allbinary::game::input::motion::action::GameKeyFromMotionGestureEventListener::GameKeyFromMotionGestureEventListener;
use crate::org::allbinary::input::motion::gesture::MotionGestureToMotionGestureActionAssociation::MotionGestureToMotionGestureActionAssociation;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfiguration::MotionGestureConfiguration;
use crate::org::allbinary::input::motion::gesture::configuration::MotionGestureConfigurationFactory::MotionGestureConfigurationFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::CompleteMotionGestureListenerInterface::CompleteMotionGestureListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ResolveCompleteMotionGestureListener {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl CompleteMotionGestureListenerInterface for ResolveCompleteMotionGestureListener {
    fn onMotionGestureCompleted(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::onMotionGestureCompleted(self, list) }
}

impl ResolveCompleteMotionGestureListener {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(String::from("MotionGesture to CompleteMotionGesture Reciever"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        jcall!(CompleteMotionGestureInputEventHandler::getInstance()).addListener(GameKeyFromMotionGestureEventListener::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::input::CompleteMotionGestureInputEventListener::CompleteMotionGestureInputEventListenerDyn>>));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn onMotionGestureCompleted(&mut self, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut configuration: Option<Rc<RefCell<MotionGestureConfiguration>>> = MotionGestureConfigurationFactory::getInstance();
        let mut commandActionsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(configuration).getAssociateCommandActionsList();
        let mut size: i32 = jcall!(commandActionsList).size();
        for index in (0..=size - 1).rev() {
            let mut association: Option<Rc<RefCell<MotionGestureToMotionGestureActionAssociation>>> = { let java_object = (jcall!(commandActionsList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<MotionGestureToMotionGestureActionAssociation>>>>().expect("ClassCastException").clone()) };
            if jcall!(association).isMotionGestureArrayEquals(list.clone()) == true {
                let mut completeMotionGestureInputEvent: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::motion::action::GameKeyCompleteMotionGestureInputEvent::GameKeyCompleteMotionGestureInputEventDyn>>> = (jcall!(association).getCommandAction()).clone() /* as GameKeyCompleteMotionGestureInputEvent */;
                jcall!(CompleteMotionGestureInputEventHandler::getInstance()).fireEvent(completeMotionGestureInputEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
                break;
            }
        }
    }
}

