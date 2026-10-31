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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::layer::Layer::Layer;
use crate::org::allbinary::layer::NamedInterface::NamedInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryTiledLayer {
    pub base: Layer,
    pub cellWidth: i32,
    pub cellHeight: i32,
    pub halfWidth: i32,
    pub halfHeight: i32,
    pub halfCellWidth: i32,
    pub halfCellHeight: i32,
    pub dataId: Option<i32>,
}

impl NamedInterface for AllBinaryTiledLayer {
    fn getName(&mut self) -> String { Self::getName(self) }
}
impl PaintableInterface for AllBinaryTiledLayer {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paintThreed(self, graphics) }
}

impl AllBinaryTiledLayer {
    

    pub fn new_struct(mut dataId: Option<i32>, mut width: i32, mut height: i32, mut cellWidth: i32, mut cellHeight: i32) -> Self {
        let mut this = Self {
            base: Layer::new_struct(0, 0),
            cellWidth: 0,
            cellHeight: 0,
            halfWidth: 0,
            halfHeight: 0,
            halfCellWidth: 0,
            halfCellHeight: 0,
            dataId: None,
        };
        // super(...) is in the struct literal above
        this.dataId = dataId.clone();
        this.setLayerWidth(width.clone());
        this.setLayerHeight(height.clone());
        this.base.setPosition(0, 0, 0);
        this.cellWidth = cellWidth.clone();
        this.cellHeight = cellHeight.clone();
        this.halfWidth = (this.getWidth() >> 1);
        this.halfHeight = (this.getHeight() >> 1);
        this.halfCellWidth = (cellWidth >> 1);
        this.halfCellHeight = (cellHeight >> 1);
        this
    }

