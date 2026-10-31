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
use crate::org::allbinary::game::displayable::canvas::AllBinaryGameCanvasInterface::AllBinaryGameCanvasInterface;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use super::GeographicMapCellPosition::GeographicMapCellPosition;
use super::GeographicMapCellType::GeographicMapCellType;
use super::BasicGeographicMap::BasicGeographicMap;
use super::GeographicMapCompositeInterface::GeographicMapCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GeographicMapCellPositionTracking {
    pub currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub previousGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub newGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub currentCellType: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>>,
}


impl GeographicMapCellPositionTracking {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            currentGeographicMapCellPosition: None,
            previousGeographicMapCellPosition: None,
            newGeographicMapCellPosition: None,
            currentCellType: None,
        };
        this.currentGeographicMapCellPosition = SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.previousGeographicMapCellPosition = SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.newGeographicMapCellPosition = SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.currentCellType = GeographicMapCellType_NULL_GEOGRAPHIC_MAP_CELL_TYPE.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getCurrentOverCellType(&mut self, mut geographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellType::GeographicMapCellTypeDyn>>> {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = self.currentGeographicMapCellPosition.clone();
        if !match (&geographicMapCellPosition, &self.previousGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.currentCellType = jcall!(geographicMap).getCellTypeAt(geographicMapCellPosition.clone());
        }
        return self.currentCellType.clone();
    }
    

    // throws Exception
    pub fn updateStart(&mut self, mut gameCanvasInterface: Option<Rc<RefCell<dyn AllBinaryGameCanvasInterface>>>, mut layer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>) -> bool {
        if !match (&gameCanvasInterface, &SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut geographicMapCompositeInterface: Option<Rc<RefCell<dyn GeographicMapCompositeInterface>>> = (jcall!(gameCanvasInterface).getLayerManager()).clone() /* as GeographicMapCompositeInterface */;
            let mut geographicMapInterface: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::BasicGeographicMap::BasicGeographicMapDyn>>> = jcall!(geographicMapCompositeInterface).getGeographicMapInterface()[(0) as usize].clone();
            self.newGeographicMapCellPosition = jcall!(geographicMapInterface).getCellPositionAtXY(jcall!(layer).getXP() + jcall!(layer).getHalfWidth(), jcall!(layer).getYP() + jcall!(layer).getHalfHeight());
            if !match (&self.newGeographicMapCellPosition, &self.currentGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.currentGeographicMapCellPosition = self.newGeographicMapCellPosition.clone();
                return true;
            }
        }
        return false;
    }
    

    pub fn updateEnd(&mut self) -> bool {
        if !match (&self.currentGeographicMapCellPosition, &self.previousGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.previousGeographicMapCellPosition = self.currentGeographicMapCellPosition.clone();
            return true;
        }
        return false;
    }
    

    pub fn getPreviousGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.previousGeographicMapCellPosition.clone();
    }
    

    pub fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.currentGeographicMapCellPosition.clone();
    }
}

