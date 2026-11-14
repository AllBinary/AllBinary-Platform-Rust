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
use crate::java::lang::Exception::*;
use super::LayerProcessor::LayerProcessor;
use super::SingleLayerLayerManager::SingleLayerLayerManager;
use super::LayerManager::LayerManager;
use super::AllBinaryLayer::AllBinaryLayer;
use super::AllBinaryLayerManager::AllBinaryLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SingleLayerLayerProcessor {
    pub base: LayerProcessor,
}

impl crate::org::allbinary::layer::BasicLayerProcessorInterface::BasicLayerProcessorInterface for SingleLayerLayerProcessor {
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::process(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::layer::LayerProcessorInterface::LayerProcessorInterface for SingleLayerLayerProcessor {
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> { crate::org::allbinary::layer::LayerProcessor::LayerProcessor::getLayerManager(crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn::as_LayerProcessor_mut(self)) }
    fn processAt(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, p3: i32) { crate::org::allbinary::layer::LayerProcessor::LayerProcessor::processAt(crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn::as_LayerProcessor_mut(self), p1, p2, p3) }
    fn isProcessorLayer(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { crate::org::allbinary::layer::LayerProcessor::LayerProcessor::isProcessorLayer(crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn::as_LayerProcessor_mut(self), p1) }
}

impl SingleLayerLayerProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerProcessor::new_struct(SingleLayerLayerManager::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> = self.getLayerManager();
        { let java_arguments = (allBinaryLayerManager.clone(), (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { panic!("ClassCastException: a Rust dyn Layer cannot be cast to AllBinaryLayer") })(), 0,); self.processAt(java_arguments.0, java_arguments.1, java_arguments.2) };
    }
}
impl std::ops::Deref for SingleLayerLayerProcessor {
    type Target = LayerProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SingleLayerLayerProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn for SingleLayerLayerProcessor {
    fn as_LayerProcessor(&self) -> &crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &self.base }
    fn as_LayerProcessor_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &mut self.base }
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::process(self, allBinaryLayerManager) }
}


