// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.media.graphics.geography.map.topview
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

pub struct BasicTopViewGeographicMapStrings {
    pub DEFAULT: String,
    pub OTHER: String,
    pub WALL: String,
    pub OFF_MAP: String,
    pub FLOOR: String,
    pub DOOR: String,
    pub STAIRS_UP: String,
    pub STAIRS_DOWN: String,
}

thread_local! { pub static BasicTopViewGeographicMapStrings_instance: Option<Rc<RefCell<BasicTopViewGeographicMapStrings>>> = BasicTopViewGeographicMapStrings::new(); }

impl BasicTopViewGeographicMapStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            DEFAULT: String::new(),
            OTHER: String::new(),
            WALL: String::new(),
            OFF_MAP: String::new(),
            FLOOR: String::new(),
            DOOR: String::new(),
            STAIRS_UP: String::new(),
            STAIRS_DOWN: String::new(),
        };
        this.DEFAULT = String::from("Default");
        this.OTHER = String::from("Other");
        this.WALL = String::from("Wall");
        this.OFF_MAP = String::from("OffMap");
        this.FLOOR = String::from("Floor");
        this.DOOR = String::from("Door");
        this.STAIRS_UP = String::from("StairsUp");
        this.STAIRS_DOWN = String::from("StairsDown");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicTopViewGeographicMapStrings>>> {
        return BasicTopViewGeographicMapStrings_instance.with(Clone::clone).clone();
    }
}

