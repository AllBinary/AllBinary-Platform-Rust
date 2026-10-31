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
// package org.allbinary.media.graphics.geography.map.topview
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::RaceTrackGeographicMapCellType::RaceTrackGeographicMapCellType;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::HashtableUtil::HashtableUtil;
use super::BasicTopViewGeographicMapCellType::BasicTopViewGeographicMapCellType;
use super::BasicTopViewGeographicMapStrings::BasicTopViewGeographicMapStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTopViewGeographicMapCellTypeFactory {
    pub base: GeographicMapCellTypeFactory,
    pub BLOCK_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub OFF_MAP_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub FLOOR_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub DOOR_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub STAIRS_UP_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub STAIRS_DOWN_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub OTHER_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>>,
    pub maxTileId: i32,
}


impl BasicTopViewGeographicMapCellTypeFactory {
    

    pub fn new_struct(mut tileTypeToTileIdsMap: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut maxTileId: i32) -> Self {
        let mut this = Self {
            base: GeographicMapCellTypeFactory::new_struct(),
            BLOCK_CELL_TYPE: None,
            OFF_MAP_CELL_TYPE: None,
            FLOOR_CELL_TYPE: None,
            DOOR_CELL_TYPE: None,
            STAIRS_UP_CELL_TYPE: None,
            STAIRS_DOWN_CELL_TYPE: None,
            OTHER_CELL_TYPE: None,
            maxTileId: 0,
        };
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).CONSTRUCTOR.clone());
        this.maxTileId = maxTileId.clone();
        let mut basicTopViewGeographicMapStrings: Option<Rc<RefCell<BasicTopViewGeographicMapStrings>>> = BasicTopViewGeographicMapStrings::getInstance();
        let mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> = GeographicMapCellTypeFactory::getInstance();
        let mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> = jcall!(geographicMapCellTypeFactory).getGeographicMapCellTypeArray();
        let mut r#type: i32 = 0;
        if geographicMapCellTypeArray[(r#type) as usize].is_none() {
            RaceTrackGeographicMapCellType::new(jcall!(CommonStrings::getInstance()).UNKNOWN.clone(), r#type.clone(), 999, 0);
        } else {
        }
        let mut BLOCK_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BasicTopViewGeographicMapCellType::createType(jcall!(basicTopViewGeographicMapStrings).DEFAULT.clone(), 1, 1);
        let mut OFF_MAP_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut FLOOR_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut DOOR_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut STAIRS_UP_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut STAIRS_DOWN_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut OTHER_CELL_TYPE: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(HashtableUtil::getInstance()).getKeysAsArray(tileTypeToTileIdsMap.clone());
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut idsWithTypeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut key: String = String::new();
        let mut basicPlatormGeographicMapCellType: Option<Rc<RefCell<BasicTopViewGeographicMapCellType>>> = None;
        for index in (0..size) {
            key = (keyArray[(index) as usize]).clone() /* as String */;
            idsWithTypeList = { let java_object = (jcall!(tileTypeToTileIdsMap).get(crate::java_into_object(key.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            if (key == jcall!(basicTopViewGeographicMapStrings).WALL.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).WALL.clone(), idsWithTypeList.clone(), 1000);
                BLOCK_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).OFF_MAP.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).OFF_MAP.clone(), idsWithTypeList.clone(), 1001);
                OFF_MAP_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).FLOOR.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).FLOOR.clone(), idsWithTypeList.clone(), 1);
                FLOOR_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).DOOR.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).DOOR.clone(), idsWithTypeList.clone(), 1);
                DOOR_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).STAIRS_UP.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).STAIRS_UP.clone(), idsWithTypeList.clone(), 1);
                STAIRS_UP_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).STAIRS_DOWN.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).STAIRS_DOWN.clone(), idsWithTypeList.clone(), 1);
                STAIRS_DOWN_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            } else if (key == jcall!(basicTopViewGeographicMapStrings).OTHER.clone()) {
                basicPlatormGeographicMapCellType = BasicTopViewGeographicMapCellType::new(jcall!(basicTopViewGeographicMapStrings).OTHER.clone(), idsWithTypeList.clone(), 1);
                OTHER_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            }
        }
        this.BLOCK_CELL_TYPE = BLOCK_CELL_TYPE.clone();
        this.OFF_MAP_CELL_TYPE = OFF_MAP_CELL_TYPE.clone();
        this.FLOOR_CELL_TYPE = FLOOR_CELL_TYPE.clone();
        this.DOOR_CELL_TYPE = DOOR_CELL_TYPE.clone();
        this.STAIRS_UP_CELL_TYPE = STAIRS_UP_CELL_TYPE.clone();
        this.STAIRS_DOWN_CELL_TYPE = STAIRS_DOWN_CELL_TYPE.clone();
        this.OTHER_CELL_TYPE = OTHER_CELL_TYPE.clone();
        r#type = this.maxTileId - 1;
        if geographicMapCellTypeArray[(r#type) as usize].is_none() {
            RaceTrackGeographicMapCellType::new(jcall!(commonStrings).START.clone(), r#type.clone(), 1, 0);
        } else {
        }
        r#type = this.maxTileId - 2;
        if geographicMapCellTypeArray[(r#type) as usize].is_none() {
            RaceTrackGeographicMapCellType::new(jcall!(commonStrings).START.clone(), r#type.clone(), 1, 0);
        } else {
        }
        this
    }

    pub fn new(mut tileTypeToTileIdsMap: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut maxTileId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(tileTypeToTileIdsMap, maxTileId))))
    }
    

    pub fn getStartType(&mut self) -> i32 {
        return self.maxTileId - 1;
    }
    

    pub fn getEndType(&mut self) -> i32 {
        return self.maxTileId - 2;
    }
    

    pub fn getEmptyType(&mut self) -> i32 {
        return jcall!(self.FLOOR_CELL_TYPE).getTypes()[(0) as usize].clone();
    }
    

    pub fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool {
        if jcall!(self.FLOOR_CELL_TYPE).isType(cellType.clone()) {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(String::from("key: WALL/BLOCK_CELL_TYPE: ")).append(jcall!(self.BLOCK_CELL_TYPE).toString()).append(String::from("key: FLOOR_CELL_TYPE: ")).append(jcall!(self.FLOOR_CELL_TYPE).toString()).append(String::from("key: OTHER_CELL_TYPE: ")).append(jcall!(self.OTHER_CELL_TYPE).toString()).append(String::from("key: OFF_MAP_CELL_TYPE: ")).append(jcall!(self.OFF_MAP_CELL_TYPE).toString()).append(String::from("key: DOOR_CELL_TYPE: ")).append(jcall!(self.DOOR_CELL_TYPE).toString()).append(String::from("key: STAIRS_DOWN_CELL_TYPE: ")).append(jcall!(self.STAIRS_DOWN_CELL_TYPE).toString()).append(String::from("key: STAIRS_UP_CELL_TYPE: ")).append(jcall!(self.STAIRS_UP_CELL_TYPE).toString()).toString();
    }
}
impl std::ops::Deref for BasicTopViewGeographicMapCellTypeFactory {
    type Target = GeographicMapCellTypeFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicTopViewGeographicMapCellTypeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn for BasicTopViewGeographicMapCellTypeFactory {
    fn as_GeographicMapCellTypeFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &self.base }
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &mut self.base }
    fn getStartType(&mut self) -> i32 { Self::getStartType(self) }
    fn getEndType(&mut self) -> i32 { Self::getEndType(self) }
    fn getEmptyType(&mut self) -> i32 { Self::getEmptyType(self) }
    fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool { Self::isPath(self, cellType) }
}


