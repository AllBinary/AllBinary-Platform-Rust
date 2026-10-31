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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::RaceTrackGeographicMapInterface::RaceTrackGeographicMapInterface;
use super::RaceTrackInfo::RaceTrackInfo;
use super::RaceTrackData::RaceTrackData;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseRaceTrackGeographicMap {
    pub base: BasicGeographicMap,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>,
    pub raceTrackData: Option<Rc<RefCell<RaceTrackData>>>,
    pub miniGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>,
}

// impl RaceTrackGeographicMapInterface for BaseRaceTrackGeographicMap with the methods of the trait

impl BaseRaceTrackGeographicMap {
    

    pub fn new_struct(mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut miniGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BasicGeographicMap::new_struct(jcall!(raceTrackInfo).getId(), jcall!(raceTrackInfo).getName(), jcall!(raceTrackData).getCellTypeIdToGeographicMapCellTypeArray(), tiledLayer.clone(), jcall!(raceTrackInfo).getForegroundBasicColor(), jcall!(raceTrackInfo).getBackgroundBasicColor(), geographicMapCellPositionFactoryInterface.clone(), geographicMapCellPositionBaseFactory.clone(), geographicMapCellTypeFactory.clone()),
            logUtil: None,
            raceTrackInfo: None,
            raceTrackData: None,
            miniGeographicMap: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.raceTrackInfo = raceTrackInfo.clone();
        this.raceTrackData = raceTrackData.clone();
        this.miniGeographicMap = miniGeographicMap.clone();
        let mut miniTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(this.miniGeographicMap).getAllBinaryTiledLayer();
        if jcall!(miniTiledLayer).getColumns() != jcall!(this.getGeographicMapCellPositionFactory()).getColumns() {
            let mut error: String = jcall!(StringMaker::new()).append(String::from("RaceTrackMap has incorrect Mini Map columns: ")).appendint(jcall!(miniTiledLayer).getColumns()).append(String::from(" != ")).appendint(((jcall!(this.getGeographicMapCellPositionFactory()).getColumns()) as i32)).toString();
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
        }
        if jcall!(miniTiledLayer).getRows() != jcall!(this.getGeographicMapCellPositionFactory()).getRows() {
            let mut error: String = String::from("RaceTrackMap has incorrect Mini Map rows");
            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(this.logUtil).putF(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
            panic!("{:?}", format!("{}: {:?}", "Exception", (error.clone())));
        }
        this
    }

    pub fn new(mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut miniGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(raceTrackInfo, raceTrackData, tiledLayer, miniGeographicMap, geographicMapCellPositionFactoryInterface, geographicMapCellPositionBaseFactory, geographicMapCellTypeFactory))))
    }
    

    pub fn getRaceTrackInfo(&mut self) -> Option<Rc<RefCell<RaceTrackInfo>>> {
        return self.raceTrackInfo.clone();
    }
    

    pub fn setRaceTrackInfo(&mut self, mut aRaceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>) {
        self.raceTrackInfo = aRaceTrackInfo.clone();
    }
    

    pub fn setMiniBasicGeographicMap(&mut self, mut miniBasicGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) {
        self.miniGeographicMap = miniBasicGeographicMap.clone();
    }
    

    pub fn getMiniBasicGeographicMap(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> {
        return self.miniGeographicMap.clone();
    }
    

    pub fn getRaceTrackData(&mut self) -> Option<Rc<RefCell<RaceTrackData>>> {
        return self.raceTrackData.clone();
    }
    

    pub fn setRaceTrackData(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) {
        self.raceTrackData = raceTrackData.clone();
    }
}
impl std::ops::Deref for BaseRaceTrackGeographicMap {
    type Target = BasicGeographicMap;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BaseRaceTrackGeographicMap {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BaseRaceTrackGeographicMapDyn: crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn {
    fn as_BaseRaceTrackGeographicMap(&self) -> &BaseRaceTrackGeographicMap;
    fn as_BaseRaceTrackGeographicMap_mut(&mut self) -> &mut BaseRaceTrackGeographicMap;
    fn getRaceTrackInfo(&mut self) -> Option<Rc<RefCell<RaceTrackInfo>>> { BaseRaceTrackGeographicMap::getRaceTrackInfo(self.as_BaseRaceTrackGeographicMap_mut()) }
    fn setRaceTrackInfo(&mut self, mut aRaceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>) { BaseRaceTrackGeographicMap::setRaceTrackInfo(self.as_BaseRaceTrackGeographicMap_mut(), aRaceTrackInfo) }
    fn setMiniBasicGeographicMap(&mut self, mut miniBasicGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) { BaseRaceTrackGeographicMap::setMiniBasicGeographicMap(self.as_BaseRaceTrackGeographicMap_mut(), miniBasicGeographicMap) }
    fn getMiniBasicGeographicMap(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> { BaseRaceTrackGeographicMap::getMiniBasicGeographicMap(self.as_BaseRaceTrackGeographicMap_mut()) }
    fn getRaceTrackData(&mut self) -> Option<Rc<RefCell<RaceTrackData>>> { BaseRaceTrackGeographicMap::getRaceTrackData(self.as_BaseRaceTrackGeographicMap_mut()) }
    fn setRaceTrackData(&mut self, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>) { BaseRaceTrackGeographicMap::setRaceTrackData(self.as_BaseRaceTrackGeographicMap_mut(), raceTrackData) }
}
impl BaseRaceTrackGeographicMapDyn for BaseRaceTrackGeographicMap {
    fn as_BaseRaceTrackGeographicMap(&self) -> &BaseRaceTrackGeographicMap { self }
    fn as_BaseRaceTrackGeographicMap_mut(&mut self) -> &mut BaseRaceTrackGeographicMap { self }
}

impl crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn for BaseRaceTrackGeographicMap {
    fn as_BasicGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap { &self.base }
    fn as_BasicGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMapDyn for BaseRaceTrackGeographicMap {
    fn as_SimpleGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &self.base.base }
    fn as_SimpleGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &mut self.base.base }
}


