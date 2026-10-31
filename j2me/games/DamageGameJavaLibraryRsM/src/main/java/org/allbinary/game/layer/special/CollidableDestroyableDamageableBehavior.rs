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
use crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::combat::damage::DamageUtil::DamageUtil;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use super::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableDestroyableDamageableBehavior {
    pub base: CollidableBaseBehavior,
    pub damageUtil: Option<Rc<RefCell<DamageUtil>>>,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableDestroyableDamageableBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableDestroyableDamageableBehavior {
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableBaseBehavior::new_struct(collidable.clone()),
            damageUtil: None,
        };
        this.damageUtil = DamageUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        if !match (&jcall!(ownerLayer).getGroupInterface()[(0) as usize], &jcall!(collisionLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return self.base.isCollision(ownerLayer.clone(), collisionLayer.clone());
        }
        return false;
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        let mut ownerCollidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (ownerLayer).clone() /* as CollidableDestroyableDamageableLayer */;
        let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = (collidableInterfaceCompositeInterface).clone() /* as CollidableDestroyableDamageableLayer */;
        jcall!(ownerCollidableDestroyableDamageableLayer).damage(jcall!(collidableDestroyableDamageableLayer).getDamage(0), 0);
    }
    

    pub fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool {
        ForcedLogUtil::log(String::from("No Longer Used"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        return false;
    }
    

    // throws Exception
    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        ForcedLogUtil::log(String::from("No Longer Used"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
}
impl std::ops::Deref for CollidableDestroyableDamageableBehavior {
    type Target = CollidableBaseBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableDestroyableDamageableBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CollidableDestroyableDamageableBehaviorDyn: crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &CollidableDestroyableDamageableBehavior;
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut CollidableDestroyableDamageableBehavior;
}
impl CollidableDestroyableDamageableBehaviorDyn for CollidableDestroyableDamageableBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &CollidableDestroyableDamageableBehavior { self }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut CollidableDestroyableDamageableBehavior { self }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableDestroyableDamageableBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, collisionLayer) }
    fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool { Self::isCollisionInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
}


