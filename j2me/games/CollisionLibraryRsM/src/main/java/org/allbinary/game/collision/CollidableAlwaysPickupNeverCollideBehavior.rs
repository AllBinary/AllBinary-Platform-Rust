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
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior;
use super::CollisionType::CollisionType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableAlwaysPickupNeverCollideBehavior {
    pub base: CollidableNeverCollideBehavior,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableAlwaysPickupNeverCollideBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableAlwaysPickupNeverCollideBehavior {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: CollidableNeverCollideBehavior::new_struct(true),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> {
        return jcall!(CollisionTypeFactory::CollisionTypeFactory::getInstance()).PICKUP.clone();
    }
}
impl std::ops::Deref for CollidableAlwaysPickupNeverCollideBehavior {
    type Target = CollidableNeverCollideBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableAlwaysPickupNeverCollideBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableAlwaysPickupNeverCollideBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base }
    fn collide(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior::collide(crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn::as_CollidableNeverCollideBehavior_mut(self), p1, p2) }
    fn isCollision(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior::isCollision(crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn::as_CollidableNeverCollideBehavior_mut(self), p1, p2) }
    fn isCollisionInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) -> bool { crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior::isCollisionInterface(crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn::as_CollidableNeverCollideBehavior_mut(self), p1, p2) }
    fn collideInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) { crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior::collideInterface(crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn::as_CollidableNeverCollideBehavior_mut(self), p1, p2) }
    fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> { Self::getCollisionTypeWith(self, layerInterface) }
}
impl crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehaviorDyn for CollidableAlwaysPickupNeverCollideBehavior {
    fn as_CollidableNeverCollideBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior { &self.base }
    fn as_CollidableNeverCollideBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableNeverCollideBehavior::CollidableNeverCollideBehavior { &mut self.base }
}


