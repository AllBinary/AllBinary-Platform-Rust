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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graph::V::V;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CellPosition {
    pub base: V,
    pub id: i32,
    pub column: i32,
    pub row: i32,
}


impl CellPosition {
    

    pub fn new_struct(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32) -> Self {
        let mut this = Self {
            base: V::new_struct(),
            id: 0,
            column: 0,
            row: 0,
        };
        this.column = column.clone();
        this.row = row.clone();
        this.id = (row * columns) + column;
        this
    }

    pub fn new(mut column: i32, mut row: i32, mut columns: i32, mut rows: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(column, row, columns, rows))))
    }
    

    pub fn getColumn(&mut self) -> i32 {
        return self.column.clone();
    }
    

    pub fn getRow(&mut self) -> i32 {
        return self.row.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return CellPosition::toStringColRow(self.getColumn(), self.getRow());
    }
    

    pub fn toStringCellPosition(mut basicGeographicMapCellPosition: Option<Rc<RefCell<dyn CellPositionDyn>>>) -> String {
        return CellPosition::toStringColRow(jcall!(basicGeographicMapCellPosition).getColumn(), jcall!(basicGeographicMapCellPosition).getRow());
    }
    

    pub fn toStringColRow(mut i_column: i32, mut i_row: i32) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        jcall!(stringBuffer).append(jcall!(commonSeps).PARENTHESIS_OPEN.clone());
        jcall!(stringBuffer).appendint(i_column.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).COMMA.clone());
        jcall!(stringBuffer).appendint(i_row.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).PARENTHESIS_CLOSE.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getId(&mut self) -> i32 {
        return self.id.clone();
    }
}
impl std::ops::Deref for CellPosition {
    type Target = V;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CellPosition {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CellPositionDyn: crate::org::allbinary::graph::V::VDyn {
    fn as_CellPosition(&self) -> &CellPosition;
    fn as_CellPosition_mut(&mut self) -> &mut CellPosition;
    fn getColumn(&mut self) -> i32 { CellPosition::getColumn(self.as_CellPosition_mut()) }
    fn getRow(&mut self) -> i32 { CellPosition::getRow(self.as_CellPosition_mut()) }
    fn toString(&mut self) -> String { CellPosition::toString(self.as_CellPosition_mut()) }
}
impl CellPositionDyn for CellPosition {
    fn as_CellPosition(&self) -> &CellPosition { self }
    fn as_CellPosition_mut(&mut self) -> &mut CellPosition { self }
}

impl crate::org::allbinary::graph::V::VDyn for CellPosition {
    fn as_V(&self) -> &crate::org::allbinary::graph::V::V { &self.base }
    fn as_V_mut(&mut self) -> &mut crate::org::allbinary::graph::V::V { &mut self.base }
    fn getId(&mut self) -> i32 { Self::getId(self) }
}


