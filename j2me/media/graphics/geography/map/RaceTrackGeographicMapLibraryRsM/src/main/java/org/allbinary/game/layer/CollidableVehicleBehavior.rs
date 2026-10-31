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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::pickup::PickedUpLayerInterface::PickedUpLayerInterface;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface;
use crate::org::allbinary::game::collision::CollisionType::CollisionType;
use crate::org::allbinary::game::collision::CollisionTypeFactory::CollisionTypeFactory;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use super::VehiclePropertiesCompositeInterface::VehiclePropertiesCompositeInterface;
use super::CollidableCompositeLayer::CollidableCompositeLayer;
use super::VehicleProperties::VehicleProperties;
use super::VehicleFrictionProperties::VehicleFrictionProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollidableVehicleBehavior {
    pub base: CollidableDestroyableDamageableBehavior,
    pub totalImpactVelocity: i64,
    pub halfImpactVelocityX: i64,
    pub halfImpactVelocityY: i64,
}

impl crate::org::allbinary::layer::NamedInterface::NamedInterface for CollidableVehicleBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl CollidableVehicleBehavior {
    

    pub fn new_struct(mut collidable: bool) -> Self {
        let mut this = Self {
            base: CollidableDestroyableDamageableBehavior::new_struct(collidable.clone()),
            totalImpactVelocity: 0_i64,
            halfImpactVelocityX: 0_i64,
            halfImpactVelocityY: 0_i64,
        };
        this.totalImpactVelocity = 0_i64;
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
            let mut collidableDestroyableDamageableLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> = ((ownerLayer).clone() /* as CollidableDestroyableDamageableLayer */);
            jcall!(jcall!(collidableDestroyableDamageableLayer).getPickupBehavior()).doPickupLayer((collidableInterfaceCompositeInterface).clone() /* as PickedUpLayerInterface */);
        } else if match (&collisionType, &jcall!(collisionTypeFactory).COLLISION) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.base.collide(ownerLayer.clone(), collidableInterfaceCompositeInterface.clone());
        } else {
            self.collideVehicle(ownerLayer.clone(), (collidableInterfaceCompositeInterface).clone() /* as VehiclePropertiesCompositeInterface */);
        }
    }
    

    pub fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) {
        ForcedLogUtil::log(String::from("Don't Use Interface Version It Is Slower"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn collideVehicle(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut vehiclePropertiesCompositeInterface: Option<Rc<RefCell<dyn VehiclePropertiesCompositeInterface>>>) {
        let mut ownerVehicleLayerInterface: Option<Rc<RefCell<dyn VehiclePropertiesCompositeInterface>>> = ((ownerLayer).clone() /* as VehiclePropertiesCompositeInterface */);
        let mut ownerVehicleProperties: Option<Rc<RefCell<VehicleProperties>>> = jcall!(ownerVehicleLayerInterface).getVehicleProperties();
        let mut vehicleProperties: Option<Rc<RefCell<VehicleProperties>>> = jcall!(vehiclePropertiesCompositeInterface).getVehicleProperties();
        let mut impactVelocityX: i64 = jcall!(jcall!(jcall!(ownerVehicleProperties).getVelocityProperties()).getVelocityXBasicDecimalP()).getUnscaled() - jcall!(jcall!(jcall!(vehicleProperties).getVelocityProperties()).getVelocityXBasicDecimalP()).getUnscaled();
        let mut impactVelocityY: i64 = jcall!(jcall!(jcall!(ownerVehicleProperties).getVelocityProperties()).getVelocityYBasicDecimalP()).getUnscaled() - jcall!(jcall!(jcall!(vehicleProperties).getVelocityProperties()).getVelocityYBasicDecimalP()).getUnscaled();
        if impactVelocityX != 0_i64 {
            self.halfImpactVelocityX = (impactVelocityX >> 1);
            jcall!(jcall!(jcall!(ownerVehicleProperties).getVelocityProperties()).getVelocityXBasicDecimalP()).addlong(self.halfImpactVelocityX.clone());
            jcall!(jcall!(jcall!(vehicleProperties).getVelocityProperties()).getVelocityXBasicDecimalP()).addlong(self.halfImpactVelocityX.clone());
        } else {
        }
        if impactVelocityY != 0_i64 {
            self.halfImpactVelocityY = (impactVelocityY >> 1);
            jcall!(jcall!(jcall!(ownerVehicleProperties).getVelocityProperties()).getVelocityYBasicDecimalP()).addlong(self.halfImpactVelocityY.clone());
            jcall!(jcall!(jcall!(vehicleProperties).getVelocityProperties()).getVelocityXBasicDecimalP()).addlong(self.halfImpactVelocityY.clone());
        } else {
        }
        self.collideFriction(ownerVehicleLayerInterface.clone());
        self.collideFriction(vehiclePropertiesCompositeInterface.clone());
        self.totalImpactVelocity = impactVelocityX + impactVelocityY;
    }
    

    pub fn collideFriction(&mut self, mut vehiclePropertiesCompositeInterface: Option<Rc<RefCell<dyn VehiclePropertiesCompositeInterface>>>) {
        let mut vehicleFrictionProperties: Option<Rc<RefCell<VehicleFrictionProperties>>> = jcall!(jcall!(vehiclePropertiesCompositeInterface).getVehicleProperties()).getVehicleFrictionProperties();
        jcall!(jcall!(jcall!(vehiclePropertiesCompositeInterface).getVehicleProperties()).getVehicleFrictionProperties()).friction(jcall!(jcall!(vehiclePropertiesCompositeInterface).getVehicleProperties()).getVelocityProperties(), jcall!(vehicleFrictionProperties).getCrashFrictionNominator());
    }
}
impl std::ops::Deref for CollidableVehicleBehavior {
    type Target = CollidableDestroyableDamageableBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CollidableVehicleBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehaviorDyn for CollidableVehicleBehavior {
    fn as_CollidableBaseBehavior(&self) -> &crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &self.base.base }
    fn as_CollidableBaseBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::collision::CollidableBaseBehavior::CollidableBaseBehavior { &mut self.base.base }
    fn collide(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) { Self::collide(self, ownerLayer, collidableInterfaceCompositeInterface) }
    fn isCollision(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollision(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn isCollisionInterface(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, p2: Option<Rc<RefCell<dyn crate::org::allbinary::game::collision::CollidableInterfaceCompositeInterface::CollidableInterfaceCompositeInterface>>>) -> bool { crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior::isCollisionInterface(crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn::as_CollidableDestroyableDamageableBehavior_mut(self), p1, p2) }
    fn collideInterface(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::CollidableCompositeLayer::CollidableCompositeLayerDyn>>>, mut collidableInterfaceCompositeInterface: Option<Rc<RefCell<dyn CollidableInterfaceCompositeInterface>>>) { Self::collideInterface(self, ownerLayer, collidableInterfaceCompositeInterface) }
}
impl crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehaviorDyn for CollidableVehicleBehavior {
    fn as_CollidableDestroyableDamageableBehavior(&self) -> &crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &self.base }
    fn as_CollidableDestroyableDamageableBehavior_mut(&mut self) -> &mut crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableBehavior::CollidableDestroyableDamageableBehavior { &mut self.base }
}


