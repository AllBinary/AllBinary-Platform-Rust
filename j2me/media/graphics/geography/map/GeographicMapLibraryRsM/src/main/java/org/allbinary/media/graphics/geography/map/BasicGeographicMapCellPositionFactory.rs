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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::java::lang::Exception::*;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::BasicGeographicMap::BasicGeographicMap;
use super::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;
use super::GeographicMapCellPositionFactoryInitVisitorInterface::GeographicMapCellPositionFactoryInitVisitorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicGeographicMapCellPositionFactory {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub geographicMapCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>,
    pub geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>,
    pub geographicMapCellPositionFactoryInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>,
    pub columns: i32,
    pub rows: i32,
    pub tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>,
}


impl BasicGeographicMapCellPositionFactory {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            geographicMapCellPositionArray: Vec::new(),
            geographicMapInterface: None,
            geographicMapCellPositionFactoryInterface: None,
            columns: 0,
            rows: 0,
            tiledLayer: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.geographicMapInterface = geographicMapInterface.clone();
        this.geographicMapCellPositionFactoryInterface = { let java_value = (jcall!(this.geographicMapInterface).getGeographicMapCellPositionFactoryInterface()).clone(); java_value };
        this.tiledLayer = { let java_value = (jcall!(this.geographicMapInterface).getAllBinaryTiledLayer()).clone(); java_value };
        this.columns = { let java_value = (jcall!(this.tiledLayer).getColumns()).clone(); java_value };
        this.rows = { let java_value = (jcall!(this.tiledLayer).getRows()).clone(); java_value };
        this.geographicMapCellPositionArray = vec![vec![None; (this.columns) as usize]; (this.rows) as usize];
        this.init();
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.tiledLayer.clone();
        let mut columns: i32 = self.getColumns();
        let mut rows: i32 = self.getRows();
        let mut width: i32 = jcall!(tiledLayer).getCellWidth();
        let mut height: i32 = jcall!(tiledLayer).getCellHeight();
        for column in (0..columns) {
            for row in (0..rows) {
                self.createInstance(column.clone(), row.clone(), width.clone(), height.clone());
            }
        }
    }
    

    // throws Exception
    pub fn visit(&mut self, mut geographicMapCelPositionFactoryInitVisitorInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInitVisitorInterface>>>) {
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = self.tiledLayer.clone();
        let mut columns: i32 = self.getColumns();
        let mut rows: i32 = self.getRows();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = None;
            for column in (0..columns) {
                for row in (0..rows) {
                    cellPosition = self.getAt(column.clone(), row.clone());
                    jcall!(geographicMapCelPositionFactoryInitVisitorInterface).visit(tiledLayer.clone(), cellPosition.clone());
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append({ let java_field = jcall!(commonSeps).BRACKET_OPEN.clone(); java_field }.clone()).appendint(rows.clone()).append({ let java_field = jcall!(commonSeps).BRACKET_CLOSE.clone(); java_field }.clone()).append({ let java_field = jcall!(commonSeps).BRACKET_OPEN.clone(); java_field }.clone()).appendint(columns.clone()).append({ let java_field = jcall!(commonSeps).BRACKET_CLOSE.clone(); java_field }.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("visit"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            panic!("{:?}", e);
            }
        }
    }
    

    // throws Exception
    pub fn getAt(&mut self, mut i_column: i32, mut i_row: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.geographicMapCellPositionArray[(i_row) as usize][(i_column) as usize].clone();
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut anotherMapGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return { let java_arguments = (jcall!(anotherMapGeographicMapCellPosition).getColumn(), jcall!(anotherMapGeographicMapCellPosition).getRow(),); self.getAt(java_arguments.0, java_arguments.1) };
    }
    

    // throws Exception
    pub fn createInstance(&mut self, mut i_column: i32, mut i_row: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut cellPositionCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = crate::java_into_object(self.geographicMapCellPositionArray[(i_row) as usize][(i_column) as usize].clone());
        if cellPositionCanBeNull.is_none() {
            cellPositionCanBeNull = crate::java_into_object({ let java_value = (jcall!(self.geographicMapCellPositionFactoryInterface).getInstance(self.geographicMapInterface.clone(), i_column.clone(), i_row.clone(), self.columns.clone(), self.rows.clone(), width.clone(), height.clone())).clone(); java_value });
            { let java_index = (i_column) as usize; self.geographicMapCellPositionArray[(i_row) as usize][java_index] = { let java_object = (cellPositionCanBeNull).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) }; };
        }
        return { let java_object = (cellPositionCanBeNull).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
    }
    

    pub fn getColumns(&mut self) -> i32 {
        return self.columns.clone();
    }
    

    pub fn getRows(&mut self) -> i32 {
        return self.rows.clone();
    }
}
pub trait BasicGeographicMapCellPositionFactoryDyn: std::any::Any {
    fn as_BasicGeographicMapCellPositionFactory(&self) -> &BasicGeographicMapCellPositionFactory;
    fn as_BasicGeographicMapCellPositionFactory_mut(&mut self) -> &mut BasicGeographicMapCellPositionFactory;
    fn init(&mut self) { BasicGeographicMapCellPositionFactory::init(self.as_BasicGeographicMapCellPositionFactory_mut()) }
    fn visit(&mut self, mut geographicMapCelPositionFactoryInitVisitorInterface: Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInitVisitorInterface>>>) { BasicGeographicMapCellPositionFactory::visit(self.as_BasicGeographicMapCellPositionFactory_mut(), geographicMapCelPositionFactoryInitVisitorInterface) }
    fn getAt(&mut self, mut i_column: i32, mut i_row: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMapCellPositionFactory::getAt(self.as_BasicGeographicMapCellPositionFactory_mut(), i_column, i_row) }
    fn getInstance(&mut self, mut anotherMapGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMapCellPositionFactory::getInstance(self.as_BasicGeographicMapCellPositionFactory_mut(), anotherMapGeographicMapCellPosition) }
    fn createInstance(&mut self, mut i_column: i32, mut i_row: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { BasicGeographicMapCellPositionFactory::createInstance(self.as_BasicGeographicMapCellPositionFactory_mut(), i_column, i_row, width, height) }
    fn getColumns(&mut self) -> i32 { BasicGeographicMapCellPositionFactory::getColumns(self.as_BasicGeographicMapCellPositionFactory_mut()) }
    fn getRows(&mut self) -> i32 { BasicGeographicMapCellPositionFactory::getRows(self.as_BasicGeographicMapCellPositionFactory_mut()) }
}
impl BasicGeographicMapCellPositionFactoryDyn for BasicGeographicMapCellPositionFactory {
    fn as_BasicGeographicMapCellPositionFactory(&self) -> &BasicGeographicMapCellPositionFactory { self }
    fn as_BasicGeographicMapCellPositionFactory_mut(&mut self) -> &mut BasicGeographicMapCellPositionFactory { self }
}


