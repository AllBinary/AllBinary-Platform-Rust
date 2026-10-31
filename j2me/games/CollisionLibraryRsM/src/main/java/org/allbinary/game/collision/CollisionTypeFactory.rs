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
// package org.allbinary.game.collision
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::CollisionType::CollisionType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CollisionTypeFactory {
    pub NONE: Option<Rc<RefCell<CollisionType>>>,
    pub DAMAGE: Option<Rc<RefCell<CollisionType>>>,
    pub PICKUP: Option<Rc<RefCell<CollisionType>>>,
    pub COLLISION: Option<Rc<RefCell<CollisionType>>>,
}

pub static mut CollisionTypeFactory_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<CollisionTypeFactory>>>> = std::cell::LazyCell::new(|| CollisionTypeFactory::new());

impl CollisionTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NONE: None,
            DAMAGE: None,
            PICKUP: None,
            COLLISION: None,
        };
        this.NONE = CollisionType::new(String::from("None"));
        this.DAMAGE = CollisionType::new(String::from("Damage"));
        this.PICKUP = CollisionType::new(String::from("Pickup"));
        this.COLLISION = CollisionType::new(String::from("Collision"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CollisionTypeFactory>>> {
        return unsafe { CollisionTypeFactory_SINGLETON.clone() }.clone();
    }
}

