// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::LayerPositionFinderInterface::LayerPositionFinderInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::drop::DropCellPositionHistory::DropCellPositionHistory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRTSGameLayerPositionFinder {
}

thread_local! { pub static NullRTSGameLayerPositionFinder_instance: Option<Rc<RefCell<NullRTSGameLayerPositionFinder>>> = NullRTSGameLayerPositionFinder::new(); }
impl LayerPositionFinderInterface for NullRTSGameLayerPositionFinder {
    fn getLayerInterface(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> { Self::getLayerInterface(self, geographicMapCellPosition) }
}

impl NullRTSGameLayerPositionFinder {
    

    pub fn getInstance() -> Option<Rc<RefCell<NullRTSGameLayerPositionFinder>>> {
        return NullRTSGameLayerPositionFinder_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLayerInterface(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> {
        let mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = jcall!(DropCellPositionHistory::getInstance()).getLayerInterface(geographicMapCellPosition.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::CellPosition::CellPositionDyn>>));
        return layerInterface.clone();
    }
}

