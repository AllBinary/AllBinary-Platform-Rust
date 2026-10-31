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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::AllBinaryLayer::AllBinaryLayer;
use super::LayerManager::LayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManagerLoggingBase {
}


impl LayerManagerLoggingBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
    }
    

    pub fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
    }
    

    pub fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
    }
    

    pub fn removeResult(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut result: bool) {
    }
    

    pub fn clear(&mut self) {
    }
}
pub trait LayerManagerLoggingBaseDyn {
    fn as_LayerManagerLoggingBase(&self) -> &LayerManagerLoggingBase;
    fn as_LayerManagerLoggingBase_mut(&mut self) -> &mut LayerManagerLoggingBase;
    fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManagerLoggingBase::append(self.as_LayerManagerLoggingBase_mut(), layerInterface) }
    fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { LayerManagerLoggingBase::appendAt(self.as_LayerManagerLoggingBase_mut(), layerInterface, index) }
    fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { LayerManagerLoggingBase::remove(self.as_LayerManagerLoggingBase_mut(), layerInterface) }
    fn removeResult(&mut self, mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut result: bool) { LayerManagerLoggingBase::removeResult(self.as_LayerManagerLoggingBase_mut(), layerManager, layerInterface, result) }
    fn clear(&mut self) { LayerManagerLoggingBase::clear(self.as_LayerManagerLoggingBase_mut()) }
}
impl LayerManagerLoggingBaseDyn for LayerManagerLoggingBase {
    fn as_LayerManagerLoggingBase(&self) -> &LayerManagerLoggingBase { self }
    fn as_LayerManagerLoggingBase_mut(&mut self) -> &mut LayerManagerLoggingBase { self }
}


