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
use crate::org::allbinary::game::physics::FrictionProperties::FrictionProperties;
use crate::org::allbinary::game::physics::friction::FrictionData::FrictionData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VehicleFrictionProperties {
    pub base: FrictionProperties,
    pub TIRE_FRICTION_NOMINATOR: i32,
    pub BRAKE_FRICTION_NOMINATOR: i32,
    pub AIR_FRICTION_NOMINATOR: i32,
    pub CRASH_FRICTION_NOMINATOR: i32,
}


impl VehicleFrictionProperties {
    

    pub fn new_struct(mut tireFriction: i32, mut brakeFriction: i32, mut airFriction: i32, mut crashFriction: i32) -> Self {
        let mut this = Self {
            base: FrictionProperties::new_struct(),
            TIRE_FRICTION_NOMINATOR: 0,
            BRAKE_FRICTION_NOMINATOR: 0,
            AIR_FRICTION_NOMINATOR: 0,
            CRASH_FRICTION_NOMINATOR: 0,
        };
        this.TIRE_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - tireFriction;
        this.BRAKE_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - brakeFriction;
        this.AIR_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - airFriction;
        this.CRASH_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - crashFriction;
        this
    }

    pub fn new(mut tireFriction: i32, mut brakeFriction: i32, mut airFriction: i32, mut crashFriction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(tireFriction, brakeFriction, airFriction, crashFriction))))
    }
    

    pub fn getTireFrictionNominator(&mut self) -> i32 {
        return self.TIRE_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getBrakeFrictionNominator(&mut self) -> i32 {
        return self.BRAKE_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getAirFrictionNominator(&mut self) -> i32 {
        return self.AIR_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getCrashFrictionNominator(&mut self) -> i32 {
        return self.CRASH_FRICTION_NOMINATOR.clone();
    }
}
impl std::ops::Deref for VehicleFrictionProperties {
    type Target = FrictionProperties;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VehicleFrictionProperties {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::physics::FrictionProperties::FrictionPropertiesDyn for VehicleFrictionProperties {
    fn as_FrictionProperties(&self) -> &crate::org::allbinary::game::physics::FrictionProperties::FrictionProperties { &self.base }
    fn as_FrictionProperties_mut(&mut self) -> &mut crate::org::allbinary::game::physics::FrictionProperties::FrictionProperties { &mut self.base }
}


