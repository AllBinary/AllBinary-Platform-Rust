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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::game::TiledLayer::TiledLayer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::AllBinaryTiledLayer::AllBinaryTiledLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryJ2METiledLayer {
    pub base: AllBinaryTiledLayer,
    pub tiledLayerP: Option<Rc<RefCell<TiledLayer>>>,
    pub debugColor: i32,
    pub font2: Option<Rc<RefCell<Font>>>,
}


impl AllBinaryJ2METiledLayer {
    

    pub fn new_struct(mut dataId: Option<i32>, mut tiledLayer: Option<Rc<RefCell<TiledLayer>>>, mut mapTwoDArray: Vec<Vec<i32>>, mut debugColor: i32) -> Self {
        let mut this = Self {
            base: AllBinaryTiledLayer::new_struct(dataId.clone(), jcall!(tiledLayer).getWidth(), jcall!(tiledLayer).getHeight(), jcall!(tiledLayer).getCellWidth(), jcall!(tiledLayer).getCellHeight()),
            tiledLayerP: None,
            debugColor: 0,
            font2: None,
        };
        this.font2 = Font::getFont(unsafe { crate::javax::microedition::lcdui::Font::Font_FACE_SYSTEM.clone() }.clone(), unsafe { crate::javax::microedition::lcdui::Font::Font_STYLE_PLAIN.clone() }.clone(), 4);
        // super(...) is in the struct literal above
        this.tiledLayerP = tiledLayer.clone();
        this.debugColor = debugColor.clone();
        this.setCells(mapTwoDArray.clone());
        this
    }

    pub fn new(mut dataId: Option<i32>, mut tiledLayer: Option<Rc<RefCell<TiledLayer>>>, mut mapTwoDArray: Vec<Vec<i32>>, mut debugColor: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(dataId, tiledLayer, mapTwoDArray, debugColor))))
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.tiledLayerP).paint(graphics.clone());
    }
    

    pub fn paintDebug(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        jcall!(graphics).setFont(self.font2.clone());
        jcall!(graphics).setColor(self.debugColor.clone());
        let mut x: i32 = jcall!(self.tiledLayerP).getX();
        let mut y: i32 = jcall!(self.tiledLayerP).getY();
        let mut firstColumn: i32 = 0;
        let mut firstRow: i32 = 0;
        let mut totalColumns: i32 = jcall!(self.tiledLayerP).getColumns();
        let mut totalRows: i32 = jcall!(self.tiledLayerP).getRows();
        let mut x0: i32 = x.clone();
        let mut tile: i32 = 0;
        let mut tileHeight: i32 = jcall!(self.tiledLayerP).getCellHeight();
        let mut tileWidth: i32 = jcall!(self.tiledLayerP).getCellWidth();
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut stringMaker: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        {
            let mut rowIndex: i32 = firstRow.clone();
            while rowIndex < totalRows {
                // continue skips the update below
            x = x0.clone();
            {
                let mut columnIndex: i32 = firstColumn.clone();
                while columnIndex < totalColumns {
                    // continue skips the update below
                tile = jcall!(self.tiledLayerP).getCell(columnIndex.clone(), rowIndex.clone());
                if tile < 0 {
                    tile = jcall!(self.tiledLayerP).getAnimatedTile(tile.clone());
                }
                if tile == 0 {
                    continue;
                }
                tile -= 1;
                jcall!(stringMaker).delete(0, jcall!(stringMaker).length());
                jcall!(graphics).drawString(jcall!(stringMaker).appendint(columnIndex.clone()).append(jcall!(commonSeps).COMMA.clone()).appendint(rowIndex.clone()).append(jcall!(commonSeps).COLON.clone()).appendint(tile.clone()).toString(), x + 5, y + 10, 0);
                columnIndex += 1;
                x += tileWidth.clone();
                }
            }
            rowIndex += 1;
            y += tileHeight.clone();
            }
        }
        jcall!(graphics).setFont(font.clone());
    }
    

    pub fn moveDXY(&mut self, mut dx: i32, mut dy: i32) {
        jcall!(self.tiledLayerP).r#move(dx.clone(), dy.clone());
        self.base.moveDXY(-dx, -dy);
    }
    

    pub fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.tiledLayerP).setPosition(x.clone(), y.clone());
        self.base.setPosition(-x, -y, z.clone());
    }
    

    pub fn getTiledLayer(&mut self) -> Option<Rc<RefCell<TiledLayer>>> {
        return self.tiledLayerP.clone();
    }
    

    pub fn getCellWidth(&mut self) -> i32 {
        return jcall!(self.tiledLayerP).getCellWidth();
    }
    

    pub fn getCellHeight(&mut self) -> i32 {
        return jcall!(self.tiledLayerP).getCellHeight();
    }
    

    pub fn getCell(&mut self, mut col: i32, mut row: i32) -> i32 {
        return jcall!(self.tiledLayerP).getCell(col.clone(), row.clone());
    }
    

    pub fn getColumns(&mut self) -> i32 {
        return jcall!(self.tiledLayerP).getColumns();
    }
    

    pub fn getRows(&mut self) -> i32 {
        return jcall!(self.tiledLayerP).getRows();
    }
    

    pub fn setCell(&mut self, mut col: i32, mut row: i32, mut index: i32) {
        jcall!(self.tiledLayerP).setCell(col.clone(), row.clone(), index.clone());
    }
    

    pub fn getAnimatedTile(&mut self, mut animationTileIndex: i32) -> i32 {
        return jcall!(self.tiledLayerP).getAnimatedTile(animationTileIndex.clone());
    }
}
impl std::ops::Deref for AllBinaryJ2METiledLayer {
    type Target = AllBinaryTiledLayer;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryJ2METiledLayer {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn for AllBinaryJ2METiledLayer {
    fn as_AllBinaryTiledLayer(&self) -> &crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer { &self.base }
    fn as_AllBinaryTiledLayer_mut(&mut self) -> &mut crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer { &mut self.base }
    fn getCellWidth(&mut self) -> i32 { Self::getCellWidth(self) }
    fn getCellHeight(&mut self) -> i32 { Self::getCellHeight(self) }
    fn getCell(&mut self, mut col: i32, mut row: i32) -> i32 { Self::getCell(self, col, row) }
    fn getColumns(&mut self) -> i32 { Self::getColumns(self) }
    fn getRows(&mut self) -> i32 { Self::getRows(self) }
    fn setCell(&mut self, mut col: i32, mut row: i32, mut index: i32) { Self::setCell(self, col, row, index) }
    fn getAnimatedTile(&mut self, mut animationTileIndex: i32) -> i32 { Self::getAnimatedTile(self, animationTileIndex) }
}
impl crate::org::allbinary::layer::Layer::LayerDyn for AllBinaryJ2METiledLayer {
    fn as_Layer(&self) -> &crate::org::allbinary::layer::Layer::Layer { &self.base.base }
    fn as_Layer_mut(&mut self) -> &mut crate::org::allbinary::layer::Layer::Layer { &mut self.base.base }
    fn setPosition(&mut self, mut x: i32, mut y: i32, mut z: i32) { Self::setPosition(self, x, y, z) }
    fn moveDXY(&mut self, mut dx: i32, mut dy: i32) { Self::moveDXY(self, dx, dy) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for AllBinaryJ2METiledLayer {
    fn getName(&mut self) -> String { crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer::getName(crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn::as_AllBinaryTiledLayer_mut(self)) }
}


