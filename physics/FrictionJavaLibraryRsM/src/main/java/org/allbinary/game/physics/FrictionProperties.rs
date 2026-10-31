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
// package org.allbinary.game.physics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::physics::friction::FrictionData::FrictionData;
use crate::org::allbinary::game::physics::velocity::DragVelocityBehavior::DragVelocityBehavior;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FrictionProperties {
}


impl FrictionProperties {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn friction(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut nominator: i32) {
        jcall!(crate::org::allbinary::game::physics::velocity::DragVelocityBehavior::DragVelocityBehavior_instance.with(Clone::clone)).reduce(velocityProperties.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>), nominator.clone(), FrictionData::getFrictionDenominator());
    }
}
pub trait FrictionPropertiesDyn {
    fn as_FrictionProperties(&self) -> &FrictionProperties;
    fn as_FrictionProperties_mut(&mut self) -> &mut FrictionProperties;
    fn friction(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut nominator: i32) { FrictionProperties::friction(self.as_FrictionProperties_mut(), velocityProperties, nominator) }
}
impl FrictionPropertiesDyn for FrictionProperties {
    fn as_FrictionProperties(&self) -> &FrictionProperties { self }
    fn as_FrictionProperties_mut(&mut self) -> &mut FrictionProperties { self }
}


