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
// package org.allbinary.game.layer.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::collision::CollisionType::CollisionType;
use crate::org::allbinary::game::collision::CollisionTypeFactory::CollisionTypeFactory;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterface::PickedUpLayerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use super::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior;
use super::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableCharacterBehavior {
    pub base: CollidableDestroyableDamageableBehavior,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableCharacterBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableCharacterBehavior {
    

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
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        let mut collisionTypeFactory: Option<Rc<RefCell<CollisionTypeFactory>>> = CollisionTypeFactory::getInstance();
        let mut collisionType: Option<Rc<RefCell<CollisionType>>> = jcall!(jcall!(collidableInterfaceCompositeInterface).getCollidableInferface()).getCollisionTypeWith(ownerLayer.clone());
        if match (&collisionType, &jcall!(collisionTypeFactory).PICKUP) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (ownerLayer).clone() /* as CollidableDestroyableDamageableLayer */;
            jcall!(jcall!(collidableDestroyableDamageableLayer).getPickupBehavior()).doPickupLayer((collidableInterfaceCompositeInterface).clone() /* as PickedUpLayerInterface */);
        } else {
            self.base.collide(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        }
    }
    

    // throws Exception
    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        ForcedLogUtil::log(jcall!(self.commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
}
impl std::ops::Deref for CollidableCharacterBehavior {
    type Target = CollidableDestroyableDamageableBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableCharacterBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableCharacterBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base }
}
impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableCharacterBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn isCollision(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollision(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn isCollisionInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollisionInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
}


