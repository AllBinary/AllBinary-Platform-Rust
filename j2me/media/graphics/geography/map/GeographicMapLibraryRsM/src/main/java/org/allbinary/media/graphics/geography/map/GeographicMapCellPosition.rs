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
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::graphics::GPoint::GPoint;
use crate::org::allbinary::graphics::PointFactory::PointFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPosition {
    pub base: CellPosition,
    pub point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub midPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
}


impl GeographicMapCellPosition {
    

    pub fn new_struct(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            base: CellPosition::new_struct(column.clone(), row.clone(), columns.clone(), rows.clone()),
            point: None,
            midPoint: None,
        };
        // super(...) is in the struct literal above
        let mut x: i32 = this.getColumn() * width;
        let mut y: i32 = this.getRow() * height;
        let mut pointFactory: Option<Rc<RefCell<PointFactory>>> = PointFactory::getInstance();
        this.point = jcall!(pointFactory).createXY(x.clone(), y.clone());
        this.midPoint = jcall!(pointFactory).createXY(x + (width >> 1), y + (height >> 1));
        this
    }

    pub fn new(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(column, row, columns, rows, width, height))))
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.point.clone();
    }
    

    pub fn getMidPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.midPoint.clone();
    }
    

    pub fn setMidPoint(&mut self, mut midPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.midPoint = midPoint.clone();
    }
}
impl std::ops::Deref for GeographicMapCellPosition {
    type Target = CellPosition;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GeographicMapCellPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait GeographicMapCellPositionDyn: crate::org::allbinary::graphics::CellPosition::CellPositionDyn {
    fn as_GeographicMapCellPosition(&self) -> &GeographicMapCellPosition;
    fn as_GeographicMapCellPosition_mut(&mut self) -> &mut GeographicMapCellPosition;
    fn getPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> { GeographicMapCellPosition::getPoint(self.as_GeographicMapCellPosition_mut()) }
    fn getMidPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> { GeographicMapCellPosition::getMidPoint(self.as_GeographicMapCellPosition_mut()) }
    fn setMidPoint(&mut self, mut midPoint: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) { GeographicMapCellPosition::setMidPoint(self.as_GeographicMapCellPosition_mut(), midPoint) }
}
impl GeographicMapCellPositionDyn for GeographicMapCellPosition {
    fn as_GeographicMapCellPosition(&self) -> &GeographicMapCellPosition { self }
    fn as_GeographicMapCellPosition_mut(&mut self) -> &mut GeographicMapCellPosition { self }
}

impl crate::org::allbinary::graphics::CellPosition::CellPositionDyn for GeographicMapCellPosition {
    fn as_CellPosition(&self) -> &crate::org::allbinary::graphics::CellPosition::CellPosition { &self.base }
    fn as_CellPosition_mut(&mut self) -> &mut crate::org::allbinary::graphics::CellPosition::CellPosition { &mut self.base }
}
impl crate::org::allbinary::graph::V::VDyn for GeographicMapCellPosition {
    fn as_V(&self) -> &crate::org::allbinary::graph::V::V { &self.base.base }
    fn as_V_mut(&mut self) -> &mut crate::org::allbinary::graph::V::V { &mut self.base.base }
    fn getId(&mut self) -> i32 { crate::org::allbinary::graphics::CellPosition::CellPosition::getId(crate::org::allbinary::graphics::CellPosition::CellPositionDyn::as_CellPosition_mut(self)) }
}


