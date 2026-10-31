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
use super::RaceTrackGeographicMap::RaceTrackGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait RaceTrackGeographicMapInterfaceFactoryInterface {
    

    // throws Exception
    fn getLevelInstance(&mut self, id: Option<i32>) -> Option<Rc<RefCell<RaceTrackGeographicMap>>>;
    

    // throws Exception
    fn getTrackInstance(&mut self, id: Option<i32>) -> Option<Rc<RefCell<RaceTrackGeographicMap>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


