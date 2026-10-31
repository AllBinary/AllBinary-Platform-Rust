// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot  
// Generated Code Do Not Modify
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerEvent {
    pub base: AllBinaryEventObject,
    pub rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
}


impl RTSLayerEvent {
    

    pub fn new_struct(mut rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) -> Self {
        let mut this = Self {
            base: AllBinaryEventObject::new_struct(rtsLayer.clone()),
            rtsLayer: None,
        };
        this.rtsLayer = NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(rtsLayer))))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("RTSLayerEvent: \n"));
        jcall!(stringBuffer).append(String::from("RTSLayer: "));
        jcall!(stringBuffer).append(jcall!(self.getRtsLayer()).toString());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getRtsLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        return self.rtsLayer.clone();
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        self.rtsLayer = rtsLayer.clone();
    }
}
impl std::ops::Deref for RTSLayerEvent {
    type Target = AllBinaryEventObject;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerEvent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn for RTSLayerEvent {
    fn as_AllBinaryEventObject(&self) -> &crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &self.base }
    fn as_AllBinaryEventObject_mut(&mut self) -> &mut crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject { &mut self.base }
}


