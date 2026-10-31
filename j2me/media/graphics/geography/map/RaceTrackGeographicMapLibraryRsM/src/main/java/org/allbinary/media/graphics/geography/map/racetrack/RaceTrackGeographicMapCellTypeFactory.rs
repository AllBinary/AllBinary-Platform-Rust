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
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use super::RaceTrackGeographicMapCellTypes::RaceTrackGeographicMapCellTypes;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackGeographicMapCellTypeFactory {
    pub base: GeographicMapCellTypeFactory,
    pub EMPTY_CELL_TYPE: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>,
    pub EASY_CELL_TYPE: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>,
    pub BOTTOM_LEFT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub TOP_LEFT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub TOP_RIGHT_TURN_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub VERTICAL_STRAIGHT_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub START_LINE_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub DEFAULT_FINISH_LINE_ROAD_CELL_TYPE: Option<Rc<RefCell<RaceTrackGeographicMapCellType>>>,
    pub FINISH_LINE_ROAD_CELL_TYPE: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>,
}

thread_local! { pub static RaceTrackGeographicMapCellTypeFactory_instanceC: Option<Rc<RefCell<dyn RaceTrackGeographicMapCellTypeFactoryDyn>>> = RaceTrackGeographicMapCellTypeFactory::new().map(|reference| reference as Rc<RefCell<dyn RaceTrackGeographicMapCellTypeFactoryDyn>>); }

impl RaceTrackGeographicMapCellTypeFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn RaceTrackGeographicMapCellTypeFactoryDyn>>> {
        return RaceTrackGeographicMapCellTypeFactory_instanceC.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: GeographicMapCellTypeFactory::new_struct(),
            EMPTY_CELL_TYPE: None,
            EASY_CELL_TYPE: None,
            BOTTOM_LEFT_TURN_ROAD_CELL_TYPE: None,
            BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE: None,
            TOP_LEFT_TURN_ROAD_CELL_TYPE: None,
            TOP_RIGHT_TURN_ROAD_CELL_TYPE: None,
            HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE: None,
            VERTICAL_STRAIGHT_ROAD_CELL_TYPE: None,
            START_LINE_ROAD_CELL_TYPE: None,
            DEFAULT_FINISH_LINE_ROAD_CELL_TYPE: None,
            FINISH_LINE_ROAD_CELL_TYPE: None,
        };
        this.EMPTY_CELL_TYPE = GeographicMapCellType::new(0, -1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
        this.EASY_CELL_TYPE = this.EMPTY_CELL_TYPE.clone();
        this.FINISH_LINE_ROAD_CELL_TYPE = crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE.with(Clone::clone).clone();
        let mut raceTrackGeographicMapCellTypes: Option<Rc<RefCell<RaceTrackGeographicMapCellTypes>>> = RaceTrackGeographicMapCellTypes::getInstance();
        this.BOTTOM_LEFT_TURN_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).BOTTOM_LEFT_TURN_ROAD_CELL_TYPE.clone();
        this.BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).BOTTOM_RIGHT_TURN_ROAD_CELL_TYPE.clone();
        this.TOP_LEFT_TURN_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).TOP_LEFT_TURN_ROAD_CELL_TYPE.clone();
        this.TOP_RIGHT_TURN_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).TOP_RIGHT_TURN_ROAD_CELL_TYPE.clone();
        this.HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).HORIZONTAL_STRAIGHT_ROAD_CELL_TYPE.clone();
        this.VERTICAL_STRAIGHT_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).VERTICAL_STRAIGHT_ROAD_CELL_TYPE.clone();
        this.START_LINE_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).START_LINE_ROAD_CELL_TYPE.clone();
        this.DEFAULT_FINISH_LINE_ROAD_CELL_TYPE = jcall!(raceTrackGeographicMapCellTypes).DEFAULT_FINISH_LINE_ROAD_CELL_TYPE.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getStartType(&mut self) -> i32 {
        return jcall!(self.START_LINE_ROAD_CELL_TYPE).getType();
    }
    

    pub fn getEndType(&mut self) -> i32 {
        return jcall!(self.FINISH_LINE_ROAD_CELL_TYPE).getType();
    }
    

    pub fn getEmptyType(&mut self) -> i32 {
        return jcall!(self.EMPTY_CELL_TYPE).getType();
    }
    

    pub fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool {
        if !match (&cellType, &self.EMPTY_CELL_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && !match (&cellType, &self.EASY_CELL_TYPE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for RaceTrackGeographicMapCellTypeFactory {
    type Target = GeographicMapCellTypeFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackGeographicMapCellTypeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RaceTrackGeographicMapCellTypeFactoryDyn: crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn {
    fn as_RaceTrackGeographicMapCellTypeFactory(&self) -> &RaceTrackGeographicMapCellTypeFactory;
    fn as_RaceTrackGeographicMapCellTypeFactory_mut(&mut self) -> &mut RaceTrackGeographicMapCellTypeFactory;
}
impl RaceTrackGeographicMapCellTypeFactoryDyn for RaceTrackGeographicMapCellTypeFactory {
    fn as_RaceTrackGeographicMapCellTypeFactory(&self) -> &RaceTrackGeographicMapCellTypeFactory { self }
    fn as_RaceTrackGeographicMapCellTypeFactory_mut(&mut self) -> &mut RaceTrackGeographicMapCellTypeFactory { self }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn for RaceTrackGeographicMapCellTypeFactory {
    fn as_GeographicMapCellTypeFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &self.base }
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &mut self.base }
    fn getStartType(&mut self) -> i32 { Self::getStartType(self) }
    fn getEndType(&mut self) -> i32 { Self::getEndType(self) }
    fn getEmptyType(&mut self) -> i32 { Self::getEmptyType(self) }
    fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool { Self::isPath(self, cellType) }
}


