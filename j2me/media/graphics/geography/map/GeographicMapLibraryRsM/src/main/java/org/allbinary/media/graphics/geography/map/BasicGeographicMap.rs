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
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::SimpleGeographicMap::SimpleGeographicMap;
use super::GeographicMapInterface::GeographicMapInterface;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use super::GeographicMapCellTypeFactory::GeographicMapCellTypeFactory;
use super::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactory;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::GeographicMapCellType::GeographicMapCellType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGeographicMap {
    pub base: SimpleGeographicMap,
    pub mathUtil: Option<Rc<RefCell<MathUtil>>>,
    pub geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>>,
    pub geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>,
    pub geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>,
}

thread_local! { pub static BasicGeographicMap_NULL_BASIC_GEOGRAPHIC_MAP_ARRAY: Vec<Option<Rc<RefCell<dyn BasicGeographicMapDyn>>>> = vec![None; (0) as usize]; }
// impl GeographicMapInterface for BasicGeographicMap with the methods of the trait

impl BasicGeographicMap {
    

    pub fn new_struct(mut id: Option<i32>, mut name: String, mut cellTypeIdToGeographicMapCellType: Vec<i32>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: SimpleGeographicMap::new_struct(id.clone(), name.clone(), cellTypeIdToGeographicMapCellType.clone(), tiledLayer.clone(), foregroundBasicColor.clone(), backgroundBasicColor.clone()),
            mathUtil: None,
            geographicMapCellPositionFactory: None,
            geographicMapCellPositionFactoryInterface: None,
            geographicMapCellTypeFactory: None,
        };
        this.mathUtil = MathUtil::getInstance();
        // super(...) is in the struct literal above
        this.geographicMapCellPositionFactoryInterface = geographicMapCellPositionFactoryInterface.clone();
        this.geographicMapCellTypeFactory = geographicMapCellTypeFactory.clone();
        this
    }

    pub fn new(mut id: Option<i32>, mut name: String, mut cellTypeIdToGeographicMapCellType: Vec<i32>, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut foregroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>, mut geographicMapCellPositionBaseFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionBaseFactory::GeographicMapCellPositionBaseFactoryDyn>>>, mut geographicMapCellTypeFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(id, name, cellTypeIdToGeographicMapCellType, tiledLayer, foregroundBasicColor, backgroundBasicColor, geographicMapCellPositionFactoryInterface, geographicMapCellPositionBaseFactory, geographicMapCellTypeFactory))));
            this.geographicMapCellPositionFactory = jcall!(geographicMapCellPositionBaseFactory).getInstance((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // throws Exception
    pub fn getCellPosition(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        match direction {
            switch_value if switch_value == 0 => {
                return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn() - 1, jcall!(oldGeographicMapCellPosition).getRow(),); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
            }
            switch_value if switch_value == 1 => {
                return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn() + 1, jcall!(oldGeographicMapCellPosition).getRow(),); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
            }
            switch_value if switch_value == 2 => {
                return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn(), jcall!(oldGeographicMapCellPosition).getRow() - 1,); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
            }
            switch_value if switch_value == 3 => {
                return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn(), jcall!(oldGeographicMapCellPosition).getRow() + 1,); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
            }
            _ => {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Only Four Directions"))));
            }
        }
    }
    

    // throws Exception
    pub fn getCellPositionNoThrow(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        match direction {
            switch_value if switch_value == 0 => {
                if jcall!(oldGeographicMapCellPosition).getColumn() - 1 >= 0 {
                    return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn() - 1, jcall!(oldGeographicMapCellPosition).getRow(),); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
                } else {
                    return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                }
            }
            switch_value if switch_value == 1 => {
                if jcall!(oldGeographicMapCellPosition).getColumn() + 1 < jcall!(self.getAllBinaryTiledLayer()).getColumns() {
                    return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn() + 1, jcall!(oldGeographicMapCellPosition).getRow(),); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
                } else {
                    return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                }
            }
            switch_value if switch_value == 2 => {
                if jcall!(oldGeographicMapCellPosition).getRow() - 1 >= 0 {
                    return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn(), jcall!(oldGeographicMapCellPosition).getRow() - 1,); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
                } else {
                    return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                }
            }
            switch_value if switch_value == 3 => {
                if jcall!(oldGeographicMapCellPosition).getRow() + 1 < jcall!(self.getAllBinaryTiledLayer()).getRows() {
                    return { let java_receiver = self.geographicMapCellPositionFactory.clone(); let java_arguments = (jcall!(oldGeographicMapCellPosition).getColumn(), jcall!(oldGeographicMapCellPosition).getRow() + 1,); let java_result = jcall!(java_receiver).getAt(java_arguments.0, java_arguments.1); java_result };
                } else {
                    return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                }
            }
            _ => {
                return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
            }
        }
    }
    

    // throws Exception
    pub fn isOfFourDirections(&mut self, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut newGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        for index in (0..4) {
            if match (&newGeographicMapCellPosition, &self.getCellPositionNoThrow(index.clone(), oldGeographicMapCellPosition.clone())) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return true;
            }
        }
        return false;
    }
    

    // throws Exception
    pub fn getCellPositionAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.getAllBinaryTiledLayer();
        let mut i_column: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (x / jcall!(allBinaryTiledLayer).getCellHeight(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        let mut i_row: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (y / jcall!(allBinaryTiledLayer).getCellWidth(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        return jcall!(self.geographicMapCellPositionFactory).getAt(i_column.clone(), i_row.clone());
    }
    

    pub fn getCellPositionAtXYNoThrow(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.getAllBinaryTiledLayer();
        let mut i_column: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (x / jcall!(allBinaryTiledLayer).getCellHeight(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        let mut i_row: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (y / jcall!(allBinaryTiledLayer).getCellWidth(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        if jcall!(allBinaryTiledLayer).getColumns() > i_column && jcall!(allBinaryTiledLayer).getRows() > i_row {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                return jcall!(self.geographicMapCellPositionFactory).getAt(i_column.clone(), i_row.clone());
            })) {
                Ok(java_return) => return java_return,
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
                }
            }
        } else {
            return SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        }
    }
    

    // throws Exception
    pub fn isOnMap(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.getAllBinaryTiledLayer();
        let mut i_column: i32 = jcall!(geographicMapCellPosition).getColumn();
        let mut i_row: i32 = jcall!(geographicMapCellPosition).getRow();
        if jcall!(allBinaryTiledLayer).getColumns() > i_column && jcall!(allBinaryTiledLayer).getRows() > i_row {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn getCellPositionAtNoThrow(&mut self, mut x: i32, mut y: i32, mut x2: i32, mut y2: i32, mut geographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(geographicMapCellPositionList).clear();
        let mut allBinaryTiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.getAllBinaryTiledLayer();
        let mut i_columnMin: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (x / jcall!(allBinaryTiledLayer).getCellHeight(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        let mut i_rowMin: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (y / jcall!(allBinaryTiledLayer).getCellWidth(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result };
        let mut i_columnMax: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (x2 / jcall!(allBinaryTiledLayer).getCellHeight(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result } + 1;
        let mut i_rowMax: i32 = { let java_receiver = self.mathUtil.clone(); let java_arguments = (y2 / jcall!(allBinaryTiledLayer).getCellWidth(),); let java_result = jcall!(java_receiver).abs(java_arguments.0); java_result } + 1;
        for columnIndex in (i_columnMin..i_columnMax) {
            for rowIndex in (i_rowMin..i_rowMax) {
                if jcall!(allBinaryTiledLayer).getColumns() > columnIndex && jcall!(allBinaryTiledLayer).getRows() > rowIndex {
                    jcall!(geographicMapCellPositionList).add(crate::java_into_object(jcall!(self.geographicMapCellPositionFactory).getAt(columnIndex.clone(), rowIndex.clone())));
                }
            }
        }
        return geographicMapCellPositionList.clone();
    }
    

    // throws Exception
    pub fn getCellPositionsAt(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>, mut currentCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>, mut cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>) -> bool {
        let mut hasChanged: bool = false;
        let mut size: i32 = (cellPositionArray.len() as i32).clone();
        let mut size2: i32 = (cellPositionArray[(0) as usize].len() as i32).clone();
        let mut xPortion: i32 = jcall!(layer).getXP() / (size - 1);
        let mut yPortion: i32 = jcall!(layer).getYP() / (size - 1);
        for index in (0..size) {
            for index2 in (0..size2) {
                let mut x: i32 = xPortion * index;
                let mut y: i32 = yPortion * index;
                cellPositionArray[(index) as usize][(index2) as usize] = self.getCellPositionAtXY(x.clone(), y.clone());
                if !match (&currentCellPositionArray[(index) as usize][(index2) as usize], &cellPositionArray[(index) as usize][(index2) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    hasChanged = true;
                }
            }
        }
        return hasChanged.clone();
    }
    

    // throws Exception
    pub fn getCellTypeAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.getCellPositionAtXY(x.clone(), y.clone());
        return self.getCellTypeAt(cellPosition.clone());
    }
    

    pub fn getGeographicMapCellPositionFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> {
        return self.geographicMapCellPositionFactory.clone();
    }
    

    pub fn getGeographicMapCellPositionFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>> {
        return self.geographicMapCellPositionFactoryInterface.clone();
    }
    

    pub fn getGeographicMapCellTypeFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> {
        return self.geographicMapCellTypeFactory.clone();
    }
}
impl std::ops::Deref for BasicGeographicMap {
    type Target = SimpleGeographicMap;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicGeographicMap {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicGeographicMapDyn: crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMapDyn {
    fn as_BasicGeographicMap(&self) -> &BasicGeographicMap;
    fn as_BasicGeographicMap_mut(&mut self) -> &mut BasicGeographicMap;
    fn getCellPosition(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMap::getCellPosition(self.as_BasicGeographicMap_mut(), direction, oldGeographicMapCellPosition) }
    fn getCellPositionNoThrow(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMap::getCellPositionNoThrow(self.as_BasicGeographicMap_mut(), direction, oldGeographicMapCellPosition) }
    fn isOfFourDirections(&mut self, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut newGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { BasicGeographicMap::isOfFourDirections(self.as_BasicGeographicMap_mut(), oldGeographicMapCellPosition, newGeographicMapCellPosition) }
    fn getCellPositionAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMap::getCellPositionAtXY(self.as_BasicGeographicMap_mut(), x, y) }
    fn getCellPositionAtXYNoThrow(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMap::getCellPositionAtXYNoThrow(self.as_BasicGeographicMap_mut(), x, y) }
    fn isOnMap(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { BasicGeographicMap::isOnMap(self.as_BasicGeographicMap_mut(), geographicMapCellPosition) }
    fn getCellPositionAtNoThrow(&mut self, mut x: i32, mut y: i32, mut x2: i32, mut y2: i32, mut geographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { BasicGeographicMap::getCellPositionAtNoThrow(self.as_BasicGeographicMap_mut(), x, y, x2, y2, geographicMapCellPositionList) }
    fn getCellPositionsAt(&mut self, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>, mut currentCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>, mut cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>) -> bool { BasicGeographicMap::getCellPositionsAt(self.as_BasicGeographicMap_mut(), layer, currentCellPositionArray, cellPositionArray) }
    fn getCellTypeAtXY(&mut self, mut x: i32, mut y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> { BasicGeographicMap::getCellTypeAtXY(self.as_BasicGeographicMap_mut(), x, y) }
    fn getGeographicMapCellPositionFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> { BasicGeographicMap::getGeographicMapCellPositionFactory(self.as_BasicGeographicMap_mut()) }
    fn getGeographicMapCellPositionFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>> { BasicGeographicMap::getGeographicMapCellPositionFactoryInterface(self.as_BasicGeographicMap_mut()) }
    fn getGeographicMapCellTypeFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellTypeFactory::GeographicMapCellTypeFactoryDyn>>> { BasicGeographicMap::getGeographicMapCellTypeFactory(self.as_BasicGeographicMap_mut()) }
}
impl BasicGeographicMapDyn for BasicGeographicMap {
    fn as_BasicGeographicMap(&self) -> &BasicGeographicMap { self }
    fn as_BasicGeographicMap_mut(&mut self) -> &mut BasicGeographicMap { self }
}

impl crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMapDyn for BasicGeographicMap {
    fn as_SimpleGeographicMap(&self) -> &crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &self.base }
    fn as_SimpleGeographicMap_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMap::SimpleGeographicMap { &mut self.base }
}


