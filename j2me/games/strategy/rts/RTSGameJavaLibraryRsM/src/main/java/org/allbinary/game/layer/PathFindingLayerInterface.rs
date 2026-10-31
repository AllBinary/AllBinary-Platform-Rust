// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::game::combat::destroy::DestroyableInterface::DestroyableInterface;
use crate::org::allbinary::game::identification::GroupInterfaceCompositeInterface::GroupInterfaceCompositeInterface;
use crate::org::allbinary::game::input::GameKeyEventSourceInterface::GameKeyEventSourceInterface;
use crate::org::allbinary::game::layer::waypoint::WaypointLayerInterface::WaypointLayerInterface;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use super::SelectionHudPaintable::SelectionHudPaintable;
use super::RTSLayer2LogHelper::RTSLayer2LogHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait PathFindingLayerInterface: WaypointLayerInterface + GameKeyEventSourceInterface + GroupInterfaceCompositeInterface + DestroyableInterface {
    

    // throws Exception
    fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn shouldHandleStartSameAsEnd(&mut self) -> bool;
    

    fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>>;
    

    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>>;
    

    // throws Exception
    fn handleCost(&mut self, ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>);
    

    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>>;
    

    fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>>;
    

    // throws Exception
    fn setTarget(&mut self, targetGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>);
}


