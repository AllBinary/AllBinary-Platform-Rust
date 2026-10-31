// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.game.physics.velocity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::BasicVelocityProperties::BasicVelocityProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VelocityBehaviorBase {
}


impl VelocityBehaviorBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn reduce(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
    }
    

    pub fn reduceX(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
    }
    

    pub fn reduceY(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
    }
}
pub trait VelocityBehaviorBaseDyn {
    fn as_VelocityBehaviorBase(&self) -> &VelocityBehaviorBase;
    fn as_VelocityBehaviorBase_mut(&mut self) -> &mut VelocityBehaviorBase;
    fn reduce(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { VelocityBehaviorBase::reduce(self.as_VelocityBehaviorBase_mut(), velocityProperties, nominator, denominator) }
    fn reduceX(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { VelocityBehaviorBase::reduceX(self.as_VelocityBehaviorBase_mut(), velocityProperties, nominator, denominator) }
    fn reduceY(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { VelocityBehaviorBase::reduceY(self.as_VelocityBehaviorBase_mut(), velocityProperties, nominator, denominator) }
}
impl VelocityBehaviorBaseDyn for VelocityBehaviorBase {
    fn as_VelocityBehaviorBase(&self) -> &VelocityBehaviorBase { self }
    fn as_VelocityBehaviorBase_mut(&mut self) -> &mut VelocityBehaviorBase { self }
}


