// 
// AllBinary Open License Version 1 
// Copyright (c) 2006 AllBinary 
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
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use super::GeologicalResource::GeologicalResource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeologicalGeographicMapCellPosition {
    pub base: GeographicMapCellPosition,
    pub geologicalResource: Option<Rc<RefCell<GeologicalResource>>>,
}


impl GeologicalGeographicMapCellPosition {
    

    pub fn new_struct(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32, mut geologicalResource: Option<Rc<RefCell<GeologicalResource>>>) -> Self {
        let mut this = Self {
            base: GeographicMapCellPosition::new_struct(column.clone(), row.clone(), columns.clone(), rows.clone(), width.clone(), height.clone()),
            geologicalResource: None,
        };
        // super(...) is in the struct literal above
        this.geologicalResource = geologicalResource.clone();
        this
    }

    pub fn new(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32, mut geologicalResource: Option<Rc<RefCell<GeologicalResource>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(column, row, columns, rows, width, height, geologicalResource))))
    }
    

    pub fn getGeologicalResource(&mut self) -> Option<Rc<RefCell<GeologicalResource>>> {
        return self.geologicalResource.clone();
    }
}
impl std::ops::Deref for GeologicalGeographicMapCellPosition {
    type Target = GeographicMapCellPosition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeologicalGeographicMapCellPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn for GeologicalGeographicMapCellPosition {
    fn as_GeographicMapCellPosition(&self) -> &crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition { &self.base }
    fn as_GeographicMapCellPosition_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition { &mut self.base }
}
impl crate::org::allbinary::graphics::CellPosition::CellPositionDyn for GeologicalGeographicMapCellPosition {
    fn as_CellPosition(&self) -> &crate::org::allbinary::graphics::CellPosition::CellPosition { &self.base.base }
    fn as_CellPosition_mut(&mut self) -> &mut crate::org::allbinary::graphics::CellPosition::CellPosition { &mut self.base.base }
}
impl crate::org::allbinary::graph::V::VDyn for GeologicalGeographicMapCellPosition {
    fn as_V(&self) -> &crate::org::allbinary::graph::V::V { &self.base.base.base }
    fn as_V_mut(&mut self) -> &mut crate::org::allbinary::graph::V::V { &mut self.base.base.base }
    fn getId(&mut self) -> i32 { crate::org::allbinary::graphics::CellPosition::CellPosition::getId(crate::org::allbinary::graphics::CellPosition::CellPositionDyn::as_CellPosition_mut(self)) }
}


