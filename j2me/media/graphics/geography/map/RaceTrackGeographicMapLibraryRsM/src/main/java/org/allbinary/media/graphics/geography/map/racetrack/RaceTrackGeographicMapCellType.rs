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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackGeographicMapCellType {
    pub base: GeographicMapCellType,
    pub name: String,
    pub travelCost: i32,
}


impl RaceTrackGeographicMapCellType {
    

    pub fn new_struct(mut name: String, mut r#type: i32, mut travelCost: i32, mut reset: i32) -> Self {
        let mut this = Self {
            base: GeographicMapCellType::new_struct(r#type.clone(), reset.clone()),
            name: String::new(),
            travelCost: 0,
        };
        // super(...) is in the struct literal above
        this.name = name.clone();
        this.travelCost = travelCost.clone();
        this
    }

    pub fn new(mut name: String, mut r#type: i32, mut travelCost: i32, mut reset: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, r#type, travelCost, reset))))
    }
    

    pub fn getTravelCost(&mut self) -> i32 {
        return self.travelCost.clone();
    }
}
impl std::ops::Deref for RaceTrackGeographicMapCellType {
    type Target = GeographicMapCellType;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackGeographicMapCellType {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn for RaceTrackGeographicMapCellType {
    fn as_GeographicMapCellType(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType { &self.base }
    fn as_GeographicMapCellType_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType { &mut self.base }
}


