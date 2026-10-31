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
// package org.allbinary.game.layer.weapon
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::WeaponLayer::WeaponLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WeaponLayerArrayLayerCircularStaticPool {
    pub MAX: i32,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub ALL_WEAPONLAYER_ARRAY: Vec<Vec<Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>,
}

thread_local! { pub static WeaponLayerArrayLayerCircularStaticPool_instance: Option<Rc<RefCell<WeaponLayerArrayLayerCircularStaticPool>>> = WeaponLayerArrayLayerCircularStaticPool::new(); }

impl WeaponLayerArrayLayerCircularStaticPool {

    pub fn new_struct() -> Self {
        let mut this = Self {
            MAX: 0,
            circularIndexUtil: None,
            ALL_WEAPONLAYER_ARRAY: Vec::new(),
        };
        this.MAX = 5;
        this.circularIndexUtil = CircularIndexUtil::createInstance(this.MAX.clone());
        this.ALL_WEAPONLAYER_ARRAY = vec![Vec::new(); (4) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WeaponLayerArrayLayerCircularStaticPool>>> {
        return WeaponLayerArrayLayerCircularStaticPool_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self) {
        let mut WEAPONLAYER_ARRAY: Vec<Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>> = vec![Vec::new(); (self.MAX * 2) as usize];
        let mut TWO_WEAPONLAYER_ARRAY: Vec<Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>> = vec![Vec::new(); (self.MAX) as usize];
        let mut THREE_WEAPONLAYER_ARRAY: Vec<Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>>> = vec![Vec::new(); (self.MAX) as usize];
        for index in (0..self.MAX) {
            WEAPONLAYER_ARRAY[(index) as usize] = vec![None; (1) as usize];
            TWO_WEAPONLAYER_ARRAY[(index) as usize] = vec![None; (2) as usize];
            THREE_WEAPONLAYER_ARRAY[(index) as usize] = vec![None; (3) as usize];
        }
        self.ALL_WEAPONLAYER_ARRAY[(1) as usize] = WEAPONLAYER_ARRAY.clone();
        self.ALL_WEAPONLAYER_ARRAY[(2) as usize] = TWO_WEAPONLAYER_ARRAY.clone();
        self.ALL_WEAPONLAYER_ARRAY[(3) as usize] = THREE_WEAPONLAYER_ARRAY.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn getInstanceArray(&mut self, mut size: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn>>>> {
        let mut weaponLayerArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::weapon::WeaponLayer::WeaponLayerDyn>>>> = (self.ALL_WEAPONLAYER_ARRAY[(size) as usize][(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as  */;
        jcall!(self.circularIndexUtil).next();
        return weaponLayerArray.clone();
    }
}

