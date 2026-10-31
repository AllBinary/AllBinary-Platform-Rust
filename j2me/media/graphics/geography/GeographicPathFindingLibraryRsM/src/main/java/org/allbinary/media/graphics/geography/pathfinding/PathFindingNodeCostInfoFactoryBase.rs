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
// package org.allbinary.media.graphics.geography.pathfinding
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use super::PathFindingNodeCostInfoFactoryBaseInterface::PathFindingNodeCostInfoFactoryBaseInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PathFindingNodeCostInfoFactoryBase {
}

thread_local! { pub static PathFindingNodeCostInfoFactoryBase_NULL_PATH_FINDING_NODE_COST_INFO_FACTORY_BASE: Option<Rc<RefCell<dyn PathFindingNodeCostInfoFactoryBaseDyn>>> = PathFindingNodeCostInfoFactoryBase::new().map(|reference| reference as Rc<RefCell<dyn PathFindingNodeCostInfoFactoryBaseDyn>>); }
impl PathFindingNodeCostInfoFactoryBaseInterface for PathFindingNodeCostInfoFactoryBase {
    fn create(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut costFromStart: i64, mut costToEnd: i64) { Self::create(self, geographicMapInterface, comingFromGeographicMapCellPosition, geographicMapCellPosition, costFromStart, costToEnd) }
    fn getTotalCost(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i64 { Self::getTotalCost(self, geographicMapInterface, comingFromGeographicMapCellPosition, geographicMapCellPosition) }
}

impl PathFindingNodeCostInfoFactoryBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn create(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut costFromStart: i64, mut costToEnd: i64) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getTotalCost(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut comingFromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i64 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait PathFindingNodeCostInfoFactoryBaseDyn: crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingNodeCostInfoFactoryBaseInterface::PathFindingNodeCostInfoFactoryBaseInterface {
    fn as_PathFindingNodeCostInfoFactoryBase(&self) -> &PathFindingNodeCostInfoFactoryBase;
    fn as_PathFindingNodeCostInfoFactoryBase_mut(&mut self) -> &mut PathFindingNodeCostInfoFactoryBase;
}
impl PathFindingNodeCostInfoFactoryBaseDyn for PathFindingNodeCostInfoFactoryBase {
    fn as_PathFindingNodeCostInfoFactoryBase(&self) -> &PathFindingNodeCostInfoFactoryBase { self }
    fn as_PathFindingNodeCostInfoFactoryBase_mut(&mut self) -> &mut PathFindingNodeCostInfoFactoryBase { self }
}


