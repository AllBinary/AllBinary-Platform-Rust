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
use super::BasicVelocityProperties::BasicVelocityProperties;
use super::VelocityInterface::VelocityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VelocityProperties {
    pub base: BasicVelocityProperties,
    pub maxForwardVelocity: i32,
    pub maxReverseVelocity: i32,
}

thread_local! { pub static VelocityProperties_NULL_VELOCITY_PROPERTIES: Option<Rc<RefCell<VelocityProperties>>> = VelocityProperties::new(0, 0); }
// impl VelocityInterface for VelocityProperties with the methods of the trait

impl VelocityProperties {
    

    pub fn new_struct(mut maxForwardVelocity: i32, mut maxReverseVelocity: i32) -> Self {
        let mut this = Self {
            base: BasicVelocityProperties::new_struct(),
            maxForwardVelocity: 0,
            maxReverseVelocity: 0,
        };
        this.setMaxForwardVelocity(maxForwardVelocity.clone());
        this.setMaxReverseVelocity(maxReverseVelocity.clone());
        this
    }

    pub fn new(mut maxForwardVelocity: i32, mut maxReverseVelocity: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxForwardVelocity, maxReverseVelocity))))
    }
    

    pub fn getMaxForwardVelocity(&mut self) -> i32 {
        return self.maxForwardVelocity.clone();
    }
    

    pub fn setMaxForwardVelocity(&mut self, mut maxForwardVelocity: i32) {
        self.maxForwardVelocity = maxForwardVelocity.clone();
    }
    

    pub fn getMaxReverseVelocity(&mut self) -> i32 {
        return self.maxReverseVelocity.clone();
    }
    

    pub fn setMaxReverseVelocity(&mut self, mut maxReverseVelocity: i32) {
        self.maxReverseVelocity = maxReverseVelocity.clone();
    }
    

    pub fn limitMaxXYForwardVelocity(&mut self) {
        { let java_arguments = (self.getMaxForwardVelocity(),); self.limitMaxXYVelocity(java_arguments.0) };
    }
    

    pub fn limitMaxXYReverseVelocity(&mut self) {
        { let java_arguments = (self.getMaxReverseVelocity(),); self.limitMaxXYVelocity(java_arguments.0) };
    }
    

    pub fn limitMaxYForwardVelocity(&mut self) {
        { let java_arguments = (self.getMaxForwardVelocity(),); self.limitMaxYVelocity(java_arguments.0) };
    }
    

    pub fn limitMaxYReverseVelocity(&mut self) {
        { let java_arguments = (self.getMaxReverseVelocity(),); self.limitMaxYVelocity(java_arguments.0) };
    }
    

    pub fn limitMaxXForwardVelocity(&mut self) {
        { let java_arguments = (self.getMaxForwardVelocity(),); self.limitMaxXVelocity(java_arguments.0) };
    }
    

    pub fn limitMaxXReverseVelocity(&mut self) {
        { let java_arguments = (self.getMaxReverseVelocity(),); self.limitMaxXVelocity(java_arguments.0) };
    }
    

    pub fn isOverXYMaxForwardVelocity(&mut self) -> bool {
        return { let java_arguments = (self.getMaxForwardVelocity(),); self.isOverXYMaxVelocity(java_arguments.0) };
    }
    

    pub fn isOverXYMaxReverseVelocity(&mut self) -> bool {
        return { let java_arguments = (self.getMaxReverseVelocity(),); self.isOverXYMaxVelocity(java_arguments.0) };
    }
    

    pub fn limitXYToForwardAndReverseMaxVelocity(&mut self) {
        self.limitMaxXYForwardVelocity();
        self.limitMaxXYReverseVelocity();
    }
    

    pub fn limitMaxXYVelocity(&mut self, mut maxVelocity: i32) {
        self.limitMaxXVelocity(maxVelocity.clone());
        self.limitMaxYVelocity(maxVelocity.clone());
    }
    

    pub fn limitMaxPositiveYVelocity(&mut self, mut maxVelocity: i32) {
        if jcall!(self.velocityYBasicDecimal).getUnscaled() > ((maxVelocity) as i64) {
            jcall!(self.velocityYBasicDecimal).setint(maxVelocity.clone());
        }
    }
    

    pub fn limitMaxNegativeYVelocity(&mut self, mut maxVelocity: i32) {
        if jcall!(self.velocityYBasicDecimal).getUnscaled() < ((-maxVelocity) as i64) {
            jcall!(self.velocityYBasicDecimal).setint(-maxVelocity);
        }
    }
    

    pub fn limitMaxYVelocity(&mut self, mut maxVelocity: i32) {
        self.limitMaxPositiveYVelocity(maxVelocity.clone());
        self.limitMaxNegativeYVelocity(maxVelocity.clone());
    }
    

    pub fn limitMaxXVelocity(&mut self, mut maxVelocity: i32) {
        if jcall!(self.velocityXBasicDecimal).getUnscaled() > ((maxVelocity) as i64) {
            jcall!(self.velocityXBasicDecimal).setint(maxVelocity.clone());
        }
        if jcall!(self.velocityXBasicDecimal).getUnscaled() < ((-maxVelocity) as i64) {
            jcall!(self.velocityXBasicDecimal).setint(-maxVelocity);
        }
    }
    

    pub fn isOverXYMaxVelocity(&mut self, mut maxVelocity: i32) -> bool {
        if jcall!(self.velocityXBasicDecimal).getUnscaled() > ((maxVelocity) as i64) {
            return true;
        }
        if jcall!(self.velocityXBasicDecimal).getUnscaled() < ((-maxVelocity) as i64) {
            return true;
        }
        if jcall!(self.velocityYBasicDecimal).getUnscaled() > ((maxVelocity) as i64) {
            return true;
        }
        if jcall!(self.velocityYBasicDecimal).getUnscaled() < ((-maxVelocity) as i64) {
            return true;
        }
        return false;
    }
    

    pub fn setVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) {
        self.base.setVelocityi(magnitude.clone(), angle.clone(), otherAngle.clone());
        self.limitXYToForwardAndReverseMaxVelocity();
    }
    

    pub fn addVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) {
        self.base.addVelocityi(magnitude.clone(), angle.clone(), otherAngle.clone());
        self.limitXYToForwardAndReverseMaxVelocity();
    }
}
impl std::ops::Deref for VelocityProperties {
    type Target = BasicVelocityProperties;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VelocityProperties {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn for VelocityProperties {
    fn as_BasicVelocityProperties(&self) -> &crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties { &self.base }
    fn as_BasicVelocityProperties_mut(&mut self) -> &mut crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties { &mut self.base }
    fn setVelocityi(&mut self, mut magnitude: i64, mut angle: i32, mut otherAngle: i32) { Self::setVelocityi(self, magnitude, angle, otherAngle) }
}


