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
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::topview::BasicTopViewGeographicMapCellTypeFactory::BasicTopViewGeographicMapCellTypeFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TopViewCharacterBehavior {
}


impl TopViewCharacterBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) {
    }
    

    pub fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) {
    }
    

    pub fn hasSolidBlock(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool {
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        let mut basicTopViewGeographicMapCellTypeFactory: Option<Rc<RefCell<BasicTopViewGeographicMapCellTypeFactory>>> = None;
        for index in (0..size) {
            basicTopViewGeographicMapCellTypeFactory = (jcall!(geographicMapInterfaceArray[(index) as usize]).getGeographicMapCellTypeFactory()).clone() /* as BasicTopViewGeographicMapCellTypeFactory */;
            if jcall!(jcall!(basicTopViewGeographicMapCellTypeFactory).BLOCK_CELL_TYPE).isType(geographicMapCellTypeArray[(index) as usize].clone()) {
                return true;
            }
        }
        return false;
    }
}
pub trait TopViewCharacterBehaviorDyn {
    fn as_TopViewCharacterBehavior(&self) -> &TopViewCharacterBehavior;
    fn as_TopViewCharacterBehavior_mut(&mut self) -> &mut TopViewCharacterBehavior;
    fn terrainEvent(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut direction: Option<Rc<RefCell<Direction>>>, mut x: i32, mut y: i32, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<GeographicMapCellPosition>>>) { TopViewCharacterBehavior::terrainEvent(self.as_TopViewCharacterBehavior_mut(), layer, direction, x, y, geographicMapInterfaceArray, geographicMapCellPosition) }
    fn terrainMove(&mut self, mut layer: Option<Rc<RefCell<AllBinaryLayer>>>, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut x: i32, mut y: i32) { TopViewCharacterBehavior::terrainMove(self.as_TopViewCharacterBehavior_mut(), layer, geographicMapInterfaceArray, x, y) }
    fn hasSolidBlock(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<BasicGeographicMap>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<GeographicMapCellType>>>>) -> bool { TopViewCharacterBehavior::hasSolidBlock(self.as_TopViewCharacterBehavior_mut(), geographicMapInterfaceArray, geographicMapCellTypeArray) }
}
impl TopViewCharacterBehaviorDyn for TopViewCharacterBehavior {
    fn as_TopViewCharacterBehavior(&self) -> &TopViewCharacterBehavior { self }
    fn as_TopViewCharacterBehavior_mut(&mut self) -> &mut TopViewCharacterBehavior { self }
}


