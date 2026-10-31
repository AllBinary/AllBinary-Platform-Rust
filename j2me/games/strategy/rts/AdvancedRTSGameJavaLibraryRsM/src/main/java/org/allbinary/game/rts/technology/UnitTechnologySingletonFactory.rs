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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitTechnologySingletonFactory {
    pub damage: i32,
    pub health: i32,
    pub reload: i32,
    pub repair: i32,
    pub speed: i32,
}

thread_local! { pub static UnitTechnologySingletonFactory_instance: Option<Rc<RefCell<UnitTechnologySingletonFactory>>> = UnitTechnologySingletonFactory::new(); }

impl UnitTechnologySingletonFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<UnitTechnologySingletonFactory>>> {
        return UnitTechnologySingletonFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            damage: 0,
            health: 0,
            reload: 0,
            repair: 0,
            speed: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
        self.setDamage(0);
        self.setHealth(0);
        self.setReload(0);
        self.setRepair(0);
        self.setSpeed(0);
    }
    

    pub fn getDamage(&mut self) -> i32 {
        return self.damage.clone();
    }
    

    pub fn setDamage(&mut self, mut damage: i32) {
        self.damage = damage.clone();
    }
    

    pub fn getHealth(&mut self) -> i32 {
        return self.health.clone();
    }
    

    pub fn setHealth(&mut self, mut health: i32) {
        self.health = health.clone();
    }
    

    pub fn getReload(&mut self) -> i32 {
        return self.reload.clone();
    }
    

    pub fn setReload(&mut self, mut reload: i32) {
        self.reload = reload.clone();
    }
    

    pub fn getRepair(&mut self) -> i32 {
        return self.repair.clone();
    }
    

    pub fn setRepair(&mut self, mut repair: i32) {
        self.repair = repair.clone();
    }
    

    pub fn getSpeed(&mut self) -> i32 {
        return self.speed.clone();
    }
    

    pub fn setSpeed(&mut self, mut speed: i32) {
        self.speed = speed.clone();
    }
}

