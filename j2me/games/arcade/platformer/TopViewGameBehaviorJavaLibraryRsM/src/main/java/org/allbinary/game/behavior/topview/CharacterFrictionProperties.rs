// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
use crate::org::allbinary::game::physics::FrictionProperties::FrictionProperties;
use crate::org::allbinary::game::physics::friction::FrictionData::FrictionData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CharacterFrictionProperties {
    pub base: FrictionProperties,
    pub WATER_FRICTION_NOMINATOR: i32,
    pub AIR_FRICTION_NOMINATOR: i32,
    pub COLLISION_FRICTION_NOMINATOR: i32,
}


impl CharacterFrictionProperties {
    

    pub fn new_struct(mut airFriction: i32, mut waterFriction: i32, mut collisionFriction: i32) -> Self {
        let mut this = Self {
            base: FrictionProperties::new_struct(),
            WATER_FRICTION_NOMINATOR: 0,
            AIR_FRICTION_NOMINATOR: 0,
            COLLISION_FRICTION_NOMINATOR: 0,
        };
        this.AIR_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - airFriction;
        this.COLLISION_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - collisionFriction;
        this.WATER_FRICTION_NOMINATOR = FrictionData::getFrictionDenominator() - waterFriction;
        this
    }

    pub fn new(mut airFriction: i32, mut waterFriction: i32, mut collisionFriction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(airFriction, waterFriction, collisionFriction))))
    }
    

    pub fn getWATER_FRICTION_NOMINATOR(&mut self) -> i32 {
        return self.WATER_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getAIR_FRICTION_NOMINATOR(&mut self) -> i32 {
        return self.AIR_FRICTION_NOMINATOR.clone();
    }
    

    pub fn getCOLLISION_FRICTION_NOMINATOR(&mut self) -> i32 {
        return self.COLLISION_FRICTION_NOMINATOR.clone();
    }
}
impl std::ops::Deref for CharacterFrictionProperties {
    type Target = FrictionProperties;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CharacterFrictionProperties {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



