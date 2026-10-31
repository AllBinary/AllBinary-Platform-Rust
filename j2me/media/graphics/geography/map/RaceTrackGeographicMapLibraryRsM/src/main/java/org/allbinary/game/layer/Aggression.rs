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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Aggression {
    pub ram: Option<i32>,
    pub projectile: Option<i32>,
    pub drops: Option<i32>,
}


impl Aggression {
    

    pub fn new_struct(mut ram: Option<i32>, mut projectile: Option<i32>, mut drops: Option<i32>) -> Self {
        let mut this = Self {
            ram: None,
            projectile: None,
            drops: None,
        };
        this.ram = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.projectile = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.drops = jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(0);
        this.setRam(ram.clone());
        this.setProjectile(projectile.clone());
        this.setDrops(drops.clone());
        this
    }

    pub fn new(mut ram: Option<i32>, mut projectile: Option<i32>, mut drops: Option<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(ram, projectile, drops))))
    }
    

    pub fn getRam(&mut self) -> Option<i32> {
        return self.ram.clone();
    }
    

    pub fn setRam(&mut self, mut ram: Option<i32>) {
        self.ram = ram.clone();
    }
    

    pub fn getProjectile(&mut self) -> Option<i32> {
        return self.projectile.clone();
    }
    

    pub fn setProjectile(&mut self, mut projectile: Option<i32>) {
        self.projectile = projectile.clone();
    }
    

    pub fn getDrops(&mut self) -> Option<i32> {
        return self.drops.clone();
    }
    

    pub fn setDrops(&mut self, mut drops: Option<i32>) {
        self.drops = drops.clone();
    }
}

