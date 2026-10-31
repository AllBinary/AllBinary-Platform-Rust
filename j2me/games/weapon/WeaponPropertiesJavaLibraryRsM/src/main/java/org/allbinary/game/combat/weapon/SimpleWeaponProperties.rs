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
// package org.allbinary.game.combat.weapon
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

pub struct SimpleWeaponProperties {
    pub damage: i32,
    pub dissipation: i16,
    pub range: i32,
}


impl SimpleWeaponProperties {

    pub fn new_struct() -> Self {
        let mut this = Self {
            damage: 0,
            dissipation: 0,
            range: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn set(&mut self, mut simpleWeaponProperties: Option<Rc<RefCell<dyn SimpleWeaponPropertiesDyn>>>) {
        { let java_arguments = (jcall!(simpleWeaponProperties).getDamage(),); self.setDamage(java_arguments.0) };
        { let java_arguments = (jcall!(simpleWeaponProperties).getDissipation(),); self.setDissipation(java_arguments.0) };
        { let java_arguments = (jcall!(simpleWeaponProperties).getRange(),); self.setRange(java_arguments.0) };
    }
    

    pub fn setDamage(&mut self, mut damage: i32) {
        self.damage = damage.clone();
    }
    

    pub fn getDamage(&mut self) -> i32 {
        return self.damage.clone();
    }
    

    pub fn setDissipation(&mut self, mut dissipation: i16) {
        self.dissipation = dissipation.clone();
    }
    

    pub fn getDissipation(&mut self) -> i16 {
        return self.dissipation.clone();
    }
    

    pub fn getRange(&mut self) -> i32 {
        return self.range.clone();
    }
    

    pub fn setRange(&mut self, mut range: i32) {
        self.range = range.clone();
    }
}
pub trait SimpleWeaponPropertiesDyn {
    fn as_SimpleWeaponProperties(&self) -> &SimpleWeaponProperties;
    fn as_SimpleWeaponProperties_mut(&mut self) -> &mut SimpleWeaponProperties;
    fn set(&mut self, mut simpleWeaponProperties: Option<Rc<RefCell<dyn SimpleWeaponPropertiesDyn>>>) { SimpleWeaponProperties::set(self.as_SimpleWeaponProperties_mut(), simpleWeaponProperties) }
    fn setDamage(&mut self, mut damage: i32) { SimpleWeaponProperties::setDamage(self.as_SimpleWeaponProperties_mut(), damage) }
    fn getDamage(&mut self) -> i32 { SimpleWeaponProperties::getDamage(self.as_SimpleWeaponProperties_mut()) }
    fn setDissipation(&mut self, mut dissipation: i16) { SimpleWeaponProperties::setDissipation(self.as_SimpleWeaponProperties_mut(), dissipation) }
    fn getDissipation(&mut self) -> i16 { SimpleWeaponProperties::getDissipation(self.as_SimpleWeaponProperties_mut()) }
    fn getRange(&mut self) -> i32 { SimpleWeaponProperties::getRange(self.as_SimpleWeaponProperties_mut()) }
    fn setRange(&mut self, mut range: i32) { SimpleWeaponProperties::setRange(self.as_SimpleWeaponProperties_mut(), range) }
}
impl SimpleWeaponPropertiesDyn for SimpleWeaponProperties {
    fn as_SimpleWeaponProperties(&self) -> &SimpleWeaponProperties { self }
    fn as_SimpleWeaponProperties_mut(&mut self) -> &mut SimpleWeaponProperties { self }
}


