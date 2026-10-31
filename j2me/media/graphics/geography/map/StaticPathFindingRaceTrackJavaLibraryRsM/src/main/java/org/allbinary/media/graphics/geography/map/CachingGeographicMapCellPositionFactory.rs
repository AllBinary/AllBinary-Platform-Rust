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
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::BasicGeographicMap::BasicGeographicMap;
use super::GeographicMapCellPosition::GeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CachingGeographicMapCellPositionFactory {
    pub base: BasicGeographicMapCellPositionFactory,
}


impl CachingGeographicMapCellPositionFactory {
    

    pub fn new_struct(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Self {
        let mut this = Self {
            base: BasicGeographicMapCellPositionFactory::new_struct(geographicMapInterface.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(geographicMapInterface))))
    }
    

    // throws Exception
    pub fn createInstance(&mut self, mut i_column: i32, mut i_row: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = GeographicMapCellPositionFactory::GeographicMapCellPositionFactory::getHashtable();
        let mut cellPositionKey: String = CellPosition::toStringColRow(i_column.clone(), i_row.clone());
        let mut cellPositionCanBeNull: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashtable).get(crate::java_into_object(cellPositionKey.clone()));
        if cellPositionCanBeNull.is_none() {
            cellPositionCanBeNull = { let java_receiver = self.geographicMapCellPositionFactoryInterface.clone(); let java_arguments = (self.geographicMapInterface.clone(), i_column.clone(), i_row.clone(), self.getColumns(), self.getRows(), width.clone(), height.clone(),); let java_result = jcall!(java_receiver).getInstance(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6); java_result };
            self.geographicMapCellPositionArray[(i_row) as usize][(i_column) as usize] = (cellPositionCanBeNull).clone() /* as GeographicMapCellPosition */;
            jcall!(hashtable).put(crate::java_into_object(cellPositionKey.clone()), cellPositionCanBeNull.clone());
        }
        return (cellPositionCanBeNull).clone() /* as GeographicMapCellPosition */;
    }
}
impl std::ops::Deref for CachingGeographicMapCellPositionFactory {
    type Target = BasicGeographicMapCellPositionFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CachingGeographicMapCellPositionFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn for CachingGeographicMapCellPositionFactory {
    fn as_BasicGeographicMapCellPositionFactory(&self) -> &crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory { &self.base }
    fn as_BasicGeographicMapCellPositionFactory_mut(&mut self) -> &mut crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory { &mut self.base }
    fn createInstance(&mut self, mut i_column: i32, mut i_row: i32, mut width: i32, mut height: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::createInstance(self, i_column, i_row, width, height) }
}


