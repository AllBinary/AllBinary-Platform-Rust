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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonLocationHelper {
    pub totalColumns: i32,
    pub totalRows: i32,
    pub columnsRemainder: i32,
    pub rowsRemainder: i32,
    pub columnsRemainderHalf: i32,
    pub rowsRemainderHalf: i32,
}


impl TouchButtonLocationHelper {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            totalColumns: 0,
            totalRows: 0,
            columnsRemainder: 0,
            rowsRemainder: 0,
            columnsRemainderHalf: 0,
            rowsRemainderHalf: 0,
        };
        let mut buttonSize: i32 = jcall!(CommonButtons::CommonButtons::getInstance()).STANDARD_BUTTON_SIZE.clone();
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        this.totalColumns = ((((jcall!(displayInfo).getLastWidth()) as f32) / ((buttonSize) as f32)) as f64).round() as i64;
        this.totalRows = ((((jcall!(displayInfo).getLastHeight()) as f32) / ((buttonSize) as f32)) as f64).round() as i64;
        this.columnsRemainder = jcall!(displayInfo).getLastWidth() % buttonSize;
        this.columnsRemainderHalf = (this.columnsRemainder >> 1);
        this.rowsRemainder = jcall!(displayInfo).getLastHeight() % buttonSize;
        this.rowsRemainderHalf = (this.rowsRemainder >> 1);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getTotalColumns(&mut self) -> i32 {
        return self.totalColumns.clone();
    }
    

    pub fn getTotalRows(&mut self) -> i32 {
        return self.totalRows.clone();
    }
    

    pub fn getColumnsRemainder(&mut self) -> i32 {
        return self.columnsRemainder.clone();
    }
    

    pub fn getRowsRemainder(&mut self) -> i32 {
        return self.rowsRemainder.clone();
    }
    

    pub fn getColumnsRemainderHalf(&mut self) -> i32 {
        return self.columnsRemainderHalf.clone();
    }
    

    pub fn getRowsRemainderHalf(&mut self) -> i32 {
        return self.rowsRemainderHalf.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("totalColumns: "));
        jcall!(stringBuffer).appendint(self.totalColumns.clone());
        jcall!(stringBuffer).append(String::from(" totalRows: "));
        jcall!(stringBuffer).appendint(self.totalRows.clone());
        jcall!(stringBuffer).append(String::from(" columnsRemainder: "));
        jcall!(stringBuffer).appendint(self.columnsRemainder.clone());
        jcall!(stringBuffer).append(String::from(" rowsRemainder: "));
        jcall!(stringBuffer).appendint(self.rowsRemainder.clone());
        jcall!(stringBuffer).append(String::from(" columnsRemainderHalf: "));
        jcall!(stringBuffer).appendint(self.columnsRemainderHalf.clone());
        jcall!(stringBuffer).append(String::from(" rowsRemainderHalf: "));
        jcall!(stringBuffer).appendint(self.rowsRemainderHalf.clone());
        return jcall!(stringBuffer).toString();
    }
}

