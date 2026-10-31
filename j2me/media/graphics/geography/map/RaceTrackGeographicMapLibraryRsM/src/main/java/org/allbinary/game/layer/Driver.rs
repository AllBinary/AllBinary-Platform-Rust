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
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use super::Aggression::Aggression;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Driver {
    pub turnSpeed: Option<i32>,
    pub trackPathSelectionSkill: Option<i32>,
    pub aggression: Option<Rc<RefCell<Aggression>>>,
}

thread_local! { pub static Driver_NULL_DRIVER: Option<Rc<RefCell<Driver>>> = Driver::new(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0), Aggression::new(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0), jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0))); }

impl Driver {
    

    pub fn new_struct(mut turnSpeed: Option<i32>, mut trackPathSelectionSkill: Option<i32>, mut aggression: Option<Rc<RefCell<Aggression>>>) -> Self {
        let mut this = Self {
            turnSpeed: None,
            trackPathSelectionSkill: None,
            aggression: None,
        };
        this.turnSpeed = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.trackPathSelectionSkill = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.setTurnSpeed(turnSpeed.clone());
        this.setTrackPathSelectionSkill(trackPathSelectionSkill.clone());
        this.aggression = aggression.clone();
        this
    }

    pub fn new(mut turnSpeed: Option<i32>, mut trackPathSelectionSkill: Option<i32>, mut aggression: Option<Rc<RefCell<Aggression>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(turnSpeed, trackPathSelectionSkill, aggression))))
    }
    

    pub fn getTurnSpeed(&mut self) -> Option<i32> {
        return self.turnSpeed.clone();
    }
    

    pub fn setTurnSpeed(&mut self, mut turnSpeed: Option<i32>) {
        self.turnSpeed = turnSpeed.clone();
    }
    

    pub fn getTrackPathSelectionSkill(&mut self) -> Option<i32> {
        return self.trackPathSelectionSkill.clone();
    }
    

    pub fn setTrackPathSelectionSkill(&mut self, mut trackPathSelectionSkill: Option<i32>) {
        self.trackPathSelectionSkill = trackPathSelectionSkill.clone();
    }
    

    pub fn getAggression(&mut self) -> Option<Rc<RefCell<Aggression>>> {
        return self.aggression.clone();
    }
    

    pub fn setAggression(&mut self, mut aggression: Option<Rc<RefCell<Aggression>>>) {
        self.aggression = aggression.clone();
    }
}

