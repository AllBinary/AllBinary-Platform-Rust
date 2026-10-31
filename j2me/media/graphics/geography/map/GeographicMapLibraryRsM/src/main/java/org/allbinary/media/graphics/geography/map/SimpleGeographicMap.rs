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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::GeographicMapCellType::GeographicMapCellType;
use super::GeographicMapCellPosition::GeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimpleGeographicMap {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub id: Option<i32>,
    pub name: String,
    pub cellTypeIdToGeographicMapCellType: Vec<i32>,
    pub tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>,
    pub foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>,
    pub geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>,
}


impl SimpleGeographicMap {
    

    pub fn new_struct(mut id: Option<i32>, mut name: String, mut cellTypeIdToGeographicMapCellType: Vec<i32>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            commonStrings: None,
            id: None,
            name: String::new(),
            cellTypeIdToGeographicMapCellType: Vec::new(),
            tiledLayer: None,
            foregroundBasicColor: None,
            backgroundBasicColor: None,
            geographicMapCellTypeFactory: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.foregroundBasicColor = jcall!(BasicColorFactory::getInstance()).NULL_COLOR.clone();
        this.backgroundBasicColor = jcall!(BasicColorFactory::getInstance()).NULL_COLOR.clone();
        this.geographicMapCellTypeFactory = GeographicMapCellTypeFactory::getInstance();
        this.id = id.clone();
        this.name = name.clone();
        this.cellTypeIdToGeographicMapCellType = cellTypeIdToGeographicMapCellType.clone();
        this.tiledLayer = tiledLayer.clone();
        this.setBackgroundBasicColor(backgroundBasicColor.clone());
        this.setForegroundBasicColor(foregroundBasicColor.clone());
        this
    }

    pub fn new(mut id: Option<i32>, mut name: String, mut cellTypeIdToGeographicMapCellType: Vec<i32>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, name, cellTypeIdToGeographicMapCellType, tiledLayer, foregroundBasicColor, backgroundBasicColor))))
    }
    

    pub fn getId(&mut self) -> Option<i32> {
        return self.id.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn reset(&mut self) {
    }
    

    pub fn getAllBinaryTiledLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> {
        return self.tiledLayer.clone();
    }
    

    // throws Exception
    pub fn getCellTypeAt(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        let mut i_column: i32 = jcall!(cellPosition).getColumn();
        let mut i_row: i32 = jcall!(cellPosition).getRow();
        let mut cellTypeId: i32 = jcall!(self.tiledLayer).getCell(i_column.clone(), i_row.clone());
        if cellTypeId < 0 {
            cellTypeId = jcall!(self.tiledLayer).getAnimatedTile(cellTypeId.clone());
        }
        return jcall!(self.geographicMapCellTypeFactory).get(self.cellTypeIdToGeographicMapCellType[(cellTypeId) as usize].clone());
    }
    

    pub fn getCellTypeFromMapCellTypeInt(&mut self, mut cellTypeId: i32) -> i32 {
        return self.cellTypeIdToGeographicMapCellType[(cellTypeId) as usize].clone();
    }
    

    pub fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.foregroundBasicColor.clone();
    }
    

    pub fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.foregroundBasicColor = foregroundBasicColor.clone();
    }
    

    pub fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.backgroundBasicColor.clone();
    }
    

    pub fn setBackgroundBasicColor(&mut self, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.backgroundBasicColor = backgroundBasicColor.clone();
    }
}
pub trait SimpleGeographicMapDyn {
    fn as_SimpleGeographicMap(&self) -> &SimpleGeographicMap;
    fn as_SimpleGeographicMap_mut(&mut self) -> &mut SimpleGeographicMap;
    fn getId(&mut self) -> Option<i32> { SimpleGeographicMap::getId(self.as_SimpleGeographicMap_mut()) }
    fn getName(&mut self) -> String { SimpleGeographicMap::getName(self.as_SimpleGeographicMap_mut()) }
    fn reset(&mut self) { SimpleGeographicMap::reset(self.as_SimpleGeographicMap_mut()) }
    fn getAllBinaryTiledLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> { SimpleGeographicMap::getAllBinaryTiledLayer(self.as_SimpleGeographicMap_mut()) }
    fn getCellTypeAt(&mut self, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> { SimpleGeographicMap::getCellTypeAt(self.as_SimpleGeographicMap_mut(), cellPosition) }
    fn getCellTypeFromMapCellTypeInt(&mut self, mut cellTypeId: i32) -> i32 { SimpleGeographicMap::getCellTypeFromMapCellTypeInt(self.as_SimpleGeographicMap_mut(), cellTypeId) }
    fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { SimpleGeographicMap::getForegroundBasicColor(self.as_SimpleGeographicMap_mut()) }
    fn setForegroundBasicColor(&mut self, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>) { SimpleGeographicMap::setForegroundBasicColor(self.as_SimpleGeographicMap_mut(), foregroundBasicColor) }
    fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>> { SimpleGeographicMap::getBackgroundBasicColor(self.as_SimpleGeographicMap_mut()) }
    fn setBackgroundBasicColor(&mut self, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>) { SimpleGeographicMap::setBackgroundBasicColor(self.as_SimpleGeographicMap_mut(), backgroundBasicColor) }
}
impl SimpleGeographicMapDyn for SimpleGeographicMap {
    fn as_SimpleGeographicMap(&self) -> &SimpleGeographicMap { self }
    fn as_SimpleGeographicMap_mut(&mut self) -> &mut SimpleGeographicMap { self }
}


