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
use super::LayerManagerEventListenerInterface::LayerManagerEventListenerInterface;
use super::LayerManagerEvent::LayerManagerEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManagerEventListener {
}

impl LayerManagerEventListenerInterface for LayerManagerEventListener {
    fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onCreateLayerManagerEvent(self, layerManagerEvent) }
    fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { Self::onDeleteLayerManagerEvent(self, layerManagerEvent) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for LayerManagerEventListener {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}

impl LayerManagerEventListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
    }
    

    // throws Exception
    pub fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) {
    }
    

    // throws Exception
    pub fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) {
    }
}
pub trait LayerManagerEventListenerDyn {
    fn as_LayerManagerEventListener(&self) -> &LayerManagerEventListener;
    fn as_LayerManagerEventListener_mut(&mut self) -> &mut LayerManagerEventListener;
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { LayerManagerEventListener::onEvent(self.as_LayerManagerEventListener_mut(), eventObject) }
    fn onCreateLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { LayerManagerEventListener::onCreateLayerManagerEvent(self.as_LayerManagerEventListener_mut(), layerManagerEvent) }
    fn onDeleteLayerManagerEvent(&mut self, mut layerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>) { LayerManagerEventListener::onDeleteLayerManagerEvent(self.as_LayerManagerEventListener_mut(), layerManagerEvent) }
}
impl LayerManagerEventListenerDyn for LayerManagerEventListener {
    fn as_LayerManagerEventListener(&self) -> &LayerManagerEventListener { self }
    fn as_LayerManagerEventListener_mut(&mut self) -> &mut LayerManagerEventListener { self }
}


