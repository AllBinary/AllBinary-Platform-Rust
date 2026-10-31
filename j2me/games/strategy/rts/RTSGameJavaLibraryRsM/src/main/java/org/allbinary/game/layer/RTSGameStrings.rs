// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSGameStrings {
    pub DRAG_TO_SPOT: String,
    pub NOT_YOURS: String,
    pub NEW_UNIT: String,
    pub NEW_WAYPOINT: String,
    pub BUILDING: String,
    pub UPGRADE: String,
    pub DOWNGRADE: String,
    pub NO_MONEY: String,
    pub SPOT_TAKEN: String,
    pub BUILDING_COLLISION: String,
    pub STRUCTURE_TO_CLOSE: String,
    pub MAP_EDGE: String,
    pub ROAD_COLLISION: String,
    pub SELECT_BUILD_SPOT: String,
    pub BUILD_ON_PATH: String,
    pub DRAGGABLE: String,
}

thread_local! { pub static RTSGameStrings_instance: Option<Rc<RefCell<RTSGameStrings>>> = RTSGameStrings::new(); }

impl RTSGameStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            DRAG_TO_SPOT: String::new(),
            NOT_YOURS: String::new(),
            NEW_UNIT: String::new(),
            NEW_WAYPOINT: String::new(),
            BUILDING: String::new(),
            UPGRADE: String::new(),
            DOWNGRADE: String::new(),
            NO_MONEY: String::new(),
            SPOT_TAKEN: String::new(),
            BUILDING_COLLISION: String::new(),
            STRUCTURE_TO_CLOSE: String::new(),
            MAP_EDGE: String::new(),
            ROAD_COLLISION: String::new(),
            SELECT_BUILD_SPOT: String::new(),
            BUILD_ON_PATH: String::new(),
            DRAGGABLE: String::new(),
        };
        this.DRAG_TO_SPOT = String::from("Please Drag");
        this.NOT_YOURS = String::from("Not Yours");
        this.NEW_UNIT = String::from("New Unit");
        this.NEW_WAYPOINT = String::from("New Waypoint");
        this.BUILDING = String::from("Building");
        this.UPGRADE = String::from("Upgrading");
        this.DOWNGRADE = String::from("Downgrading");
        this.NO_MONEY = String::from("Not Enough Money");
        this.SPOT_TAKEN = String::from("Spot Taken");
        this.BUILDING_COLLISION = String::from("Building Collision");
        this.STRUCTURE_TO_CLOSE = String::from("Building To Close");
        this.MAP_EDGE = String::from("Map Edge");
        this.ROAD_COLLISION = String::from("Road Collision");
        this.SELECT_BUILD_SPOT = String::from("Select Build Spot");
        this.BUILD_ON_PATH = String::from("Not On Path");
        this.DRAGGABLE = String::from("Draggable");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RTSGameStrings>>> {
        return RTSGameStrings_instance.with(Clone::clone).clone();
    }
}

