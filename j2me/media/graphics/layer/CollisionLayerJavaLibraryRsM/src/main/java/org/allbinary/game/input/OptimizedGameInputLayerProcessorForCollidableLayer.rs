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
// package org.allbinary.game.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayer;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::LayerProcessor::LayerProcessor;
use super::OptimizedGameInputLayerManager::OptimizedGameInputLayerManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OptimizedGameInputLayerProcessorForCollidableLayer {
    pub base: LayerProcessor,
}


impl OptimizedGameInputLayerProcessorForCollidableLayer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerProcessor::new_struct(OptimizedGameInputLayerManager::new()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) {
        let mut gameInputInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryGameLayer::AllBinaryGameLayerDyn>>> = (layerInterface).clone() /* as AllBinaryGameLayer */;
        jcall!(gameInputInterface).processInput(allBinaryLayerManager.clone());
    }
    

    pub fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        if jcall!(layerInterface).implmentsGameInputInterface() {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for OptimizedGameInputLayerProcessorForCollidableLayer {
    type Target = LayerProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OptimizedGameInputLayerProcessorForCollidableLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn for OptimizedGameInputLayerProcessorForCollidableLayer {
    fn as_LayerProcessor(&self) -> &crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &self.base }
    fn as_LayerProcessor_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &mut self.base }
    fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut index: i32) { Self::processAt(self, allBinaryLayerManager, layerInterface, index) }
    fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { Self::isProcessorLayer(self, layerInterface) }
}


