// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.game.layer.geological.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::game::rand::MyRandomFactory::MyRandomFactory;
use crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMap;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use super::GeologicalGeographicMapCellPosition::GeologicalGeographicMapCellPosition;
use super::GeologicalResource::GeologicalResource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeologicalGeographicMapCellPositionFactory {
    pub resourcePerMapCellType: Vec<i32>,
}

impl GeographicMapCellPositionFactoryInterface for GeologicalGeographicMapCellPositionFactory {
    fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut i_column: i32, mut i_row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getInstance(self, geographicMapInterface, i_column, i_row, columns, rows, width, height) }
}

impl GeologicalGeographicMapCellPositionFactory {
    

    pub fn new_struct(mut resourcePerMapCellType: Vec<i32>) -> Self {
        let mut this = Self {
            resourcePerMapCellType: Vec::new(),
        };
        this.resourcePerMapCellType = resourcePerMapCellType.clone();
        this
    }

    pub fn new(mut resourcePerMapCellType: Vec<i32>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(resourcePerMapCellType))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut i_column: i32, mut i_row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut resources: i32 = self.resourcePerMapCellType[(jcall!(tiledLayer).getCell(i_column.clone(), i_row.clone())) as usize] + jcall!(MyRandomFactory::getInstance()).getAbsoluteNextInt(1000);
        return GeologicalGeographicMapCellPosition::new(i_column.clone(), i_row.clone(), columns.clone(), rows.clone(), width.clone(), height.clone(), GeologicalResource::new(resources.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>);
    }
}

