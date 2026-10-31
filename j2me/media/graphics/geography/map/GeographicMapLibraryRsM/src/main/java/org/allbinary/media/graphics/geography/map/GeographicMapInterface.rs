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
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::layer::Layer::Layer;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::GeographicMapCellType::GeographicMapCellType;
use super::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactory;
use super::GeographicMapCellPositionFactoryInterface::GeographicMapCellPositionFactoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GeographicMapInterface {
    

    fn getId(&mut self) -> Option<i32>;
    

    fn getName(&mut self) -> String;
    

    fn reset(&mut self);
    

    fn getAllBinaryTiledLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>;
    

    // throws Exception
    fn getCellPositionsAt(&mut self, layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::Layer::LayerDyn>>>, currentCellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>, cellPositionArray: Vec<Vec<Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>>>) -> bool;
    

    // throws Exception
    fn getCellPositionAtXY(&mut self, x: i32, y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>;
    

    // throws Exception
    fn getCellPositionAtXYNoThrow(&mut self, x: i32, y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>;
    

    fn getCellTypeFromMapCellTypeInt(&mut self, cellTypeId: i32) -> i32;
    

    // throws Exception
    fn getCellTypeAtXY(&mut self, x: i32, y: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>;
    

    // throws Exception
    fn getCellTypeAt(&mut self, geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>;
    

    fn getGeographicMapCellPositionFactory(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMapCellPositionFactory::BasicGeographicMapCellPositionFactoryDyn>>>;
    

    fn getGeographicMapCellPositionFactoryInterface(&mut self) -> Option<Rc<RefCell<dyn GeographicMapCellPositionFactoryInterface>>>;
    

    fn getForegroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>>;
    

    fn getBackgroundBasicColor(&mut self) -> Option<Rc<RefCell<BasicColor>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


