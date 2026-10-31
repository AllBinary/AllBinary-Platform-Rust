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
// package org.allbinary.game.behavior.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait TopViewCharacterInterface {
    

    fn terrainMove(&mut self, geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, x: i32, y: i32);
    

    // throws Exception
    fn terrainEvent(&mut self, x: i32, y: i32, geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>, geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>);
    

    fn r#move(&mut self);
    

    fn upp(&mut self);
    

    fn leftp(&mut self);
    

    fn rightp(&mut self);
    

    // throws Exception
    fn reset(&mut self);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


