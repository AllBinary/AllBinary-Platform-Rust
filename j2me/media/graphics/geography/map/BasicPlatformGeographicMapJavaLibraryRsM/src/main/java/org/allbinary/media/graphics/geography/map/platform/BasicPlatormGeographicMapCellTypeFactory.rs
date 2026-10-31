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
// package org.allbinary.media.graphics.geography.map.platform
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Enumeration;
use super::*;
// use crate::java::util::Enumeration::Enumeration;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellType;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::EnumerationUtil::EnumerationUtil;
use super::BasicPlatormGeographicMapCellType::BasicPlatormGeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicPlatormGeographicMapCellTypeFactory {
    pub base: GeographicMapCellTypeFactory,
    pub enumerationUtil: Option<Rc<RefCell<EnumerationUtil>>>,
    pub BLOCK_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>>,
    pub JUMP_THRU_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>>,
    pub LADDER_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>>,
    pub maxTileId: i32,
}


impl BasicPlatormGeographicMapCellTypeFactory {
    

    pub fn new_struct(mut tileTypeToTileIdsMap: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>, mut maxTileId: i32) -> Self {
        let mut this = Self {
            base: GeographicMapCellTypeFactory::new_struct(),
            enumerationUtil: None,
            BLOCK_CELL_TYPE: None,
            JUMP_THRU_CELL_TYPE: None,
            LADDER_CELL_TYPE: None,
            maxTileId: 0,
        };
        this.enumerationUtil = EnumerationUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(this.logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
        this.maxTileId = maxTileId.clone();
        GeographicMapCellType::new(0, 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
        let mut BLOCK_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> = BasicPlatormGeographicMapCellType::createType(1);
        let mut JUMP_THRU_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut LADDER_CELL_TYPE: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> = BLOCK_CELL_TYPE.clone();
        let mut PLATFORM: String = String::from("Platform");
        let mut JUMP_TRHU: String = String::from("JumpThru");
        let mut LADDER: String = String::from("Ladder");
        let mut enumeration: Option<Rc<RefCell<dyn Enumeration>>> = jcall!(tileTypeToTileIdsMap).keys();
        let mut idsWithTypeList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = None;
        let mut key: String = String::new();
        let mut basicPlatormGeographicMapCellType: Option<Rc<RefCell<BasicPlatormGeographicMapCellType>>> = None;
        while jcall!(this.enumerationUtil).hasMoreElements(enumeration.clone()) {
            key = (jcall!(this.enumerationUtil).nextElement(enumeration.clone())).clone() /* as String */;
            jcall!(this.logUtil).putF(key.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone());
            idsWithTypeList = { let java_object = (jcall!(tileTypeToTileIdsMap).get(crate::java_into_object(key.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            basicPlatormGeographicMapCellType = BasicPlatormGeographicMapCellType::new(idsWithTypeList.clone());
            if (key == PLATFORM.clone()) {
                BLOCK_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            }
            if (key == JUMP_TRHU.clone()) {
                JUMP_THRU_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            }
            if (key == LADDER.clone()) {
                LADDER_CELL_TYPE = basicPlatormGeographicMapCellType.clone();
            }
        }
        this.BLOCK_CELL_TYPE = BLOCK_CELL_TYPE.clone();
        this.JUMP_THRU_CELL_TYPE = JUMP_THRU_CELL_TYPE.clone();
        this.LADDER_CELL_TYPE = LADDER_CELL_TYPE.clone();
        GeographicMapCellType::new(this.maxTileId - 1, 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
        GeographicMapCellType::new(this.maxTileId - 2, 0).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>);
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
        return 0;
    }
    

    pub fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool {
        if jcall!(cellType).getType() == 0 {
            return true;
        }
        return false;
    }
}
impl std::ops::Deref for BasicPlatormGeographicMapCellTypeFactory {
    type Target = GeographicMapCellTypeFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicPlatormGeographicMapCellTypeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn for BasicPlatormGeographicMapCellTypeFactory {
    fn as_GeographicMapCellTypeFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &self.base }
    fn as_GeographicMapCellTypeFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory { &mut self.base }
    fn getStartType(&mut self) -> i32 { Self::getStartType(self) }
    fn getEndType(&mut self) -> i32 { Self::getEndType(self) }
    fn getEmptyType(&mut self) -> i32 { Self::getEmptyType(self) }
    fn isPath(&mut self, mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>) -> bool { Self::isPath(self, cellType) }
}


