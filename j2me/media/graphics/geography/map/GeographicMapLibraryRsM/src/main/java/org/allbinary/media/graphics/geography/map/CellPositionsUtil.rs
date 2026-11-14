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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use crate::java::lang::Exception::*;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::BasicGeographicMap::BasicGeographicMap;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CellPositionsUtil {
    pub reusableSingleThreadedSurroundingList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub surroundArray: Vec<Vec<i32>>,
}

thread_local! { pub static CellPositionsUtil_instance: Option<Rc<RefCell<CellPositionsUtil>>> = CellPositionsUtil::new(); }

impl CellPositionsUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            reusableSingleThreadedSurroundingList: None,
            surroundArray: Vec::new(),
        };
        this.reusableSingleThreadedSurroundingList = BasicArrayListS::new(8).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.surroundArray = vec![vec![-1, -1], vec![0, -1], vec![1, -1], vec![-1, 1], vec![0, 1], vec![1, 1], vec![1, 0], vec![-1, 0]];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CellPositionsUtil>>> {
        return CellPositionsUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getAll(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut topRightGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut columns: i32, mut rows: i32, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableList).clear();
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut lastColumn: i32 = jcall!(topRightGeographicMapCellPosition).getColumn() + columns;
        let mut lastRow: i32 = jcall!(topRightGeographicMapCellPosition).getRow() + rows;
        if (columns > 1 && lastColumn > jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getColumns()) || (rows > 1 && lastRow > jcall!(jcall!(geographicMapInterface).getAllBinaryTiledLayer()).getRows()) {
            return reusableList.clone();
        }
        for rowIndex in (0..rows) {
            for columnIndex in (0..columns) {
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(jcall!(topRightGeographicMapCellPosition).getColumn() + columnIndex, jcall!(topRightGeographicMapCellPosition).getRow() + rowIndex);
                jcall!(reusableList).add(crate::java_into_object(geographicMapCellPosition.clone()));
            }
        }
        return reusableList.clone();
    }
    

    // throws Exception
    pub fn getAllSurrounding(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut occupyList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>, mut reusableList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableList).clear();
        for index in (0..={ let java_bound = jcall!(occupyList).size() - 1; java_bound }).rev() {
            let mut layerGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(occupyList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
            let mut surroundingGeographicMapCellPositionList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_arguments = (geographicMapInterface.clone(), layerGeographicMapCellPosition.clone(), self.reusableSingleThreadedSurroundingList.clone(),); self.getAllSurroundingAt(java_arguments.0, java_arguments.1, java_arguments.2) };
            for index2 in (0..={ let java_bound = jcall!(surroundingGeographicMapCellPositionList).size() - 1; java_bound }).rev() {
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = { let java_object = (jcall!(surroundingGeographicMapCellPositionList).get(index2.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>().expect("ClassCastException").clone()) };
                if !jcall!(reusableList).contains(crate::java_into_object(geographicMapCellPosition.clone())) && !jcall!(occupyList).contains(crate::java_into_object(geographicMapCellPosition.clone())) {
                    jcall!(reusableList).add(crate::java_into_object(geographicMapCellPosition.clone()));
                }
            }
        }
        return reusableList.clone();
    }
    

    // throws Exception
    pub fn getAllSurroundingAt(&mut self, mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>, mut layerGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut reusableSurroundingList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        jcall!(reusableSurroundingList).clear();
        let mut geographicMapCellPositionFactory: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>> = jcall!(geographicMapInterface).getGeographicMapCellPositionFactory();
        let mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>> = jcall!(geographicMapInterface).getAllBinaryTiledLayer();
        let mut row: i32 = 0;
        let mut column: i32 = 0;
        for index in (0..8) {
            column = { let java_value = (jcall!(layerGeographicMapCellPosition).getColumn() + self.surroundArray[(index) as usize][(0) as usize]).clone(); java_value };
            row = { let java_value = (jcall!(layerGeographicMapCellPosition).getRow() + self.surroundArray[(index) as usize][(1) as usize]).clone(); java_value };
            if jcall!(tiledLayer).isOnTileLayer(column.clone(), row.clone()) {
                let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(geographicMapCellPositionFactory).getAt(column.clone(), row.clone());
                jcall!(reusableSurroundingList).add(crate::java_into_object(geographicMapCellPosition.clone()));
            }
        }
        return reusableSurroundingList.clone();
    }
}

