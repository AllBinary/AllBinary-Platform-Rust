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
use crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInput;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::UpKeyEventListenerInterface::UpKeyEventListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpKeyEventHandlerBase {
    pub base: BasicEventHandler,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static UpKeyEventHandlerBase_TOTAL_LISTENERS: String = String::from(" Total PlayerGameInput Listeners: "); }
thread_local! { pub static UpKeyEventHandlerBase_LISTENER_LABEL: String = String::from(" PlayerGameInput Listener: "); }

impl UpKeyEventHandlerBase {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            list: None,
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) {
        if !jcall!(self.list).contains(crate::java_into_object(playerGameInput.clone())) {
            jcall!(self.list).add(crate::java_into_object(playerGameInput.clone()));
        }
    }
    

    pub fn removeAllListeners(&mut self) {
        jcall!(self.list).clear();
        self.base.removeAllListeners();
    }
    

    pub fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(self.list).remove(crate::java_into_object(eventListenerInterface.clone()));
        self.base.removeListenerSingleThreaded(eventListenerInterface.clone());
    }
    

    // synchronized
    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(self.list).remove(crate::java_into_object(eventListenerInterface.clone()));
        self.base.removeListener(eventListenerInterface.clone());
    }
    

    // throws Exception
    pub fn fireEventForInteger(&mut self, mut eventObject: Option<i32>) {
        {
            let mut index: i32 = jcall!(self.list).size();
            while { index -= 1; index } >= 0 {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(playerGameInput).onUpKeyEvent(eventObject.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(EventStrings::getInstance()).FIRE_EVENT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
            }
            }
        }
        let mut index: i32 = 0;
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        while index < jcall!(self.eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = jcall!(self.eventListenerInterfaceList).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
                self.processForInteger(eventObject.clone(), eventListenerInterface.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(EventStrings::getInstance()).FIRE_EVENT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
            }
            index += 1;
        }
    }
    

    // throws Exception
    pub fn processForInteger(&mut self, mut eventObject: Option<i32>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        let mut upKeyEventListenerInterface: Option<Rc<RefCell<dyn UpKeyEventListenerInterface>>> = panic!("ClassCastException: a Rust dyn EventListenerInterface cannot be cast to dyn UpKeyEventListenerInterface");
        jcall!(upKeyEventListenerInterface).onUpKeyEvent(eventObject.clone());
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.base.toString());
        jcall!(stringBuffer).append(UpKeyEventHandlerBase_TOTAL_LISTENERS.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(jcall!(self.list).size());
        for index in (0..jcall!(self.list).size()) {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = { let java_object = jcall!(self.list).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
                jcall!(stringBuffer).append(UpKeyEventHandlerBase_LISTENER_LABEL.with(Clone::clone).clone());
                jcall!(stringBuffer).append(jcall!(eventListenerInterface).toString());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).TOSTRING.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for UpKeyEventHandlerBase {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpKeyEventHandlerBase {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait UpKeyEventHandlerBaseDyn: crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn {
    fn as_UpKeyEventHandlerBase(&self) -> &UpKeyEventHandlerBase;
    fn as_UpKeyEventHandlerBase_mut(&mut self) -> &mut UpKeyEventHandlerBase;
    fn addListener(&mut self, mut playerGameInput: Option<Rc<RefCell<dyn crate::org::allbinary::game::input::PlayerGameInput::PlayerGameInputDyn>>>) { UpKeyEventHandlerBase::addListener(self.as_UpKeyEventHandlerBase_mut(), playerGameInput) }
    fn fireEventForInteger(&mut self, mut eventObject: Option<i32>) { UpKeyEventHandlerBase::fireEventForInteger(self.as_UpKeyEventHandlerBase_mut(), eventObject) }
    fn processForInteger(&mut self, mut eventObject: Option<i32>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { UpKeyEventHandlerBase::processForInteger(self.as_UpKeyEventHandlerBase_mut(), eventObject, eventListenerInterface) }
}
impl UpKeyEventHandlerBaseDyn for UpKeyEventHandlerBase {
    fn as_UpKeyEventHandlerBase(&self) -> &UpKeyEventHandlerBase { self }
    fn as_UpKeyEventHandlerBase_mut(&mut self) -> &mut UpKeyEventHandlerBase { self }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for UpKeyEventHandlerBase {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
    fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListenerSingleThreaded(self, eventListenerInterface) }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for UpKeyEventHandlerBase {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::fireEvent(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn removeAllListeners(&mut self) { Self::removeAllListeners(self) }
    fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListener(self, eventListenerInterface) }
}


