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
// package org.allbinary.game.layer.drop
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::DropLayerInterface::DropLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DropLayerProcessor {
    pub base: BasicLayerProcessor,
}

thread_local! { pub static DropLayerProcessor_LAYER_PROCESSOR: Option<Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>> = DropLayerProcessor::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>); }

impl DropLayerProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicLayerProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>> {
        return DropLayerProcessor_LAYER_PROCESSOR.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut size: i32 = jcall!(list).size();
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        let mut dropLayerInterface: Option<Rc<RefCell<dyn DropLayerInterface>>> = None;
        for index in (0..size) {
            layerInterface = { let java_object = (jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize]).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>>().expect("ClassCastException").clone()) };
            dropLayerInterface = (layerInterface).clone() /* as DropLayerInterface */;
            layerInterface = (jcall!(dropLayerInterface).getDroppedLayer()).clone() /* as AllBinaryLayer */;
            jcall!(allBinaryLayerManager).append(layerInterface.clone());
        }
        jcall!(list).clear();
    }
}
impl std::ops::Deref for DropLayerProcessor {
    type Target = BasicLayerProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DropLayerProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn for DropLayerProcessor {
    fn as_BasicLayerProcessor(&self) -> &crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor { &self.base }
    fn as_BasicLayerProcessor_mut(&mut self) -> &mut crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor { &mut self.base }
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::process(self, allBinaryLayerManager) }
}