    pub fn new(mut dataId: Option<i32>, mut width: i32, mut height: i32, mut cellWidth: i32, mut cellHeight: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(dataId, width, height, cellWidth, cellHeight))))
    }
    

    pub fn setCells(&mut self, mut mapTwoDArray: Vec<Vec<i32>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Start: r: "));
        jcall!(stringBuffer).appendint((mapTwoDArray.len() as i32).clone());
        jcall!(stringBuffer).append(String::from(" c: "));
        jcall!(stringBuffer).appendint((mapTwoDArray[(0) as usize].len() as i32).clone());
        jcall!(stringBuffer).append(String::from(" rows: "));
        jcall!(stringBuffer).appendint(self.getRows());
        jcall!(stringBuffer).append(String::from(" columns: "));
        jcall!(stringBuffer).appendint(self.getColumns());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setCells"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        let mut rows: i32 = self.getRows();
        let mut columns: i32 = self.getColumns();
        for col in (0..columns) {
            for row in (0..rows) {
                self.setCell(col.clone(), row.clone(), mapTwoDArray[(row) as usize][(col) as usize].clone());
            }
        }
    }
    

    pub fn updateCells(&mut self, mut mapTwoDArray: Vec<Vec<i32>>, mut fromTileId: i32, mut toTileId: i32) {
        let mut rows: i32 = self.getRows();
        let mut columns: i32 = self.getColumns();
        for col in (0..columns) {
            for row in (0..rows) {
                if fromTileId == mapTwoDArray[(row) as usize][(col) as usize] {
                    mapTwoDArray[(row) as usize][(col) as usize] = toTileId.clone();
                    self.setCell(col.clone(), row.clone(), mapTwoDArray[(row) as usize][(col) as usize].clone());
                }
            }
        }
    }
    

    pub fn getName(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
    

    pub fn getX2(&mut self) -> i32 {
        return self.x + self.getWidth();
    }
    

    pub fn getY2(&mut self) -> i32 {
        return self.y + self.getHeight();
    }
    

    pub fn getZ2(&mut self) -> i32 {
        return (((self.z + self.getDepth())) as i32);
    }
    

    pub fn getCellWidth(&mut self) -> i32 {
        return self.cellWidth.clone();
    }
    

    pub fn getCellHeight(&mut self) -> i32 {
        return self.cellHeight.clone();
    }
    

    pub fn getCell(&mut self, mut col: i32, mut row: i32) -> i32 {
        return -1;
    }
    

    pub fn getColumns(&mut self) -> i32 {
        return -1;
    }
    

    pub fn getRows(&mut self) -> i32 {
        return -1;
    }
    

    pub fn setCell(&mut self, mut col: i32, mut row: i32, mut tileIndex: i32) {
    }
    

    pub fn getHalfWidth(&mut self) -> i32 {
        return self.halfWidth.clone();
    }
    

    pub fn getHalfHeight(&mut self) -> i32 {
        return self.halfHeight.clone();
    }
    

    pub fn getDataId(&mut self) -> Option<i32> {
        return self.dataId.clone();
    }
    

    pub fn getHalfCellWidth(&mut self) -> i32 {
        return self.halfCellWidth.clone();
    }
    

    pub fn getHalfCellHeight(&mut self) -> i32 {
        return self.halfCellHeight.clone();
    }
    

    pub fn isOnTileLayer(&mut self, mut column: i32, mut row: i32) -> bool {
        if self.getColumns() > column && self.getRows() > row && row >= 0 && column >= 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getAnimatedTile(&mut self, mut animationTileIndex: i32) -> i32 {
        return animationTileIndex.clone();
    }
}
impl std::ops::Deref for AllBinaryTiledLayer {
    type Target = Layer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryTiledLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryTiledLayerDyn: crate::org::allbinary::layer::Layer::LayerDyn + crate::org::allbinary::layer::NamedInterface::NamedInterface {
    fn as_AllBinaryTiledLayer(&self) -> &AllBinaryTiledLayer;
    fn as_AllBinaryTiledLayer_mut(&mut self) -> &mut AllBinaryTiledLayer;
    fn setCells(&mut self, mut mapTwoDArray: Vec<Vec<i32>>) { AllBinaryTiledLayer::setCells(self.as_AllBinaryTiledLayer_mut(), mapTwoDArray) }
    fn updateCells(&mut self, mut mapTwoDArray: Vec<Vec<i32>>, mut fromTileId: i32, mut toTileId: i32) { AllBinaryTiledLayer::updateCells(self.as_AllBinaryTiledLayer_mut(), mapTwoDArray, fromTileId, toTileId) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { AllBinaryTiledLayer::paintThreed(self.as_AllBinaryTiledLayer_mut(), graphics) }
    fn getX2(&mut self) -> i32 { AllBinaryTiledLayer::getX2(self.as_AllBinaryTiledLayer_mut()) }
    fn getY2(&mut self) -> i32 { AllBinaryTiledLayer::getY2(self.as_AllBinaryTiledLayer_mut()) }
    fn getZ2(&mut self) -> i32 { AllBinaryTiledLayer::getZ2(self.as_AllBinaryTiledLayer_mut()) }
    fn getCellWidth(&mut self) -> i32 { AllBinaryTiledLayer::getCellWidth(self.as_AllBinaryTiledLayer_mut()) }
    fn getCellHeight(&mut self) -> i32 { AllBinaryTiledLayer::getCellHeight(self.as_AllBinaryTiledLayer_mut()) }
    fn getCell(&mut self, mut col: i32, mut row: i32) -> i32 { AllBinaryTiledLayer::getCell(self.as_AllBinaryTiledLayer_mut(), col, row) }
    fn getColumns(&mut self) -> i32 { AllBinaryTiledLayer::getColumns(self.as_AllBinaryTiledLayer_mut()) }
    fn getRows(&mut self) -> i32 { AllBinaryTiledLayer::getRows(self.as_AllBinaryTiledLayer_mut()) }
    fn setCell(&mut self, mut col: i32, mut row: i32, mut tileIndex: i32) { AllBinaryTiledLayer::setCell(self.as_AllBinaryTiledLayer_mut(), col, row, tileIndex) }
    fn getHalfWidth(&mut self) -> i32 { AllBinaryTiledLayer::getHalfWidth(self.as_AllBinaryTiledLayer_mut()) }
    fn getHalfHeight(&mut self) -> i32 { AllBinaryTiledLayer::getHalfHeight(self.as_AllBinaryTiledLayer_mut()) }
    fn getDataId(&mut self) -> Option<i32> { AllBinaryTiledLayer::getDataId(self.as_AllBinaryTiledLayer_mut()) }
    fn getHalfCellWidth(&mut self) -> i32 { AllBinaryTiledLayer::getHalfCellWidth(self.as_AllBinaryTiledLayer_mut()) }
    fn getHalfCellHeight(&mut self) -> i32 { AllBinaryTiledLayer::getHalfCellHeight(self.as_AllBinaryTiledLayer_mut()) }
    fn isOnTileLayer(&mut self, mut column: i32, mut row: i32) -> bool { AllBinaryTiledLayer::isOnTileLayer(self.as_AllBinaryTiledLayer_mut(), column, row) }
    fn getAnimatedTile(&mut self, mut animationTileIndex: i32) -> i32 { AllBinaryTiledLayer::getAnimatedTile(self.as_AllBinaryTiledLayer_mut(), animationTileIndex) }
}
impl AllBinaryTiledLayerDyn for AllBinaryTiledLayer {
    fn as_AllBinaryTiledLayer(&self) -> &AllBinaryTiledLayer { self }
    fn as_AllBinaryTiledLayer_mut(&mut self) -> &mut AllBinaryTiledLayer { self }
}

impl crate::org::allbinary::layer::Layer::LayerDyn for AllBinaryTiledLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


