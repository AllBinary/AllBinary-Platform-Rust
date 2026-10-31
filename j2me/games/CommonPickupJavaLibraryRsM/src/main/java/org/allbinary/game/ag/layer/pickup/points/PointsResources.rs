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
// package org.allbinary.game.ag.layer.pickup.points
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

pub struct PointsResources {
    pub RESOURCE: String,
    pub RESOURCE_2: String,
    pub RESOURCE_3: String,
    pub RESOURCE_4: String,
}

thread_local! { pub static PointsResources_SINGLETON: Option<Rc<RefCell<PointsResources>>> = PointsResources::new(); }

impl PointsResources {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            RESOURCE: String::new(),
            RESOURCE_2: String::new(),
            RESOURCE_3: String::new(),
            RESOURCE_4: String::new(),
        };
        this.RESOURCE = String::from("points_drop_20_by_20.png");
        this.RESOURCE_2 = String::from("points_2_drop_20_by_20.png");
        this.RESOURCE_3 = String::from("points_3_drop_20_by_20.png");
        this.RESOURCE_4 = String::from("points_4_drop_20_by_20.png");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PointsResources>>> {
        return PointsResources_SINGLETON.with(Clone::clone).clone();
    }
}

