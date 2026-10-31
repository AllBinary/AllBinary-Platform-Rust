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
// package org.allbinary.layer.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::LayerManagerEventListener::LayerManagerEventListener;
use super::LayerManagerEvent::LayerManagerEvent;
use super::LayerManagerEventListenerInterface::LayerManagerEventListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManagerEventHandler {
    pub base: BasicEventHandler,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub CREATE: String,
    pub DELETE: String,
}

pub static mut LayerManagerEventHandler_instance: std::cell::LazyCell<Option<Rc<RefCell<LayerManagerEventHandler>>>> = std::cell::LazyCell::new(|| LayerManagerEventHandler::new());

impl LayerManagerEventHandler {
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerManagerEventHandler>>> {
        return unsafe { LayerManagerEventHandler_instance.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicEventHandler::new_struct(),
            list: None,
            CREATE: String::new(),
            DELETE: String::new(),
        };
        this.list = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.CREATE = String::from("Create");
        this.DELETE = String::from("Delete");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn addListener(&mut self, mut layerManagerEventListener: Option<Rc<RefCell<dyn crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn>>>) {
        if !jcall!(self.list).contains(crate::java_into_object(layerManagerEventListener.clone())) {
            jcall!(self.list).add(crate::java_into_object(layerManagerEventListener.clone()));
        }
    }
    

    pub fn removeAllListeners(&mut self) {
        jcall!(self.list).clear();
        self.base.removeAllListeners();
    }
    

    pub fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        jcall!(self.list).remove(crate::java_into_object(eventListenerInterface.clone()));
        self.base.removeListener(eventListenerInterface.clone());
    }
    

    // throws Exception
    pub fn fireEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        {
            let mut index: i32 = jcall!(self.list).size();
            while { index -= 1; index } >= 0 {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut layerManagerEventListener: Option<Rc<RefCell<dyn crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(layerManagerEventListener).onCreateLayerManagerEvent((eventObject).clone() /* as LayerManagerEvent */);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(EventStrings::getInstance()).FIRE_EVENT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
            }
            }
        }
        self.base.fireEvent(eventObject.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn fireDeleteEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        {
            let mut index: i32 = jcall!(self.list).size();
            while { index -= 1; index } >= 0 {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut layerManagerEventListener: Option<Rc<RefCell<dyn crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn>>> = { let java_object = (jcall!(self.list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::event::LayerManagerEventListener::LayerManagerEventListenerDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(layerManagerEventListener).onDeleteLayerManagerEvent((eventObject).clone() /* as LayerManagerEvent */);
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(EventStrings::getInstance()).FIRE_EVENT.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
                }
            }
            }
        }
        let mut eventListenerInterfaceList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.eventListenerInterfaceList.clone();
        let mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>> = None;
        let mut layerManagerEventListenerInterface: Option<Rc<RefCell<dyn LayerManagerEventListenerInterface>>> = None;
        let mut index: i32 = 0;
        while index < jcall!(eventListenerInterfaceList).size() {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                eventListenerInterface = { let java_object = (jcall!(eventListenerInterfaceList).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EventListenerInterface>>>>().expect("ClassCastException").clone()) };
                layerManagerEventListenerInterface = (panic!("ClassCastException: a Rust dyn EventListenerInterface cannot be cast to dyn LayerManagerEventListenerInterface"));
                jcall!(layerManagerEventListenerInterface).onDeleteLayerManagerEvent((eventObject).clone() /* as LayerManagerEvent */);
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
    pub fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) {
        let mut layerManagerEventListenerInterface: Option<Rc<RefCell<dyn LayerManagerEventListenerInterface>>> = (panic!("ClassCastException: a Rust dyn EventListenerInterface cannot be cast to dyn LayerManagerEventListenerInterface"));
        jcall!(layerManagerEventListenerInterface).onCreateLayerManagerEvent((eventObject).clone() /* as LayerManagerEvent */);
    }
}
impl std::ops::Deref for LayerManagerEventHandler {
    type Target = BasicEventHandler;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LayerManagerEventHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn for LayerManagerEventHandler {
    fn as_BasicEventHandler(&self) -> &crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &self.base }
    fn as_BasicEventHandler_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler { &mut self.base }
    fn process(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::process(self, eventObject, eventListenerInterface) }
}
impl crate::org::allbinary::logic::util::event::handler::BasicEventHandlerInterface::BasicEventHandlerInterface for LayerManagerEventHandler {
    fn addListenerInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListenerInterface(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn addListeners(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandler::addListeners(crate::org::allbinary::logic::util::event::handler::BasicEventHandler::BasicEventHandlerDyn::as_BasicEventHandler_mut(self), p1) }
    fn fireEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::fireEvent(self, eventObject) }
    fn removeAllListeners(&mut self) { Self::removeAllListeners(self) }
    fn removeListener(&mut self, mut eventListenerInterface: Option<Rc<RefCell<dyn EventListenerInterface>>>) { Self::removeListener(self, eventListenerInterface) }
}


