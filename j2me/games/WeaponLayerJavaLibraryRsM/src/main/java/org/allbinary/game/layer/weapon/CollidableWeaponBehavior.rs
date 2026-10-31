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
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::collision::CollidableHelperFactory::CollidableHelperFactory;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::collision::CollisionHelper::CollisionHelper;
use crate::org::allbinary::game::collision::CollisionType::CollisionType;
use crate::org::allbinary::game::collision::CollisionTypeFactory::CollisionTypeFactory;
use crate::org::allbinary::game::combat::damage::DamageableInterface::DamageableInterface;
use crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableWeaponBehavior {
    pub base: CollidableDestroyableDamageableBehavior,
    pub collided: bool,
    pub collisionHelper: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollisionHelper::CollisionHelperDyn>>>,
}

pub static mut CollidableWeaponBehavior_NULL_COLLIDABLE_WEAPON_BEHAVIOR: std::cell::LazyCell<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = std::cell::LazyCell::new(|| jcall!(NullUtil::getInstance()).NULL_OBJECT.clone());
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableWeaponBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableWeaponBehavior {
    

    pub fn getNullInstance() -> Option<Rc<RefCell<CollidableWeaponBehavior>>> {
        if match (&unsafe { CollidableWeaponBehavior_NULL_COLLIDABLE_WEAPON_BEHAVIOR.clone() }, &jcall!(NullUtil::getInstance()).NULL_OBJECT) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            unsafe { *CollidableWeaponBehavior_NULL_COLLIDABLE_WEAPON_BEHAVIOR = CollidableWeaponBehavior::new(false) };
        }
        return (unsafe { CollidableWeaponBehavior_NULL_COLLIDABLE_WEAPON_BEHAVIOR.clone() }).clone() /* as CollidableWeaponBehavior */;
    }
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableDestroyableDamageableBehavior::new_struct(collidable.clone()),
            collided: false,
            collisionHelper: None,
        };
        // super(...) is in the struct literal above
        this.setCollided(false);
        this.setCollidable(true);
        this.collisionHelper = CollidableHelperFactory::getInstance();
        this
    }

    pub fn new(mut collidable: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(collidable))))
    }
    

    pub fn init(&mut self, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) {
        self.setCollided(false);
        jcall!(self.collisionHelper).setOwnerLayerInterface(sourceLayerInterface.clone());
    }
    

    pub fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool {
        if jcall!(self.collisionHelper).isCollidable(collisionLayer.clone()) {
            if !match (&jcall!(ownerLayer).getGroupInterface()[(0) as usize], &jcall!(collisionLayer).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return self.base.isCollision(ownerLayer.clone(), collisionLayer.clone());
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) {
        self.base.collide(ownerLayer.clone(), collisionLayer.clone());
        self.collided = true;
    }
    

    pub fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool {
        if jcall!(self.collisionHelper).isCollidable((collidableInterfaceCompositeInterface).clone() /* as CollidableCompositeLayer */) {
            let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = (collidableInterfaceCompositeInterface).clone() /* as AllBinaryLayer */;
            if !match (&jcall!(ownerLayer).getGroupInterface()[(0) as usize], &jcall!(layerInterface).getGroupInterface()[(0) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(self.layerCollisionUtil).isCollision(ownerLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>), layerInterface.clone()) {
                    return true;
                }
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        let mut ownerDamageableInterface: Option<Rc<RefCell<dyn DamageableInterface>>> = (ownerLayer).clone() /* as DamageableInterface */;
        let mut damageableInterface: Option<Rc<RefCell<dyn DamageableInterface>>> = panic!("ClassCastException: a Rust dyn CollidableInterfaceCompositeInterface cannot be cast to dyn DamageableInterface");
        jcall!(self.damageUtil).process(ownerDamageableInterface.clone(), damageableInterface.clone());
        self.collided = true;
    }
    

    pub fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> {
        return jcall!(CollisionTypeFactory::getInstance()).COLLISION.clone();
    }
    

    pub fn isCollided(&mut self) -> bool {
        return self.collided.clone();
    }
    

    pub fn setCollided(&mut self, mut collided: bool) {
        self.collided = collided.clone();
    }
}
impl std::ops::Deref for CollidableWeaponBehavior {
    type Target = CollidableDestroyableDamageableBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableWeaponBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableWeaponBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collisionLayer) }
    fn isCollision(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collisionLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { Self::isCollision(self, ownerLayer, collisionLayer) }
    fn isCollisionInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) -> bool { Self::isCollisionInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn getCollisionTypeWith(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> Option<Rc<RefCell<CollisionType>>> { Self::getCollisionTypeWith(self, layerInterface) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableWeaponBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base }
}


