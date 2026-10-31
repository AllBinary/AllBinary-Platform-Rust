// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::collision::CollisionType::CollisionType;
use crate::org::allbinary::game::collision::CollisionTypeFactory::CollisionTypeFactory;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior;
use super::CollidableCompositeLayer::CollidableCompositeLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableRTSBehavior {
    pub base: CollidableDestroyableDamageableBehavior,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableRTSBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableRTSBehavior {
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableDestroyableDamageableBehavior::new_struct(collidable.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        let mut collisionTypeFactory: Option<Rc<RefCell<CollisionTypeFactory>>> = CollisionTypeFactory::getInstance();
        let mut collisionType: Option<Rc<RefCell<CollisionType>>> = jcall!(jcall!(collidableInterfaceCompositeInterface).getCollidableInferface()).getCollisionTypeWith(ownerLayer.clone());
        if match (&collisionType, &jcall!(collisionTypeFactory).PICKUP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else if match (&collisionType, &jcall!(collisionTypeFactory).COLLISION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return self.base.isCollision(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        } else {
            if jcall!(collidableInterfaceCompositeInterface).getX2() <= jcall!(ownerLayer).getXP() || jcall!(collidableInterfaceCompositeInterface).getY2() <= jcall!(ownerLayer).getYP() || jcall!(collidableInterfaceCompositeInterface).getYP() >= jcall!(ownerLayer).getY2() || jcall!(collidableInterfaceCompositeInterface).getXP() >= jcall!(ownerLayer).getX2() {
                return false;
            } else {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        let mut collisionTypeFactory: Option<Rc<RefCell<CollisionTypeFactory>>> = CollisionTypeFactory::getInstance();
        let mut collisionType: Option<Rc<RefCell<CollisionType>>> = jcall!(jcall!(collidableInterfaceCompositeInterface).getCollidableInferface()).getCollisionTypeWith(ownerLayer.clone());
        if match (&collisionType, &jcall!(collisionTypeFactory).PICKUP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
        } else if match (&collisionType, &jcall!(collisionTypeFactory).COLLISION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.base.collide(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        } else {
            self.collideNone(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        }
    }
    

    // throws Exception
    pub fn collideNone(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
    }
}
impl std::ops::Deref for CollidableRTSBehavior {
    type Target = CollidableDestroyableDamageableBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableRTSBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CollidableRTSBehaviorDyn: crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn {
    fn as_CollidableRTSBehavior(&self) -> &CollidableRTSBehavior;
    fn as_CollidableRTSBehavior_mut(&mut self) -> &mut CollidableRTSBehavior;
    fn collideNone(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { CollidableRTSBehavior::collideNone(self.as_CollidableRTSBehavior_mut(), ownerLayer, collidableInterface) }
}
impl CollidableRTSBehaviorDyn for CollidableRTSBehavior {
    fn as_CollidableRTSBehavior(&self) -> &CollidableRTSBehavior { self }
    fn as_CollidableRTSBehavior_mut(&mut self) -> &mut CollidableRTSBehavior { self }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableRTSBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn isCollisionInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollisionInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn collideInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::collideInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableRTSBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base }
}


