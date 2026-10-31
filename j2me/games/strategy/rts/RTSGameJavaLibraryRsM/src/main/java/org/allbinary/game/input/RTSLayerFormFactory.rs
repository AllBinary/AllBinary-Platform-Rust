// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionForm;
use super::RTSLayerFormFactoryInterface::RTSLayerFormFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerFormFactory {
}

impl RTSLayerFormFactoryInterface for RTSLayerFormFactory {
    fn getInstance(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> { Self::getInstance(self, rtsLayer) }
}

impl RTSLayerFormFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::form::ScrollSelectionForm::ScrollSelectionFormDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait RTSLayerFormFactoryDyn: crate::org::allbinary::game::input::RTSLayerFormFactoryInterface::RTSLayerFormFactoryInterface {
    fn as_RTSLayerFormFactory(&self) -> &RTSLayerFormFactory;
    fn as_RTSLayerFormFactory_mut(&mut self) -> &mut RTSLayerFormFactory;
}
impl RTSLayerFormFactoryDyn for RTSLayerFormFactory {
    fn as_RTSLayerFormFactory(&self) -> &RTSLayerFormFactory { self }
    fn as_RTSLayerFormFactory_mut(&mut self) -> &mut RTSLayerFormFactory { self }
}


