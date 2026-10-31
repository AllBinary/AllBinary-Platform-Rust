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
// package org.allbinary.game.tracking
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrackingEvent {
    pub base: AllBinaryEventObject,
    pub layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl TrackingEvent {
    

    pub fn new_struct(mut allBinaryLayerInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(allBinaryLayerInterface.clone()),
            layerInterface: None,
        };
        this.layerInterface = crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        if !match (&allBinaryLayerInterface, &TrackingEventHandler::TrackingEventHandler::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            this.setLayerInterface((allBinaryLayerInterface).clone() /* as AllBinaryLayer */);
        }
        this
    }

    pub fn new(mut allBinaryLayerInterface: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(allBinaryLayerInterface))))
    }
    

    pub fn init(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        self.setSource(anyType.clone());
    }
    

    pub fn getLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.layerInterface.clone();
    }
    

    pub fn setLayerInterface(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.layerInterface = layerInterface.clone();
    }
    

    pub fn setLayerInterfaceForCircularStaticPool(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.layerInterface = layerInterface.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("TrackingEvent: \n"));
        jcall!(stringBuffer).append(String::from("LayerInterface: "));
        jcall!(stringBuffer).append(jcall!(self.layerInterface).toString());
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for TrackingEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TrackingEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for TrackingEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


