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
// package org.allbinary.game.behavior.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::behavior::GameLayerBehavior::GameLayerBehavior;
use crate::org::allbinary::game::physics::acceleration::BasicAccelerationProperties::BasicAccelerationProperties;
use crate::org::allbinary::game::physics::velocity::VelocityProperties::VelocityProperties;
use super::InitialJumpBehavior::InitialJumpBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TopViewGameLayerBehavior {
    pub base: GameLayerBehavior,
    pub maxGravityActionIndex: i32,
    pub isJumpAction: bool,
    pub isJumpOver: bool,
    pub isFallingWithoutJumpAttempt: bool,
    pub gravityActionIndex: i32,
}


impl TopViewGameLayerBehavior {
    

    pub fn new_struct(mut maxGravityActionIndex: i32) -> Self {
        let mut this = Self {
            base: GameLayerBehavior::new_struct(),
            maxGravityActionIndex: 0,
            isJumpAction: false,
            isJumpOver: false,
            isFallingWithoutJumpAttempt: false,
            gravityActionIndex: 0,
        };
        this.isJumpAction = true;
        this.isJumpOver = false;
        this.isFallingWithoutJumpAttempt = false;
        this.gravityActionIndex = 0;
        this.maxGravityActionIndex = maxGravityActionIndex.clone();
        this
    }

    pub fn new(mut maxGravityActionIndex: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxGravityActionIndex))))
    }
    

    pub fn gravity(&mut self) {
        if self.gravityActionIndex == 0 {
            self.gravityActionIndex += 1;
            self.isFallingWithoutJumpAttempt = true;
        }
    }
    

    pub fn land(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>) {
        jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).setint(0);
        self.landReset();
    }
    

    pub fn landReset(&mut self) {
        self.gravityActionIndex = 0;
        self.isFallingWithoutJumpAttempt = false;
        self.isJumpAction = true;
        self.isJumpOver = false;
    }
    

    pub fn up(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut acceleration: Option<Rc<RefCell<BasicAccelerationProperties>>>, mut jumpBehavior: Option<Rc<RefCell<InitialJumpBehavior>>>, mut accelerationMultiplier: i32) {
        if !self.isJumpOver {
            if self.gravityActionIndex < self.maxGravityActionIndex {
                let mut acceleration2: i32 = -jcall!(acceleration).getForward() * accelerationMultiplier;
                jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).addint(acceleration2.clone());
                jcall!(velocityProperties).limitXYToForwardAndReverseMaxVelocity();
                self.gravityActionIndex += 1;
            } else {
            }
        } else {
        }
        if self.isJumpAction {
            jcall!(jumpBehavior).process();
            self.isJumpAction = false;
        }
    }
    

    pub fn inputFrames(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>) {
        if self.gravityActionIndex > 0 && jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).getUnscaled() > 0 {
            self.isJumpOver = true;
        }
    }
}
impl std::ops::Deref for TopViewGameLayerBehavior {
    type Target = GameLayerBehavior;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TopViewGameLayerBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait TopViewGameLayerBehaviorDyn {
    fn as_TopViewGameLayerBehavior(&self) -> &TopViewGameLayerBehavior;
    fn as_TopViewGameLayerBehavior_mut(&mut self) -> &mut TopViewGameLayerBehavior;
    fn gravity(&mut self) { TopViewGameLayerBehavior::gravity(self.as_TopViewGameLayerBehavior_mut()) }
    fn land(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>) { TopViewGameLayerBehavior::land(self.as_TopViewGameLayerBehavior_mut(), velocityProperties) }
    fn landReset(&mut self) { TopViewGameLayerBehavior::landReset(self.as_TopViewGameLayerBehavior_mut()) }
    fn up(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>, mut acceleration: Option<Rc<RefCell<BasicAccelerationProperties>>>, mut jumpBehavior: Option<Rc<RefCell<InitialJumpBehavior>>>, mut accelerationMultiplier: i32) { TopViewGameLayerBehavior::up(self.as_TopViewGameLayerBehavior_mut(), velocityProperties, acceleration, jumpBehavior, accelerationMultiplier) }
    fn inputFrames(&mut self, mut velocityProperties: Option<Rc<RefCell<VelocityProperties>>>) { TopViewGameLayerBehavior::inputFrames(self.as_TopViewGameLayerBehavior_mut(), velocityProperties) }
}
impl TopViewGameLayerBehaviorDyn for TopViewGameLayerBehavior {
    fn as_TopViewGameLayerBehavior(&self) -> &TopViewGameLayerBehavior { self }
    fn as_TopViewGameLayerBehavior_mut(&mut self) -> &mut TopViewGameLayerBehavior { self }
}



