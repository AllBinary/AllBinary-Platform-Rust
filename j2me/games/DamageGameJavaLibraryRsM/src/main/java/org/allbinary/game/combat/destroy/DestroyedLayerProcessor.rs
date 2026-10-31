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
// package org.allbinary.game.combat.destroy
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::combat::destroy::event::DestroyEventCircularStaticPool::DestroyEventCircularStaticPool;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEvent::DestroyedEvent;
use crate::org::allbinary::game::combat::destroy::event::DestroyedEventHandler::DestroyedEventHandler;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DestroyedLayerProcessor {
    pub base: BasicLayerProcessor,
}

pub static mut DestroyedLayerProcessor_DESTROYED_LAYER_PROCESSOR: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>>> = std::cell::LazyCell::new(|| crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor_NULL_LAYER_PROCESSOR.with(Clone::clone).clone());

impl DestroyedLayerProcessor {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicLayerProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init() {
        unsafe { *DestroyedLayerProcessor_DESTROYED_LAYER_PROCESSOR = DestroyedLayerProcessor::new() };
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn>>> {
        return unsafe { DestroyedLayerProcessor_DESTROYED_LAYER_PROCESSOR.clone() }.clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.getList();
        let mut size: i32 = jcall!(list).size();
        let mut destroyedEventHandler: Option<Rc<RefCell<DestroyedEventHandler>>> = DestroyedEventHandler::getInstance();
        let mut destroyEventCircularStaticPool: Option<Rc<RefCell<DestroyEventCircularStaticPool>>> = DestroyEventCircularStaticPool::getInstance();
        let mut destroyedEvent: Option<Rc<RefCell<DestroyedEvent>>> = None;
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = None;
        for index in (0..size) {
            let mut layerInterfaceCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(list).as_BasicArrayList_mut().objectArray[(index) as usize].clone();
            if layerInterfaceCanBeNull.is_some() {
                layerInterface = (layerInterfaceCanBeNull).clone() /* as AllBinaryLayer */;
                jcall!(allBinaryLayerManager).remove(layerInterface.clone());
                destroyedEvent = jcall!(destroyEventCircularStaticPool).getInstanceForLayer(layerInterface.clone());
                jcall!(destroyedEventHandler).fireEvent(destroyedEvent.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>));
            } else {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            }
        }
        jcall!(list).clear();
    }
}
impl std::ops::Deref for DestroyedLayerProcessor {
    type Target = BasicLayerProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DestroyedLayerProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessorDyn for DestroyedLayerProcessor {
    fn as_BasicLayerProcessor(&self) -> &crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor { &self.base }
    fn as_BasicLayerProcessor_mut(&mut self) -> &mut crate::org::allbinary::layer::BasicLayerProcessor::BasicLayerProcessor { &mut self.base }
    fn process(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::process(self, allBinaryLayerManager) }
}


