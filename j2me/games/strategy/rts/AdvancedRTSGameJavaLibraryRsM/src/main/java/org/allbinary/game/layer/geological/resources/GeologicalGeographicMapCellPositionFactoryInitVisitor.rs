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
// package org.allbinary.game.layer.geological.resources
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayer;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionFactoryInitVisitorInterface::GeographicMapCellPositionFactoryInitVisitorInterface;
use super::GeologicalGeographicMapCellPosition::GeologicalGeographicMapCellPosition;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeologicalGeographicMapCellPositionFactoryInitVisitor {
}

impl GeographicMapCellPositionFactoryInitVisitorInterface for GeologicalGeographicMapCellPositionFactoryInitVisitor {
    fn visit(&mut self, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::visit(self, tiledLayer, cellPosition) }
}

impl GeologicalGeographicMapCellPositionFactoryInitVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn visit(&mut self, mut tiledLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AllBinaryTiledLayer::AllBinaryTiledLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut geologicalGeographicMapCellPosition: Option<Rc<RefCell<GeologicalGeographicMapCellPosition>>> = ((cellPosition).clone() /* as GeologicalGeographicMapCellPosition */);
        jcall!(jcall!(geologicalGeographicMapCellPosition).getGeologicalResource()).init();
    }
}

