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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollisionHelper {
    pub sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>,
}


impl CollisionHelper {
    

    pub fn new_struct(mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Self {
        let mut this = Self {
            sourceLayerInterface: None,
        };
        this.sourceLayerInterface = sourceLayerInterface.clone();
        this
    }

    pub fn new(mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sourceLayerInterface))))
    }
    

    pub fn isCollidable(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        return true;
    }
    

    pub fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        return self.sourceLayerInterface.clone();
    }
    

    pub fn setOwnerLayerInterface(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.sourceLayerInterface = sourceLayerInterface.clone();
    }
}
pub trait CollisionHelperDyn {
    fn as_CollisionHelper(&self) -> &CollisionHelper;
    fn as_CollisionHelper_mut(&mut self) -> &mut CollisionHelper;
    fn isCollidable(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { CollisionHelper::isCollidable(self.as_CollisionHelper_mut(), layerInterface) }
    fn getOwnerLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { CollisionHelper::getOwnerLayerInterface(self.as_CollisionHelper_mut()) }
    fn setOwnerLayerInterface(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) { CollisionHelper::setOwnerLayerInterface(self.as_CollisionHelper_mut(), sourceLayerInterface) }
}
impl CollisionHelperDyn for CollisionHelper {
    fn as_CollisionHelper(&self) -> &CollisionHelper { self }
    fn as_CollisionHelper_mut(&mut self) -> &mut CollisionHelper { self }
}


