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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap;
use super::CustomMapGeneratorBase::CustomMapGeneratorBase;
use super::BasicGeographicMapFactory::BasicGeographicMapFactory;
use super::RaceTrackInfo::RaceTrackInfo;
use super::RaceTrackData::RaceTrackData;
use super::AllBinaryTiledLayerFactoryInterface::AllBinaryTiledLayerFactoryInterface;
use super::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackGeographicMap {
    pub base: BaseRaceTrackGeographicMap,
    pub customMapGenerator: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>>,
}


impl RaceTrackGeographicMap {
    

    pub fn new_struct(mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>, mut tiledLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryTiledLayerFactoryInterface>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>, mut customMapGeneratorBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseRaceTrackGeographicMap::new_struct(raceTrackInfo.clone(), raceTrackData.clone(), jcall!(tiledLayerFactoryInterface).getInstance(raceTrackInfo.clone(), raceTrackData.clone()), jcall!(BasicGeographicMapFactory::new()).getInstance(raceTrackInfo.clone(), raceTrackData.clone(), tiledLayerFactoryInterface.clone(), geographicMapCellPositionFactoryInterface.clone(), geographicMapCellTypeFactory.clone()), geographicMapCellPositionFactoryInterface.clone(), geographicMapCellPositionBaseFactory.clone(), geographicMapCellTypeFactory.clone()),
            customMapGenerator: None,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut raceTrackInfo: Option<Rc<RefCell<RaceTrackInfo>>>, mut raceTrackData: Option<Rc<RefCell<RaceTrackData>>>, mut tiledLayerFactoryInterface: Option<Rc<RefCell<dyn AllBinaryTiledLayerFactoryInterface>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>, mut customMapGeneratorBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBaseFactory::CustomMapGeneratorBaseFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(raceTrackInfo, raceTrackData, tiledLayerFactoryInterface, geographicMapCellPositionFactoryInterface, geographicMapCellPositionBaseFactory, geographicMapCellTypeFactory, customMapGeneratorBaseFactory))));
            this.customMapGenerator = jcall!(customMapGeneratorBaseFactory).create((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getCustomMapGenerator(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::CustomMapGeneratorBase::CustomMapGeneratorBaseDyn>>> {
        return self.customMapGenerator.clone();
    }
    

    // throws Exception
    pub fn isValid(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut customMapArray: Vec<Vec<i32>> = jcall!(self.customMapGenerator).getCustomMapArray();
        if jcall!(geographicMapCellPosition).getColumn() >= (customMapArray[(0) as usize].len() as i32) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Column: "));
            jcall!(stringBuffer).appendint(jcall!(geographicMapCellPosition).getColumn());
            jcall!(stringBuffer).append(String::from(" not in: "));
            jcall!(stringBuffer).appendint((customMapArray[(0) as usize].len() as i32).clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if jcall!(geographicMapCellPosition).getColumn() == (customMapArray[(0) as usize].len() as i32) {
                return true;
            } else {
                return false;
            }
        } else if jcall!(geographicMapCellPosition).getRow() > (customMapArray.len() as i32) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(String::from("Row: "));
            jcall!(stringBuffer).appendint(jcall!(geographicMapCellPosition).getRow());
            jcall!(stringBuffer).append(String::from(" not in: "));
            jcall!(stringBuffer).appendint((customMapArray.len() as i32).clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            if jcall!(geographicMapCellPosition).getRow() == (customMapArray.len() as i32) {
                return true;
            } else {
                return false;
            }
        }
        return true;
    }
}
impl std::ops::Deref for RaceTrackGeographicMap {
    type Target = BaseRaceTrackGeographicMap;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RaceTrackGeographicMap {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn for RaceTrackGeographicMap {
    fn as_BaseRaceTrackGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap { &self.base }
    fn as_BaseRaceTrackGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap { &mut self.base }
}
impl crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn for RaceTrackGeographicMap {
    fn as_BasicGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap { &self.base.base }
    fn as_BasicGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap { &mut self.base.base }
}
impl crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMapDyn for RaceTrackGeographicMap {
    fn as_SimpleGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &self.base.base.base }
    fn as_SimpleGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &mut self.base.base.base }
}


