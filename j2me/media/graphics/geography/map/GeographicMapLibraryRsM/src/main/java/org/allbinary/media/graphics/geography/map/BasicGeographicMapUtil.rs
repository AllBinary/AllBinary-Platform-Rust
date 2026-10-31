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
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGeographicMapUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static BasicGeographicMapUtil_instance: Option<Rc<RefCell<BasicGeographicMapUtil>>> = BasicGeographicMapUtil::new(); }

impl BasicGeographicMapUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicGeographicMapUtil>>> {
        return BasicGeographicMapUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getBorderingRow(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i32 {
        let mut LOCATION_CHANGE: i32 = 1;
        match direction {
            switch_value if switch_value == 0 => {
                return jcall!(oldGeographicMapCellPosition).getRow();
            }
            switch_value if switch_value == 1 => {
                return jcall!(oldGeographicMapCellPosition).getRow();
            }
            switch_value if switch_value == 2 => {
                return jcall!(oldGeographicMapCellPosition).getRow() + LOCATION_CHANGE;
            }
            switch_value if switch_value == 3 => {
                return jcall!(oldGeographicMapCellPosition).getRow() - LOCATION_CHANGE;
            }
            _ => {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Only Four Directions"))));
            }
        }
    }
    

    // throws Exception
    pub fn getBorderingColumn(&mut self, mut direction: i32, mut oldGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> i32 {
        let mut LOCATION_CHANGE: i32 = 1;
        match direction {
            switch_value if switch_value == 0 => {
                return jcall!(oldGeographicMapCellPosition).getColumn() - LOCATION_CHANGE;
            }
            switch_value if switch_value == 1 => {
                return jcall!(oldGeographicMapCellPosition).getColumn() + LOCATION_CHANGE;
            }
            switch_value if switch_value == 2 => {
                return jcall!(oldGeographicMapCellPosition).getColumn();
            }
            switch_value if switch_value == 3 => {
                return jcall!(oldGeographicMapCellPosition).getColumn();
            }
            _ => {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Only Four Directions"))));
            }
        }
    }
    

    // throws Exception
    pub fn isSameCellPosition(&mut self, mut fromGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut toGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        let mut fromColumn: i32 = jcall!(fromGeographicMapCellPosition).getColumn();
        let mut fromRow: i32 = jcall!(fromGeographicMapCellPosition).getRow();
        let mut goColumn: i32 = jcall!(toGeographicMapCellPosition).getColumn();
        let mut goRow: i32 = jcall!(toGeographicMapCellPosition).getRow();
        if fromColumn == goColumn && fromRow == goRow {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn createAllBinaryTiledLayerArray(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>> {
        let mut tiledLayerArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>> = vec![None; ((geographicMapInterfaceArray.len() as i32)) as usize];
        return self.getAllBinaryTiledLayerArray(geographicMapInterfaceArray.clone(), tiledLayerArray.clone(), 0);
    }
    

    pub fn getAllBinaryTiledLayerArray(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut tiledLayerArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>>, mut startIndex: i32) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>> {
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = None;
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        let mut count: i32 = 0;
        {
            let mut index: i32 = size + startIndex;
            while { index -= 1; index } >= startIndex {
            geographicMapInterface = geographicMapInterfaceArray[(index) as usize].clone();
            tiledLayerArray[({ let value = count; count += 1; value }) as usize] = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
            }
        }
        return tiledLayerArray.clone();
    }
    

    pub fn r#move(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut dx: i32, mut dy: i32) {
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = None;
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            geographicMapInterface = geographicMapInterfaceArray[(index) as usize].clone();
            jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).moveDXY(dx.clone(), dy.clone());
            }
        }
    }
    

    pub fn setPosition(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut x: i32, mut y: i32) {
        let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = None;
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            geographicMapInterface = geographicMapInterfaceArray[(index) as usize].clone();
            jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).setPosition(x.clone(), y.clone(), jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getZP());
            }
        }
    }
}

