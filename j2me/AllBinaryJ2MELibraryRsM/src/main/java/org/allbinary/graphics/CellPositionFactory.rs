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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::CellPosition::CellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CellPositionFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub NONE: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>,
    pub cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>>,
    pub columns: i32,
    pub rows: i32,
}

pub static mut CellPositionFactory_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<CellPositionFactory>>>> = std::cell::LazyCell::new(|| CellPositionFactory::new());

impl CellPositionFactory {
    

    pub fn getInstance() -> Option<Rc<RefCell<CellPositionFactory>>> {
        return unsafe { CellPositionFactory_SINGLETON.clone() }.clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            NONE: None,
            cellPositionArray: Vec::new(),
            columns: 0,
            rows: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.NONE = CellPosition::new(-1, -1, -1, -1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>);
        this.cellPositionArray = vec![vec![None; (0) as usize]; (0) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut columns: i32, mut rows: i32) {
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Init: columns: "));
        jcall!(stringBuffer).appendint(columns.clone());
        jcall!(stringBuffer).append(String::from(" rows: "));
        jcall!(stringBuffer).appendint(rows.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).INIT.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.cellPositionArray = vec![vec![None; (rows) as usize]; (columns) as usize];
        self.columns = columns.clone();
        self.rows = rows.clone();
        for column in (0..columns) {
            for row in (0..rows) {
                self.createInstance(column.clone(), row.clone());
            }
        }
    }
    

    pub fn getInstanceColRow(&mut self, mut i_column: i32, mut i_row: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return self.cellPositionArray[(i_column) as usize][(i_row) as usize].clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(String::from("columns: ")).appendint(self.getColumns()).append(String::from(" rows: ")).appendint(self.getRows()).append(String::from(" col: ")).appendint(i_column.clone()).append(String::from(" row: ")).appendint(i_row.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone(), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            return self.NONE.clone();
            }
        }
    }
    

    pub fn createInstance(&mut self, mut i_column: i32, mut i_row: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> {
        let mut cellPositionCanBeNull: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>> = self.cellPositionArray[(i_column) as usize][(i_row) as usize].clone();
        if cellPositionCanBeNull.is_none() {
            cellPositionCanBeNull = CellPosition::new(i_column.clone(), i_row.clone(), self.columns.clone(), self.rows.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>);
            self.cellPositionArray[(i_column) as usize][(i_row) as usize] = cellPositionCanBeNull.clone();
        }
        return cellPositionCanBeNull.clone();
    }
    

    pub fn getColumns(&mut self) -> i32 {
        return self.columns.clone();
    }
    

    pub fn getRows(&mut self) -> i32 {
        return self.rows.clone();
    }
}

