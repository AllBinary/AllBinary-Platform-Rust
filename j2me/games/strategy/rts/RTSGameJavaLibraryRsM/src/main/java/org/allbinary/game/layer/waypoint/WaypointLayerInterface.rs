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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase;
use crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::WaypointLogHelper::WaypointLogHelper;
use super::Waypoint2LogHelper::Waypoint2LogHelper;
use super::WaypointRunnableLogHelper::WaypointRunnableLogHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait WaypointLayerInterface {
    

    // throws Exception
    fn init(&mut self, geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>);
    

    fn getMoveOutOfBuildAreaPath(&mut self, geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    // throws Exception
    fn setClosestGeographicMapCellHistory(&mut self, pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>);
    

    fn teleportTo(&mut self, geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>);
    

    // throws Exception
    fn setLoad(&mut self, resource: i32);
    

    // throws Exception
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn isSelected(&mut self) -> bool;
    

    fn shouldAddWaypointFromBuilding(&mut self) -> bool;
    

    fn isShowMoreCaptionStates(&mut self) -> bool;
    

    // throws Exception
    fn trackTo(&mut self, reason: String);
    

    // throws Exception
    fn trackToDXY(&mut self, dx: i32, dy: i32);
    

    // throws Exception
    fn buildingChase(&mut self, allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool;
    

    fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>>;
    

    fn allStop(&mut self);
    

    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool;
    

    // throws Exception
    fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>;
    

    // throws Exception
    fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>;
    

    fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>>;
    

    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>;
    

    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>>;
    

    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>>;
    

    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


