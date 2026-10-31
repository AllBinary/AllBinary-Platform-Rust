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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use super::RaceTrackGeographicMapInterface::RaceTrackGeographicMapInterface;
use super::RaceTrackInfo::RaceTrackInfo;
use super::RaceTrackData::RaceTrackData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRaceTrackGeographicMap {
}

thread_local! { pub static NullRaceTrackGeographicMap_NULL_RACE_TRACK_GEOGRAPHIC_MAP: Option<Rc<RefCell<NullRaceTrackGeographicMap>>> = NullRaceTrackGeographicMap::new(); }
impl RaceTrackGeographicMapInterface for NullRaceTrackGeographicMap {
    fn getRaceTrackInfo(&mut self) -> Option<Rc<RefCell<RaceTrackInfo>>> { Self::getRaceTrackInfo(self) }
    fn setRaceTrackInfo(&mut self, mut aRaceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>) { Self::setRaceTrackInfo(self, aRaceTrackInfo) }
    fn getRaceTrackData(&mut self) -> Option<Rc<RefCell<RaceTrackData>>> { Self::getRaceTrackData(self) }
    fn setRaceTrackData(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) { Self::setRaceTrackData(self, raceTrackData) }
}
impl crate::org::allbinary::media::graphics::geography::map::GeographicMapInterface::GeographicMapInterface for NullRaceTrackGeographicMap {
    fn getId(&mut self) -> Option<i32> { Self::getId(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn reset(&mut self) { Self::reset(self) }
    fn getAllBinaryTiledLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> { Self::getAllBinaryTiledLayer(self) }
    fn getCellPositionsAt(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>, mut currentCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>, mut cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>) -> bool { Self::getCellPositionsAt(self, layer, currentCellPositionArray, cellPositionArray) }
    fn getCellPositionAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getCellPositionAtXY(self, x, y) }
    fn getCellPositionAtXYNoThrow(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getCellPositionAtXYNoThrow(self, x, y) }
    fn getCellTypeFromMapCellTypeInt(&mut self, mut cellTypeId: i32) -> i32 { Self::getCellTypeFromMapCellTypeInt(self, cellTypeId) }
    fn getCellTypeAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> { Self::getCellTypeAtXY(self, x, y) }
    fn getCellTypeAt(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> { Self::getCellTypeAt(self, geographicMapCellPosition) }
    fn getGeographicMapCellPositionFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> { Self::getGeographicMapCellPositionFactory(self) }
    fn getGeographicMapCellPositionFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>> { Self::getGeographicMapCellPositionFactoryInterface(self) }
    fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getForegroundBasicColor(self) }
    fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getBackgroundBasicColor(self) }
}

impl NullRaceTrackGeographicMap {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getId(&mut self) -> Option<i32> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getName(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn reset(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getAllBinaryTiledLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCellPositionsAt(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>, mut currentCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>, mut cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCellPositionAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCellPositionAtXYNoThrow(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCellTypeFromMapCellTypeInt(&mut self, mut cellTypeId: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCellTypeAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCellTypeAt(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeographicMapCellPositionFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeographicMapCellPositionFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRaceTrackInfo(&mut self) -> Option<Rc<RefCell<RaceTrackInfo>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setRaceTrackInfo(&mut self, mut aRaceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRaceTrackData(&mut self) -> Option<Rc<RefCell<RaceTrackData>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setRaceTrackData(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

