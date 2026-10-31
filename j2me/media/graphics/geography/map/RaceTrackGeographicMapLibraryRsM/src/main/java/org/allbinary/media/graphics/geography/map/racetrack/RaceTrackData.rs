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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::CellPosition::CellPosition;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RaceTrackData {
    pub tileSetImage: Option<Rc<RefCell<Image>>>,
    pub miniTileSetImage: Option<Rc<RefCell<Image>>>,
    pub cellWidth: i32,
    pub cellHeight: i32,
    pub miniCellWidth: i32,
    pub miniCellHeight: i32,
    pub mapArray: Vec<Vec<i32>>,
    pub cellTypeIdToGeographicMapCellTypeArray: Vec<i32>,
    pub NO_OVER_PASSES_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>,
    pub overPassGeographicMapCellPositionArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>,
    pub id: Option<i32>,
}


impl RaceTrackData {
    

    pub fn new_struct(mut id: Option<i32>, mut cellWidth: i32, mut cellHeight: i32, mut miniCellWidth: i32, mut miniCellHeight: i32, mut cellTypeIdToGeographicMapCellTypeArray: Vec<i32>, mut mapArray: Vec<Vec<i32>>) -> Self {
        let mut this = Self {
            tileSetImage: None,
            miniTileSetImage: None,
            cellWidth: 0,
            cellHeight: 0,
            miniCellWidth: 0,
            miniCellHeight: 0,
            mapArray: Vec::new(),
            cellTypeIdToGeographicMapCellTypeArray: Vec::new(),
            NO_OVER_PASSES_ARRAY: Vec::new(),
            overPassGeographicMapCellPositionArray: Vec::new(),
            id: None,
        };
        this.tileSetImage = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        this.miniTileSetImage = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        this.cellWidth = 0;
        this.cellHeight = 0;
        this.miniCellWidth = 0;
        this.miniCellHeight = 0;
        this.mapArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone();
        this.cellTypeIdToGeographicMapCellTypeArray = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone();
        this.NO_OVER_PASSES_ARRAY = vec![None; (0) as usize];
        this.overPassGeographicMapCellPositionArray = this.NO_OVER_PASSES_ARRAY.clone();
        this.id = id.clone();
        this.cellWidth = cellWidth.clone();
        this.cellHeight = cellHeight.clone();
        this.miniCellWidth = miniCellWidth.clone();
        this.miniCellHeight = miniCellHeight.clone();
        this.mapArray = mapArray.clone();
        this.cellTypeIdToGeographicMapCellTypeArray = cellTypeIdToGeographicMapCellTypeArray.clone();
        this
    }

    pub fn new(mut id: Option<i32>, mut cellWidth: i32, mut cellHeight: i32, mut miniCellWidth: i32, mut miniCellHeight: i32, mut cellTypeIdToGeographicMapCellTypeArray: Vec<i32>, mut mapArray: Vec<Vec<i32>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(id, cellWidth, cellHeight, miniCellWidth, miniCellHeight, cellTypeIdToGeographicMapCellTypeArray, mapArray))))
    }
    

    pub fn setMapArray(&mut self, mut mapArray: Vec<Vec<i32>>) {
        self.mapArray = mapArray.clone();
    }
    

    pub fn getMapArray(&mut self) -> Vec<Vec<i32>> {
        return self.mapArray.clone();
    }
    

    pub fn setCellTypeIdToGeographicMapCellTypeArray(&mut self, mut cellTypeIdToGeographicMapCellTypeArray: Vec<i32>) {
        self.cellTypeIdToGeographicMapCellTypeArray = cellTypeIdToGeographicMapCellTypeArray.clone();
    }
    

    pub fn getCellTypeIdToGeographicMapCellTypeArray(&mut self) -> Vec<i32> {
        return self.cellTypeIdToGeographicMapCellTypeArray.clone();
    }
    

    pub fn setCellWidth(&mut self, mut cellWidth: i32) {
        self.cellWidth = cellWidth.clone();
    }
    

    pub fn getCellWidth(&mut self) -> i32 {
        return self.cellWidth.clone();
    }
    

    pub fn setCellHeight(&mut self, mut cellHeight: i32) {
        self.cellHeight = cellHeight.clone();
    }
    

    pub fn getCellHeight(&mut self) -> i32 {
        return self.cellHeight.clone();
    }
    

    pub fn getOverPassGeographicMapCellPositionArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>> {
        return self.overPassGeographicMapCellPositionArray.clone();
    }
    

    pub fn setOverPassGeographicMapCellPositionArray(&mut self, mut overPassGeographicMapCellPositionArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>>>) {
        self.overPassGeographicMapCellPositionArray = overPassGeographicMapCellPositionArray.clone();
    }
    

    pub fn getTileSetImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        return self.tileSetImage.clone();
    }
    

    pub fn setTileSetImage(&mut self, mut tileSetImage: Option<Rc<RefCell<Image>>>) {
        self.tileSetImage = tileSetImage.clone();
    }
    

    pub fn getMiniTileSetImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        return self.miniTileSetImage.clone();
    }
    

    pub fn setMiniTileSetImage(&mut self, mut miniTileSetImage: Option<Rc<RefCell<Image>>>) {
        self.miniTileSetImage = miniTileSetImage.clone();
    }
    

    pub fn getMiniCellWidth(&mut self) -> i32 {
        return self.miniCellWidth.clone();
    }
    

    pub fn setMiniCellWidth(&mut self, mut miniCellWidth: i32) {
        self.miniCellWidth = miniCellWidth.clone();
    }
    

    pub fn getMiniCellHeight(&mut self) -> i32 {
        return self.miniCellHeight.clone();
    }
    

    pub fn setMiniCellHeight(&mut self, mut miniCellHeight: i32) {
        self.miniCellHeight = miniCellHeight.clone();
    }
    

    pub fn setId(&mut self, mut id: Option<i32>) {
        self.id = id.clone();
    }
    

    pub fn getId(&mut self) -> Option<i32> {
        return self.id.clone();
    }
}

