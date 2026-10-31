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
// package org.allbinary.game.physics.velocity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::VelocityBehaviorBase::VelocityBehaviorBase;
use super::BasicVelocityProperties::BasicVelocityProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DragVelocityBehavior {
    pub base: VelocityBehaviorBase,
}

thread_local! { pub static DragVelocityBehavior_instance: Option<Rc<RefCell<DragVelocityBehavior>>> = DragVelocityBehavior::new(); }

impl DragVelocityBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: VelocityBehaviorBase::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn reduce(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
        if jcall!(jcall!(velocityProperties).getVelocityXBasicDecimalP()).getUnscaled() != 0_i64 {
            let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(velocityProperties).getVelocityXBasicDecimalP();
            jcall!(basicDecimal).multiplyint(nominator.clone());
            jcall!(basicDecimal).divideint(denominator.clone());
        }
        if jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).getUnscaled() != 0_i64 {
            let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(velocityProperties).getVelocityYBasicDecimalP();
            jcall!(basicDecimal).multiplyint(nominator.clone());
            jcall!(basicDecimal).divideint(denominator.clone());
        }
    }
    

    pub fn reduceX(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
        let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(velocityProperties).getVelocityXBasicDecimalP();
        if jcall!(basicDecimal).getUnscaled() != 0_i64 {
            jcall!(basicDecimal).multiplyint(nominator.clone());
            jcall!(basicDecimal).divideint(denominator.clone());
        }
    }
    

    pub fn reduceY(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) {
        if jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).getUnscaled() != 0_i64 {
            let mut basicDecimal: Option<Rc<RefCell<BasicDecimal>>> = jcall!(velocityProperties).getVelocityYBasicDecimalP();
            jcall!(basicDecimal).multiplyint(nominator.clone());
            jcall!(basicDecimal).divideint(denominator.clone());
        }
    }
}
impl std::ops::Deref for DragVelocityBehavior {
    type Target = VelocityBehaviorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DragVelocityBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::physics::velocity::VelocityBehaviorBase::VelocityBehaviorBaseDyn for DragVelocityBehavior {
    fn as_VelocityBehaviorBase(&self) -> &crate::org::allbinary::game::physics::velocity::VelocityBehaviorBase::VelocityBehaviorBase { &self.base }
    fn as_VelocityBehaviorBase_mut(&mut self) -> &mut crate::org::allbinary::game::physics::velocity::VelocityBehaviorBase::VelocityBehaviorBase { &mut self.base }
    fn reduce(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { Self::reduce(self, velocityProperties, nominator, denominator) }
    fn reduceX(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { Self::reduceX(self, velocityProperties, nominator, denominator) }
    fn reduceY(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut nominator: i32, mut denominator: i32) { Self::reduceY(self, velocityProperties, nominator, denominator) }
}


