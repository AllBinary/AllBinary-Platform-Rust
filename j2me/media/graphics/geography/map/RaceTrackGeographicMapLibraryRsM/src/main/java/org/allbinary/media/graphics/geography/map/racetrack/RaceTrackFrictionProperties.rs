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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::physics::friction::FrictionData::FrictionData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackFrictionProperties {
    pub ROAD_FRICTION_NOMINATOR: i32,
    pub GROUND_FRICTION_NOMINATOR: i32,
}


impl RaceTrackFrictionProperties {
    

    pub fn new_struct(mut roadFriction: i32, mut groundFriction: i32) -> Self {
        let mut this = Self {
            ROAD_FRICTION_NOMINATOR: 0,
            GROUND_FRICTION_NOMINATOR: 0,
        };
        this.ROAD_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - roadFriction;
        this.GROUND_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - groundFriction;
        this
    }

    pub fn new(mut roadFriction: i32, mut groundFriction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(roadFriction, groundFriction))))
    }
    

    pub fn getRoadFrictionNominator(&mut self) -> i32 {
        return self.ROAD_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getGroundFrictionNominator(&mut self) -> i32 {
        return self.GROUND_FRICTION_NOMINATOR.clone();
    }
}

