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
// package org.allbinary.game.physics.acceleration
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityProperties;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GravityUtil {
    pub GAME_GRAVITY_VELOCITY: i32,
    pub MAX_GAME_VELOCITY: i32,
}

thread_local! { pub static GravityUtil_instance: Option<Rc<RefCell<GravityUtil>>> = GravityUtil::new(); }

impl GravityUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            GAME_GRAVITY_VELOCITY: 0,
            MAX_GAME_VELOCITY: 0,
        };
        this.GAME_GRAVITY_VELOCITY = 1800;
        this.MAX_GAME_VELOCITY = 18000;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<GravityUtil>>> {
        return GravityUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn process(&mut self, mut velocityProperties: Option<Rc<RefCell<dyn crate::org::allbinary::game::physics::velocity::BasicVelocityProperties::BasicVelocityPropertiesDyn>>>, mut customGravity: i32) {
        jcall!(jcall!(velocityProperties).getVelocityYBasicDecimalP()).addint(customGravity.clone());
    }
}

