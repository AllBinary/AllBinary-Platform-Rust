// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableWaypointBehavior {
    pub base: CollidableRTSBehavior,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableWaypointBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableWaypointBehavior {
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableRTSBehavior::new_struct(collidable.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
    }
    

    pub fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
    }
}
impl std::ops::Deref for CollidableWaypointBehavior {
    type Target = CollidableRTSBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableWaypointBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehaviorDyn for CollidableWaypointBehavior {
    fn as_CollidableRTSBehavior(&self) -> &crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior { &self.base }
    fn as_CollidableRTSBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::CollidableRTSBehavior::CollidableRTSBehavior { &mut self.base }
}
impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableWaypointBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collisionLayer) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, collisionLayer) }
    fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool { Self::isCollisionInterface(self, ownerLayer, collidableInterface) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterface) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableWaypointBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base.base }
}


