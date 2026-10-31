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
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::LayerProcessorInterface::LayerProcessorInterface;
use super::LayerManager::LayerManager;
use super::AllBinaryLayerManager::AllBinaryLayerManager;
use super::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerProcessor {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>,
}

impl LayerProcessorInterface for LayerProcessor {
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> { Self::getLayerManager(self) }
    fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { Self::processAt(self, allBinaryLayerManager, layerInterface, index) }
    fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { Self::isProcessorLayer(self, layerInterface) }
}
impl crate::org::allbinary::layer::BasicLayerProcessorInterface::BasicLayerProcessorInterface for LayerProcessor {
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::process(self, allBinaryLayerManager) }
}

impl LayerProcessor {
    

    pub fn new_struct(mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            layerManager: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.layerManager = layerManager.clone();
        this
    }

    pub fn new(mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerManager))))
    }
    

    // throws Exception
    pub fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    // throws Exception
    pub fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(self.commonStrings).NOT_IMPLEMENTED.clone())));
    }
    

    pub fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> {
        return self.layerManager.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> = self.getLayerManager();
        let mut size: i32 = jcall!(layerManager).getSize();
        for index in (0..size) {
            { let java_arguments = (allBinaryLayerManager.clone(), (jcall!(layerManager).getLayerAt(index.clone())).clone() /* as AllBinaryLayer */, index.clone(),); self.processAt(java_arguments.0, java_arguments.1, java_arguments.2) };
        }
    }
}
pub trait LayerProcessorDyn {
    fn as_LayerProcessor(&self) -> &LayerProcessor;
    fn as_LayerProcessor_mut(&mut self) -> &mut LayerProcessor;
    fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { LayerProcessor::processAt(self.as_LayerProcessor_mut(), allBinaryLayerManager, layerInterface, index) }
    fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { LayerProcessor::isProcessorLayer(self.as_LayerProcessor_mut(), layerInterface) }
    fn getLayerManager(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> { LayerProcessor::getLayerManager(self.as_LayerProcessor_mut()) }
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { LayerProcessor::process(self.as_LayerProcessor_mut(), allBinaryLayerManager) }
}
impl LayerProcessorDyn for LayerProcessor {
    fn as_LayerProcessor(&self) -> &LayerProcessor { self }
    fn as_LayerProcessor_mut(&mut self) -> &mut LayerProcessor { self }
}


