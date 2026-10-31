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
// package org.allbinary.game.part.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::game::combat::weapon::WeaponProperties::WeaponProperties;
use crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayer;
use crate::org::allbinary::game::layer::weapon::WeaponLayerCircularPool::WeaponLayerCircularPool;
use crate::org::allbinary::game::score::ScoreableInterface::ScoreableInterface;
use crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationship;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use super::BasicWeaponPart::BasicWeaponPart;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ArcedMultiBeamWeaponPart {
    pub base: BasicWeaponPart,
    pub weaponLayerCircularStaticPool: Option<Rc<RefCell<WeaponLayerCircularPool>>>,
    pub total: i32,
    pub TOTAL_ANGLE: i32,
}

impl crate::org::allbinary::game::part::PartInterface::PartInterface for ArcedMultiBeamWeaponPart {
}

impl ArcedMultiBeamWeaponPart {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponLayerCircularStaticPool: Option<Rc<RefCell<WeaponLayerCircularPool>>>, mut total: i32, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) -> Self {
        let mut this = Self {
            base: BasicWeaponPart::new_struct(animationInterface.clone(), sourceLayerInterface.clone(), weaponProperties.clone(), scoreableInterface.clone(), relativeRelationship.clone()),
            weaponLayerCircularStaticPool: None,
            total: 0,
            TOTAL_ANGLE: 0,
        };
        this.TOTAL_ANGLE = ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32);
        // super(...) is in the struct literal above
        this.total = total.clone();
        this.weaponLayerCircularStaticPool = weaponLayerCircularStaticPool.clone();
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut sourceLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut weaponLayerCircularStaticPool: Option<Rc<RefCell<WeaponLayerCircularPool>>>, mut total: i32, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>, mut relativeRelationship: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::RelativeRelationship::RelativeRelationshipDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, sourceLayerInterface, weaponLayerCircularStaticPool, total, weaponProperties, scoreableInterface, relativeRelationship))))
    }
    

    // throws Exception
    pub fn processScore(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>) {
        let mut increment: i16 = ((((self.total - 1) / self.TOTAL_ANGLE)) as i16);
        let mut minAngle: i16 = (((((angle) as i32) - ((((increment) as i32) * self.total) >> 1))) as i16);
        let mut next: i32 = 0;
        let mut weaponLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn>>> = None;
        for index in (0..self.total) {
            weaponLayer = { let java_receiver = self.weaponLayerCircularStaticPool.clone(); let java_arguments = (self.getOwnerLayerInterface(), jcall!(self.relativeRelationship).getX(), jcall!(self.relativeRelationship).getY(), 0, (((minAngle) as i32) + ((increment) as i32)), ((otherAngle) as i32), weaponProperties.clone(), scoreableInterface.clone(),); let java_result = jcall!(java_receiver).getInstanceWeaponLayer(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6, java_arguments.7); java_result };
            jcall!(allbinaryLayerManager).append(weaponLayer.clone());
            next += ((increment) as i32).clone();
        }
    }
}
impl std::ops::Deref for ArcedMultiBeamWeaponPart {
    type Target = BasicWeaponPart;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ArcedMultiBeamWeaponPart {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn for ArcedMultiBeamWeaponPart {
    fn as_BasicWeaponPart(&self) -> &crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart { &self.base }
    fn as_BasicWeaponPart_mut(&mut self) -> &mut crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart { &mut self.base }
}
impl crate::org::allbinary::game::part::weapon::SalvoInterface::SalvoInterface for ArcedMultiBeamWeaponPart {
    fn process(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, p2: i16, p3: i16) { crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart::process(crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn::as_BasicWeaponPart_mut(self), p1, p2, p3) }
    fn processScore(&mut self, mut allbinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>, mut angle: i16, mut otherAngle: i16, mut weaponProperties: Option<Rc<RefCell<WeaponProperties>>>, mut scoreableInterface: Option<Rc<RefCell<dyn ScoreableInterface>>>) { Self::processScore(self, allbinaryLayerManager, angle, otherAngle, weaponProperties, scoreableInterface) }
}


