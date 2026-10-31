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
// package org.allbinary.logic.util.event.handler
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use crate::jsinterop::annotations::JsProperty;
use super::BasicEventHandlerInterface::BasicEventHandlerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicEventHandler {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub eventStrings: Option<Rc<RefCell<EventStrings>>>,
    pub eventListenerInterfaceList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl BasicEventHandlerInterface for BasicEventHandler {
    fn addListenerInterface(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::addListenerInterface(self, eventListenerInterface) }
    fn addListeners(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::addListeners(self, vector) }
    fn fireEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::fireEvent(self, eventObject) }
    fn removeAllListeners(&mut self) { Self::removeAllListeners(self) }
    fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListener(self, eventListenerInterface) }
}

impl BasicEventHandler {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            eventStrings: None,
            eventListenerInterfaceList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.eventStrings = EventStrings::getInstance();
        this.eventListenerInterfaceList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn removeAllListeners(&mut self) {
        self.eventListenerInterfaceList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    pub fn addListeners(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            eventListenerInterface = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
            self.addListenerInterface(eventListenerInterface.clone());
        }
    }
    

    pub fn removeListeners(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            eventListenerInterface = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
            self.removeListener(eventListenerInterface.clone());
        }
    }
    

    pub fn addListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        if !jcall!(self.eventListenerInterfaceList).contains(crate::java_into_object(eventListenerInterface.clone())) {
            jcall!(self.eventListenerInterfaceList).add(crate::java_into_object(eventListenerInterface.clone()));
        }
    }
    

    pub fn addListenerInterface(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        if !jcall!(self.eventListenerInterfaceList).contains(crate::java_into_object(eventListenerInterface.clone())) {
            jcall!(self.eventListenerInterfaceList).add(crate::java_into_object(eventListenerInterface.clone()));
        }
    }
    

    pub fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(self.eventListenerInterfaceList).remove(crate::java_into_object(eventListenerInterface.clone()));
    }
    

    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(self.eventListenerInterfaceList).remove(crate::java_into_object(eventListenerInterface.clone()));
    }
    

    // throws Exception
    pub fn fireEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        let mut index: i32 = 0;
        while index < jcall!(self.eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = jcall!(self.eventListenerInterfaceList).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
                self.process(eventObject.clone(), eventListenerInterface.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.eventStrings).FIRE_EVENT.clone(), crate::java_into_object(e.clone()));
                }
            }
            index += 1;
        }
    }
    

    // throws Exception
    pub fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(eventListenerInterface).onEvent(eventObject.clone());
    }
    

    pub fn getEventListenerInterfaceListP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.eventListenerInterfaceList.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.eventStrings).TOTAL_LISTENERS.clone());
        jcall!(stringBuffer).appendint(jcall!(self.eventListenerInterfaceList).size());
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        let mut index: i32 = 0;
        while index < jcall!(self.eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = jcall!(self.eventListenerInterfaceList).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
                jcall!(stringBuffer).append(jcall!(self.eventStrings).LISTENER_LABEL.clone());
                jcall!(stringBuffer).append(jcall!(eventListenerInterface).toString());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).TOSTRING.clone(), crate::java_into_object(e.clone()));
                }
            }
            index += 1;
        }
        return jcall!(stringBuffer).toString();
    }
}
pub trait BasicEventHandlerDyn: crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface {
    fn as_BasicEventHandler(&self) -> &BasicEventHandler;
    fn as_BasicEventHandler_mut(&mut self) -> &mut BasicEventHandler;
    fn removeListeners(&mut self, mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { BasicEventHandler::removeListeners(self.as_BasicEventHandler_mut(), vector) }
    fn addListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { BasicEventHandler::addListenerSingleThreaded(self.as_BasicEventHandler_mut(), eventListenerInterface) }
    fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { BasicEventHandler::removeListenerSingleThreaded(self.as_BasicEventHandler_mut(), eventListenerInterface) }
    fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { BasicEventHandler::process(self.as_BasicEventHandler_mut(), eventObject, eventListenerInterface) }
    fn getEventListenerInterfaceListP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicEventHandler::getEventListenerInterfaceListP(self.as_BasicEventHandler_mut()) }
}
impl BasicEventHandlerDyn for BasicEventHandler {
    fn as_BasicEventHandler(&self) -> &BasicEventHandler { self }
    fn as_BasicEventHandler_mut(&mut self) -> &mut BasicEventHandler { self }
}


