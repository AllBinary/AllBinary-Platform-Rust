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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::layer::LayerInterface::LayerInterface;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPositionTracking::GeographicMapCellPositionTracking;
use super::VehiclePropertiesCompositeInterface::VehiclePropertiesCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait VehicleLayerInterface: LayerInterface + VehiclePropertiesCompositeInterface {
    

    fn getGameKeyEventList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn getSpeedBasicDecimal(&mut self) -> Option<Rc<RefCell<BasicDecimal>>>;
    

    fn isReadyForExplosion(&mut self) -> bool;
    

    fn getRotationAnimationInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>;
    

    fn setRotationAnimationInterface(&mut self, animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>);
    

    fn isDestroyed(&mut self) -> bool;
    

    fn isFinish(&mut self) -> bool;
    

    fn getFinalPosition(&mut self) -> i32;
    

    // throws Exception
    fn handleFinish(&mut self);
    

    fn getGeographicMapCellPositionTracking(&mut self) -> Option<Rc<RefCell<GeographicMapCellPositionTracking>>>;
    

    fn getGeographicMapCellHistoryArray(&mut self) -> Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>;
    

    fn setGeographicMapCellHistoryArray(&mut self, geographicMapCellHistory: Vec<Option<Rc<RefCell<GeographicMapCellHistory>>>>);
}


