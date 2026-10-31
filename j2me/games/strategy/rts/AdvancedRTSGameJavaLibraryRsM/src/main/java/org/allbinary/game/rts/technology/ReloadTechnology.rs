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
// package org.allbinary.game.rts.technology
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::RTSInterface::RTSInterface;
use super::UnitTechnologySingletonFactory::UnitTechnologySingletonFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ReloadTechnology {
    pub level: i32,
    pub maxLevel: i32,
}

impl RTSInterface for ReloadTechnology {
    fn isSelfUpgradeable(&mut self) -> bool { Self::isSelfUpgradeable(self) }
    fn isCompleted(&mut self) -> bool { Self::isCompleted(self) }
    fn getPercentComplete(&mut self) -> i32 { Self::getPercentComplete(self) }
    fn getLevel(&mut self) -> i32 { Self::getLevel(self) }
    fn getMaxLevel(&mut self) -> i32 { Self::getMaxLevel(self) }
    fn getCost(&mut self) -> i32 { Self::getCost(self) }
    fn downgrade(&mut self) { Self::downgrade(self) }
    fn getDowngradeCost(&mut self) -> i32 { Self::getDowngradeCost(self) }
    fn isDowngradeable(&mut self) -> bool { Self::isDowngradeable(self) }
    fn getUpgradeCost(&mut self) -> i32 { Self::getUpgradeCost(self) }
    fn isUpgradeable(&mut self) -> bool { Self::isUpgradeable(self) }
    fn upgrade(&mut self) { Self::upgrade(self) }
}

impl ReloadTechnology {
    

    pub fn new_struct(mut maxLevel: i32) -> Self {
        let mut this = Self {
            level: 0,
            maxLevel: 0,
        };
        this.maxLevel = maxLevel.clone();
        this
    }

    pub fn new(mut maxLevel: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxLevel))))
    }
    

    pub fn isCompleted(&mut self) -> bool {
        return false;
    }
    

    pub fn getPercentComplete(&mut self) -> i32 {
        return 0;
    }
    

    pub fn getLevel(&mut self) -> i32 {
        return self.level.clone();
    }
    

    pub fn getMaxLevel(&mut self) -> i32 {
        return self.maxLevel.clone();
    }
    

    pub fn getCost(&mut self) -> i32 {
        return 0;
    }
    

    pub fn downgrade(&mut self) {
    }
    

    pub fn getDowngradeCost(&mut self) -> i32 {
        return 0;
    }
    

    pub fn isDowngradeable(&mut self) -> bool {
        return false;
    }
    

    pub fn getUpgradeCost(&mut self) -> i32 {
        return (self.getLevel() * self.getLevel()) * 1000;
    }
    

    pub fn isUpgradeable(&mut self) -> bool {
        if self.getLevel() < self.getMaxLevel() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn upgrade(&mut self) {
        self.level += 1;
        let mut unitTechnologySingletonFactory: Option<Rc<RefCell<UnitTechnologySingletonFactory>>> = UnitTechnologySingletonFactory::getInstance();
        jcall!(unitTechnologySingletonFactory).setReload(jcall!(unitTechnologySingletonFactory).getReload() + (self.getLevel() * 10));
    }
    

    pub fn isSelfUpgradeable(&mut self) -> bool {
        return false;
    }
}

