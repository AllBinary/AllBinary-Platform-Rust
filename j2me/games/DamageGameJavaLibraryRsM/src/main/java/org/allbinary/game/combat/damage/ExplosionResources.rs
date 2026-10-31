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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ExplosionResources {
    pub EXPLOSION_90_RESOURCE: String,
    pub EXPLOSION_60_RESOURCE: String,
    pub HALF_EXPLOSION_RESOURCE: String,
    pub THIRD_EXPLOSION_RESOURCE: String,
}

thread_local! { pub static ExplosionResources_instance: Option<Rc<RefCell<ExplosionResources>>> = ExplosionResources::new(); }

impl ExplosionResources {
    

    pub fn getInstance() -> Option<Rc<RefCell<ExplosionResources>>> {
        return ExplosionResources_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            EXPLOSION_90_RESOURCE: String::new(),
            EXPLOSION_60_RESOURCE: String::new(),
            HALF_EXPLOSION_RESOURCE: String::new(),
            THIRD_EXPLOSION_RESOURCE: String::new(),
        };
        this.EXPLOSION_90_RESOURCE = String::from("/explosion_sprite_90_by_90.png");
        this.EXPLOSION_60_RESOURCE = String::from("/explosion_sprite_60_by_60.png");
        this.HALF_EXPLOSION_RESOURCE = String::from("/explosion_sprite_30_by_30.png");
        this.THIRD_EXPLOSION_RESOURCE = String::from("/explosion_sprite_15_by_15.png");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

