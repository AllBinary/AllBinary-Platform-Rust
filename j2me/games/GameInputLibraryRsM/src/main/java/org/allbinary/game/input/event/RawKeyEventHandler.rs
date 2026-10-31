// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::RawKeyEventListener::RawKeyEventListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RawKeyEventHandler {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub eventListenerInterfaceList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static RawKeyEventHandler_instance: Option<Rc<RefCell<RawKeyEventHandler>>> = RawKeyEventHandler::new(); }
thread_local! { pub static RawKeyEventHandler_TOTAL_LISTENERS: String = String::from("Total Listeners: "); }
thread_local! { pub static RawKeyEventHandler_LISTENER_LABEL: String = String::from(" Listener: "); }

impl RawKeyEventHandler {
    

    pub fn getInstance() -> Option<Rc<RefCell<RawKeyEventHandler>>> {
        return RawKeyEventHandler_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            eventListenerInterfaceList: None,
        };
        this.logUtil = LogUtil::getInstance();
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
        let mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>> = None;
        let mut size: i32 = jcall!(vector).size();
        for index in (0..size) {
            eventListenerInterface = { let java_object = jcall!(vector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn RawKeyEventListener>>>>().expect("ClassCastException").clone()) };
            self.addListener(eventListenerInterface.clone());
        }
    }
    

    pub fn addListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>>) {
        if !jcall!(self.eventListenerInterfaceList).contains(crate::java_into_object(eventListenerInterface.clone())) {
            jcall!(self.eventListenerInterfaceList).add(crate::java_into_object(eventListenerInterface.clone()));
        }
    }
    

    pub fn addListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>>) {
        if !jcall!(self.eventListenerInterfaceList).contains(crate::java_into_object(eventListenerInterface.clone())) {
            jcall!(self.eventListenerInterfaceList).add(crate::java_into_object(eventListenerInterface.clone()));
        }
    }
    

    pub fn removeListenerSingleThreaded(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>>) {
        jcall!(self.eventListenerInterfaceList).remove(crate::java_into_object(eventListenerInterface.clone()));
    }
    

    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>>) {
        jcall!(self.eventListenerInterfaceList).remove(crate::java_into_object(eventListenerInterface.clone()));
    }
    

    // throws Exception
    pub fn fireEvent(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool) {
        let mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>> = None;
        let mut index: i32 = 0;
        while index < jcall!(self.eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = jcall!(self.eventListenerInterfaceList).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn RawKeyEventListener>>>>().expect("ClassCastException").clone()) };
                self.process(keyCode.clone(), deviceId.clone(), repeated.clone(), eventListenerInterface.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(EventStrings::getInstance()).FIRE_EVENT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
            }
            index += 1;
        }
    }
    

    // throws Exception
    pub fn process(&mut self, mut keyCode: i32, mut deviceId: i32, mut repeated: bool, mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>>) {
        jcall!(eventListenerInterface).onEventRaw(keyCode.clone(), deviceId.clone(), repeated.clone());
    }
    

    pub fn getEventListenerInterfaceListP(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.eventListenerInterfaceList.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(RawKeyEventHandler_TOTAL_LISTENERS.with(Clone::clone).clone());
        jcall!(stringBuffer).appendint(jcall!(self.eventListenerInterfaceList).size());
        let mut eventListenerInterface: Option<Rc<RefCell<dyn RawKeyEventListener>>> = None;
        let mut index: i32 = 0;
        while index < jcall!(self.eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = jcall!(self.eventListenerInterfaceList).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn RawKeyEventListener>>>>().expect("ClassCastException").clone()) };
                jcall!(stringBuffer).append(RawKeyEventHandler_LISTENER_LABEL.with(Clone::clone).clone());
                jcall!(stringBuffer).append(jcall!(eventListenerInterface).toString());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).TOSTRING.clone(), crate::java_into_object(e.clone()));
                }
            }
            index += 1;
        }
        return jcall!(stringBuffer).toString();
    }
}

