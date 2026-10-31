// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::GeographicMapCellPositionAreaInterface::GeographicMapCellPositionAreaInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionAreaBase {
}

thread_local! { pub static GeographicMapCellPositionAreaBase_NULL_GEOGRPAHIC_MAP_POSITION_AREA_BASE: Option<Rc<RefCell<dyn GeographicMapCellPositionAreaBaseDyn>>> = GeographicMapCellPositionAreaBase::new().map(|reference| reference as Rc<RefCell<dyn GeographicMapCellPositionAreaBaseDyn>>); }
impl GeographicMapCellPositionAreaInterface for GeographicMapCellPositionAreaBase {
    fn getNextSurroundingGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getNextSurroundingGeographicMapCellPosition(self) }
    fn getOccupyingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getOccupyingGeographicMapCellPositionList(self) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getSurroundingGeographicMapCellPositionList(self) }
    fn update(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { Self::update(self, geographicMapInterface) }
}

impl GeographicMapCellPositionAreaBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getNextSurroundingGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getOccupyingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn update(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait GeographicMapCellPositionAreaBaseDyn: crate::org::allbinary::game::layer::GeographicMapCellPositionAreaInterface::GeographicMapCellPositionAreaInterface {
    fn as_GeographicMapCellPositionAreaBase(&self) -> &GeographicMapCellPositionAreaBase;
    fn as_GeographicMapCellPositionAreaBase_mut(&mut self) -> &mut GeographicMapCellPositionAreaBase;
}
impl GeographicMapCellPositionAreaBaseDyn for GeographicMapCellPositionAreaBase {
    fn as_GeographicMapCellPositionAreaBase(&self) -> &GeographicMapCellPositionAreaBase { self }
    fn as_GeographicMapCellPositionAreaBase_mut(&mut self) -> &mut GeographicMapCellPositionAreaBase { self }
}


