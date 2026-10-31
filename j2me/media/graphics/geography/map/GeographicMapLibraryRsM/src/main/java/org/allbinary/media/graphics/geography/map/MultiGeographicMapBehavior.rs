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
// package org.allbinary.media.graphics.geography.map
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GeographicMapCellType::GeographicMapCellType;
use super::BasicGeographicMap::BasicGeographicMap;
use super::GeographicMapCellPosition::GeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultiGeographicMapBehavior {
}


impl MultiGeographicMapBehavior {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getCellTypeAt(&mut self, mut geographicMapInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>>, mut geographicMapCellTypeArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>> {
        let mut size: i32 = (geographicMapInterfaceArray.len() as i32).clone();
        let mut cellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> = GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE.with(Clone::clone).clone();
        {
            let mut index: i32 = size.clone();
            while { index -= 1; index } >= 0 {
            cellType = jcall!(geographicMapInterfaceArray[(index) as usize]).getCellTypeAt(geographicMapCellPosition.clone());
            geographicMapCellTypeArray[(index) as usize] = cellType.clone();
            }
        }
        return geographicMapCellTypeArray.clone();
    }
}

