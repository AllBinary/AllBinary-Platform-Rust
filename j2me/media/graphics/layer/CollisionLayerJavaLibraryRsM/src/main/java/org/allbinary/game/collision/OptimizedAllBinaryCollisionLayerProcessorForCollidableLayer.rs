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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::layer::LayerManager::LayerManager;
use crate::org::allbinary::layer::LayerProcessor::LayerProcessor;
use super::OptimizedAllBinaryCollisionLayerManager::OptimizedAllBinaryCollisionLayerManager;
use super::CollidableBaseBehavior::CollidableBaseBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OptimizedAllBinaryCollisionLayerProcessorForCollidableLayer {
    pub base: LayerProcessor,
}


impl OptimizedAllBinaryCollisionLayerProcessorForCollidableLayer {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerProcessor::new_struct(OptimizedAllBinaryCollisionLayerManager::new()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut startIndex: i32) {
        let mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> = (layerInterface).clone() /* as CollidableCompositeLayer */;
        let mut collidableBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> = jcall!(collidableInterfaceCompositeInterface).getCollidableInferface();
        if jcall!(collidableBase).isCollidable(collidableInterfaceCompositeInterface.clone()) {
            let mut layerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::LayerManager::LayerManagerDyn>>> = self.getLayerManager();
            let mut size: i32 = jcall!(layerManager).getSize();
            let mut collidableInterfaceCompositeInterface2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>> = None;
            let mut collidableBase2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn>>> = None;
            for index in (startIndex + 1..size) {
                collidableInterfaceCompositeInterface2 = (jcall!(layerManager).getLayerAt(index.clone())).clone() /* as CollidableCompositeLayer */;
                collidableBase2 = jcall!(collidableInterfaceCompositeInterface2).getCollidableInferface();
                if jcall!(collidableBase2).isCollidable(collidableInterfaceCompositeInterface.clone()) && jcall!(collidableBase).isCollision(collidableInterfaceCompositeInterface.clone(), collidableInterfaceCompositeInterface2.clone()) {
                    jcall!(collidableBase).collide(collidableInterfaceCompositeInterface.clone(), collidableInterfaceCompositeInterface2.clone());
                    jcall!(collidableBase2).collide(collidableInterfaceCompositeInterface2.clone(), collidableInterfaceCompositeInterface.clone());
                }
            }
        }
    }
    

    pub fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        if jcall!(layerInterface).implmentsCollidableInterface() {
            return true;
        } else {
            return false;
        }
    }
}
impl std::ops::Deref for OptimizedAllBinaryCollisionLayerProcessorForCollidableLayer {
    type Target = LayerProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OptimizedAllBinaryCollisionLayerProcessorForCollidableLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::LayerProcessor::LayerProcessorDyn for OptimizedAllBinaryCollisionLayerProcessorForCollidableLayer {
    fn as_LayerProcessor(&self) -> &crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &self.base }
    fn as_LayerProcessor_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerProcessor::LayerProcessor { &mut self.base }
    fn processAt(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut startIndex: i32) { Self::processAt(self, allBinaryLayerManager, layerInterface, startIndex) }
    fn isProcessorLayer(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool { Self::isProcessorLayer(self, layerInterface) }
}


