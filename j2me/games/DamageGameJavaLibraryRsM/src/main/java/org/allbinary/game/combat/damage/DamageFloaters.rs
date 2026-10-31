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
// package org.allbinary.game.combat.damage
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DamageFloaters {
    pub base: Paintable,
}

thread_local! { pub static DamageFloaters_SINGLETON: Option<Rc<RefCell<dyn DamageFloatersDyn>>> = DamageFloaters::new().map(|reference| reference as Rc<RefCell<dyn DamageFloatersDyn>>); }

impl DamageFloaters {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn DamageFloatersDyn>>> {
        return DamageFloaters_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn add(&mut self, mut damage: i32) {
    }
}
impl std::ops::Deref for DamageFloaters {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DamageFloaters {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait DamageFloatersDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_DamageFloaters(&self) -> &DamageFloaters;
    fn as_DamageFloaters_mut(&mut self) -> &mut DamageFloaters;
    fn add(&mut self, mut damage: i32) { DamageFloaters::add(self.as_DamageFloaters_mut(), damage) }
}
impl DamageFloatersDyn for DamageFloaters {
    fn as_DamageFloaters(&self) -> &DamageFloaters { self }
    fn as_DamageFloaters_mut(&mut self) -> &mut DamageFloaters { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for DamageFloaters {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}


