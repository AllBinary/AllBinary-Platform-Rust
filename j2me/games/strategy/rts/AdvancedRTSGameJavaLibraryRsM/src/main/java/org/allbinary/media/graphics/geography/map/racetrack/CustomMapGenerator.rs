// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;
use super::CustomMapGeneratorBase::CustomMapGeneratorBase;
use super::RaceTrackGeographicMap::RaceTrackGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomMapGenerator {
    pub base: CustomMapGeneratorBase,
    pub customMapArray: Vec<Vec<i32>>,
    pub raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>,
    pub geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>,
    pub dropCellPositionHistory: Option<Rc<RefCell<DropCellPositionHistory>>>,
}


impl CustomMapGenerator {
    

    pub fn new_struct(mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Self {
        let mut this = Self {
            base: CustomMapGeneratorBase::new_struct(),
            customMapArray: Vec::new(),
            raceTrackGeographicMap: None,
            geographicMapCellTypeFactory: None,
            dropCellPositionHistory: None,
        };
        this.dropCellPositionHistory = DropCellPositionHistory::getInstance();
        this.raceTrackGeographicMap = raceTrackGeographicMap.clone();
        this.geographicMapCellTypeFactory = jcall!(this.raceTrackGeographicMap).getGeographicMapCellTypeFactory();
        let mut mapArray: Vec<Vec<i32>> = jcall!(jcall!(raceTrackGeographicMap).getRaceTrackData()).getMapArray();
        let mut size2: i32 = (mapArray[(0) as usize].len() as i32).clone();
        this.customMapArray = vec![vec![0; (size2) as usize]; ((mapArray.len() as i32)) as usize];
        if (mapArray.len() as i32) != (this.customMapArray.len() as i32) || (mapArray[(0) as usize].len() as i32) != (this.customMapArray[(0) as usize].len() as i32) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Array Incorrect"))));
        }
        this
    }

    pub fn new(mut raceTrackGeographicMap: Option<Rc<RefCell<RaceTrackGeographicMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(raceTrackGeographicMap))))
    }
    

    // throws Exception
    pub fn copyMapIntoCustomMap(&mut self) {
        let mut mapArray: Vec<Vec<i32>> = jcall!(jcall!(self.raceTrackGeographicMap).getRaceTrackData()).getMapArray();
        let mut startIndex2: i32 = (mapArray[(0) as usize].len() as i32) - 1;
        for index in (0..=(mapArray.len() as i32) - 1).rev() {
            for index2 in (0..=startIndex2).rev() {
                self.customMapArray[(index) as usize][(index2) as usize] = self.getCustomType(index2.clone(), index.clone(), mapArray[(index) as usize][(index2) as usize].clone());
            }
        }
    }
    

    // throws Exception
    pub fn getCustomType(&mut self, mut column: i32, mut row: i32, mut currentType: i32) -> i32 {
        let mut emptyType: i32 = jcall!(self.geographicMapCellTypeFactory).getEmptyType();
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(self.raceTrackGeographicMap).getGeographicMapCellPositionFactory();
        let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row.clone());
        if jcall!(self.dropCellPositionHistory).isCellPositionWithDrop(cellPosition.clone()) {
            let mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(self.dropCellPositionHistory).getLayerInterface(cellPosition.clone());
            if match (&layer, &crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer_NULL_ALLBINARY_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return emptyType.clone();
            }
            let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (layer).clone() /* as AdvancedRTSGameLayer */;
            if !(jcall!(rtsLayer).getType() == WaypointLayer::getStaticType()) {
                return emptyType.clone();
            }
        }
        return currentType.clone();
    }
    

    pub fn getCustomMapArray(&mut self) -> Vec<Vec<i32>> {
        return self.customMapArray.clone();
    }
}
impl std::ops::Deref for CustomMapGenerator {
    type Target = CustomMapGeneratorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CustomMapGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn for CustomMapGenerator {
    fn as_CustomMapGeneratorBase(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBase { &self.base }
    fn as_CustomMapGeneratorBase_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBase { &mut self.base }
    fn copyMapIntoCustomMap(&mut self) { Self::copyMapIntoCustomMap(self) }
    fn getCustomMapArray(&mut self) -> Vec<Vec<i32>> { Self::getCustomMapArray(self) }
    fn getCustomType(&mut self, mut column: i32, mut row: i32, mut currentType: i32) -> i32 { Self::getCustomType(self, column, row, currentType) }
}


