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
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::RaceTrackInfo::RaceTrackInfo;
use super::RaceTrackData::RaceTrackData;
use super::AllBinaryTiledLayerFactoryInterface::AllBinaryTiledLayerFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGeographicMapFactory {
}


impl BasicGeographicMapFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>, mut tiledLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryTiledLayerFactoryInterface>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> {
        return BasicGeographicMap::new(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(((jcall!(raceTrackInfo).getId()).unwrap() as i32) + 100 + 1), jcall!(raceTrackInfo).getName(), jcall!(raceTrackData).getCellTypeIdToGeographicMapCellTypeArray(), jcall!(tiledLayerFactoryInterface).getMiniInstance(raceTrackData.clone()), jcall!(BasicColorFactory::getInstance()).CLEAR_COLOR.clone(), jcall!(BasicColorFactory::getInstance()).CLEAR_COLOR.clone(), geographicMapCellPositionFactoryInterface.clone(), GeographicMapCellPositionBaseFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>), geographicMapCellTypeFactory.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>);
    }
}

