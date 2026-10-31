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
use crate::org::allbinary::layer::event::LayerManagerEvent::LayerManagerEvent;
use crate::org::allbinary::layer::event::LayerManagerEventHandler::LayerManagerEventHandler;
use super::LayerManager::LayerManager;
use super::LayerProcessor::LayerProcessor;
use super::AllBinaryLayer::AllBinaryLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryLayerManager {
    pub base: LayerManager,
    pub layerManagerEventHandler: Option<Rc<RefCell<LayerManagerEventHandler>>>,
    pub createLayerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>,
    pub deleteLayerManagerEvent: Option<Rc<RefCell<LayerManagerEvent>>>,
    pub basicLayerProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>>,
}


impl AllBinaryLayerManager {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerManager::new_struct(LayerManagerNoDebug::LayerManagerNoDebug::getInstance()),
            layerManagerEventHandler: None,
            createLayerManagerEvent: None,
            deleteLayerManagerEvent: None,
            basicLayerProcessorArray: Vec::new(),
        };
        this.layerManagerEventHandler = LayerManagerEventHandler::getInstance();
        this.createLayerManagerEvent = LayerManagerEvent::new(this, jcall!(this.layerManagerEventHandler).CREATE.clone());
        this.deleteLayerManagerEvent = LayerManagerEvent::new(this, jcall!(this.layerManagerEventHandler).DELETE.clone());
        this.basicLayerProcessorArray = vec![None; (0) as usize];
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLayerProcessorArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>> {
        return self.basicLayerProcessorArray.clone();
    }
    

    pub fn setLayerProcessorArray(&mut self, mut layerProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>>) {
        self.basicLayerProcessorArray = layerProcessorArray.clone();
    }
    

    pub fn log(&mut self) {
        let mut size: i32 = (self.basicLayerProcessorArray.len() as i32).clone();
        for index in (0..size) {
            let mut layerProcessorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>> = self.basicLayerProcessorArray[(index) as usize].clone();
        }
    }
    

    // throws Exception
    pub fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        if !self.contains(layerInterface.clone()) {
            self.appendProcessors(layerInterface.clone());
            self.base.append(layerInterface.clone());
        }
    }
    

    // throws Exception
    pub fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
        if !self.contains(layerInterface.clone()) {
            self.appendProcessors(layerInterface.clone());
            self.base.appendAt(layerInterface.clone(), index.clone());
        }
    }
    

    // throws Exception
    pub fn appendProcessors(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.createLayerManagerEvent).setLayerInterface(layerInterface.clone());
        jcall!(self.layerManagerEventHandler).fireEvent(self.createLayerManagerEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        let mut layerProcessorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>> = None;
        {
            let mut index: i32 = (self.basicLayerProcessorArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            layerProcessorInterface = self.basicLayerProcessorArray[(index) as usize].clone();
            if jcall!(layerProcessorInterface).isProcessorLayer(layerInterface.clone()) {
                jcall!(jcall!(layerProcessorInterface).getLayerManager()).append(layerInterface.clone());
            }
            }
        }
    }
    

    // throws Exception
    // synchronized
    pub fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        jcall!(self.deleteLayerManagerEvent).setLayerInterface(layerInterface.clone());
        jcall!(self.layerManagerEventHandler).fireDeleteEvent(self.deleteLayerManagerEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
        let mut layerProcessorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>> = None;
        {
            let mut index: i32 = (self.basicLayerProcessorArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            layerProcessorInterface = self.basicLayerProcessorArray[(index) as usize].clone();
            jcall!(jcall!(layerProcessorInterface).getLayerManager()).remove(layerInterface.clone());
            }
        }
        self.base.remove(layerInterface.clone());
    }
    

    // throws Exception
    pub fn process(&mut self) {
        let mut layerProcessorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>> = None;
        let mut size: i32 = (self.basicLayerProcessorArray.len() as i32).clone();
        for index in (0..size) {
            layerProcessorInterface = self.basicLayerProcessorArray[(index) as usize].clone();
            jcall!(layerProcessorInterface).process(None /* Java this: a &mut self method has no Rc of itself */);
        }
    }
    

    // throws Exception
    pub fn cleanup(&mut self) {
        let mut layerProcessorInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>> = None;
        let mut size: i32 = (self.basicLayerProcessorArray.len() as i32).clone();
        for index in (0..size) {
            layerProcessorInterface = self.basicLayerProcessorArray[(index) as usize].clone();
            jcall!(jcall!(layerProcessorInterface).getLayerManager()).cleanup();
        }
        self.base.cleanup();
    }
}
impl std::ops::Deref for AllBinaryLayerManager {
    type Target = LayerManager;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryLayerManager {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryLayerManagerDyn: crate::org::allbinary::layer::LayerManager::LayerManagerDyn {
    fn as_AllBinaryLayerManager(&self) -> &AllBinaryLayerManager;
    fn as_AllBinaryLayerManager_mut(&mut self) -> &mut AllBinaryLayerManager;
    fn getLayerProcessorArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>> { AllBinaryLayerManager::getLayerProcessorArray(self.as_AllBinaryLayerManager_mut()) }
    fn setLayerProcessorArray(&mut self, mut layerProcessorArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn>>>>) { AllBinaryLayerManager::setLayerProcessorArray(self.as_AllBinaryLayerManager_mut(), layerProcessorArray) }
    fn log(&mut self) { AllBinaryLayerManager::log(self.as_AllBinaryLayerManager_mut()) }
    fn appendProcessors(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { AllBinaryLayerManager::appendProcessors(self.as_AllBinaryLayerManager_mut(), layerInterface) }
    fn process(&mut self) { AllBinaryLayerManager::process(self.as_AllBinaryLayerManager_mut()) }
}
impl AllBinaryLayerManagerDyn for AllBinaryLayerManager {
    fn as_AllBinaryLayerManager(&self) -> &AllBinaryLayerManager { self }
    fn as_AllBinaryLayerManager_mut(&mut self) -> &mut AllBinaryLayerManager { self }
}

impl crate::org::allbinary::layer::LayerManager::LayerManagerDyn for AllBinaryLayerManager {
    fn as_LayerManager(&self) -> &crate::org::allbinary::layer::LayerManager::LayerManager { &self.base }
    fn as_LayerManager_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerManager::LayerManager { &mut self.base }
    fn append(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::append(self, layerInterface) }
    fn appendAt(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { Self::appendAt(self, layerInterface, index) }
    fn remove(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { Self::remove(self, layerInterface) }
    fn cleanup(&mut self) { Self::cleanup(self) }
}


