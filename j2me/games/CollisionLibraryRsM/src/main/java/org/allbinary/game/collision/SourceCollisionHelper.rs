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
use super::CollisionHelper::CollisionHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SourceCollisionHelper {
    pub base: CollisionHelper,
}


impl SourceCollisionHelper {
    

    pub fn new_struct(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Self {
        let mut this = Self {
            base: CollisionHelper::new_struct(layerInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layerInterface))))
    }
    

    pub fn isCollidable(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        if !match (&self.sourceLayerInterface, &layerInterface) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for SourceCollisionHelper {
    type Target = CollisionHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SourceCollisionHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollisionHelper::CollisionHelperDyn for SourceCollisionHelper {
    fn as_CollisionHelper(&self) -> &crate::org::allbinary::game::collision::CollisionHelper::CollisionHelper { &self.base }
    fn as_CollisionHelper_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollisionHelper::CollisionHelper { &mut self.base }
    fn isCollidable(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollidable(self, layerInterface) }
}


