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
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::game::state::GameState::GameState;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdConfiguration {
    pub INNERACTIVE: String,
    pub LEADBOLT: String,
    pub ADMOB: String,
}


impl AdConfiguration {
    

    pub fn new_struct(mut advertIdArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Self {
        let mut this = Self {
            INNERACTIVE: String::new(),
            LEADBOLT: String::new(),
            ADMOB: String::new(),
        };
        this.INNERACTIVE = (advertIdArray[(0) as usize]).clone() /* as String */;
        this.ADMOB = (advertIdArray[(1) as usize]).clone() /* as String */;
        this.LEADBOLT = (advertIdArray[(2) as usize]).clone() /* as String */;
        this
    }

    pub fn new(mut advertIdArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(advertIdArray))))
    }
    

    pub fn getValidAdSpots(&mut self) -> Vec<Option<Rc<RefCell<Direction>>>> {
        return vec![None; (0) as usize];
    }
    

    pub fn process(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) {
    }
    

    pub fn processDemo(&mut self, mut state: i32) {
    }
    

    pub fn setShowAds(&mut self) {
    }
    

    pub fn setToggleAds(&mut self, mut showAds: bool) {
    }
    

    pub fn isShowAds(&mut self) -> bool {
        return true;
    }
}
pub trait AdConfigurationDyn {
    fn as_AdConfiguration(&self) -> &AdConfiguration;
    fn as_AdConfiguration_mut(&mut self) -> &mut AdConfiguration;
    fn getValidAdSpots(&mut self) -> Vec<Option<Rc<RefCell<Direction>>>> { AdConfiguration::getValidAdSpots(self.as_AdConfiguration_mut()) }
    fn process(&mut self, mut gameState: Option<Rc<RefCell<GameState>>>) { AdConfiguration::process(self.as_AdConfiguration_mut(), gameState) }
    fn processDemo(&mut self, mut state: i32) { AdConfiguration::processDemo(self.as_AdConfiguration_mut(), state) }
    fn setShowAds(&mut self) { AdConfiguration::setShowAds(self.as_AdConfiguration_mut()) }
    fn setToggleAds(&mut self, mut showAds: bool) { AdConfiguration::setToggleAds(self.as_AdConfiguration_mut(), showAds) }
    fn isShowAds(&mut self) -> bool { AdConfiguration::isShowAds(self.as_AdConfiguration_mut()) }
}
impl AdConfigurationDyn for AdConfiguration {
    fn as_AdConfiguration(&self) -> &AdConfiguration { self }
    fn as_AdConfiguration_mut(&mut self) -> &mut AdConfiguration { self }
}


