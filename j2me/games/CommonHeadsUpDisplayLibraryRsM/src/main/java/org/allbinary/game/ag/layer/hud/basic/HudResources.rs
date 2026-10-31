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
// package org.allbinary.game.ag.layer.hud.basic
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

pub struct HudResources {
    pub RESOURCE_LIFE: String,
    pub RESOURCE_HEALTH: String,
}

thread_local! { pub static HudResources_instance: Option<Rc<RefCell<HudResources>>> = HudResources::new(); }

impl HudResources {
    

    pub fn getInstance() -> Option<Rc<RefCell<HudResources>>> {
        return HudResources_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            RESOURCE_LIFE: String::new(),
            RESOURCE_HEALTH: String::new(),
        };
        this.RESOURCE_LIFE = String::from("/hud_life_10_by_10.png");
        this.RESOURCE_HEALTH = String::from("/hud_health_10_by_10.png");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

