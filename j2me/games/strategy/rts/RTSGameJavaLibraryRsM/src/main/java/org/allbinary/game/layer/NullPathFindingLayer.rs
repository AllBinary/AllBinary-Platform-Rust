// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase;
use crate::org::allbinary::game::identification::Group::Group;
use crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelper;
use crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelper;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::PathFindingLayerInterface::PathFindingLayerInterface;
use super::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBase;
use super::SelectionHudPaintable::SelectionHudPaintable;
use super::RTSLayer2LogHelper::RTSLayer2LogHelper;
use super::WaypointBehaviorBase::WaypointBehaviorBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullPathFindingLayer {
}

thread_local! { pub static NullPathFindingLayer_NULL_PATH_FINDING_LAYER: Option<Rc<RefCell<dyn NullPathFindingLayerDyn>>> = NullPathFindingLayer::new().map(|reference| reference as Rc<RefCell<dyn NullPathFindingLayerDyn>>); }
// impl PathFindingLayerInterface for NullPathFindingLayer with the methods of the trait

impl NullPathFindingLayer {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn shouldHandleStartSameAsEnd(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setTarget(&mut self, mut targetGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setLoad(&mut self, mut resource: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isSelected(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn shouldAddWaypointFromBuilding(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isShowMoreCaptionStates(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn trackTo(&mut self, mut reason: String) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn allStop(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn isDestroyed(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getSourceId(&mut self) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait NullPathFindingLayerDyn {
    fn as_NullPathFindingLayer(&self) -> &NullPathFindingLayer;
    fn as_NullPathFindingLayer_mut(&mut self) -> &mut NullPathFindingLayer;
    fn getName(&mut self) -> String { NullPathFindingLayer::getName(self.as_NullPathFindingLayer_mut()) }
    fn getEndGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { NullPathFindingLayer::getEndGeographicMapCellPositionList(self.as_NullPathFindingLayer_mut()) }
    fn shouldHandleStartSameAsEnd(&mut self) -> bool { NullPathFindingLayer::shouldHandleStartSameAsEnd(self.as_NullPathFindingLayer_mut()) }
    fn getGeographicMapCellPositionArea(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::GeographicMapCellPositionAreaBase::GeographicMapCellPositionAreaBaseDyn>>> { NullPathFindingLayer::getGeographicMapCellPositionArea(self.as_NullPathFindingLayer_mut()) }
    fn getHudPaintable(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn>>> { NullPathFindingLayer::getHudPaintable(self.as_NullPathFindingLayer_mut()) }
    fn handleCost(&mut self, mut ownerLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { NullPathFindingLayer::handleCost(self.as_NullPathFindingLayer_mut(), ownerLayer) }
    fn getParentLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { NullPathFindingLayer::getParentLayer(self.as_NullPathFindingLayer_mut()) }
    fn getRTSLayer2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn>>> { NullPathFindingLayer::getRTSLayer2LogHelper(self.as_NullPathFindingLayer_mut()) }
    fn setTarget(&mut self, mut targetGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { NullPathFindingLayer::setTarget(self.as_NullPathFindingLayer_mut(), targetGameLayer) }
    fn init(&mut self, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { NullPathFindingLayer::init(self.as_NullPathFindingLayer_mut(), geographicMapCellHistory, geographicMapCellPositionBasicArrayList) }
    fn getMoveOutOfBuildAreaPath(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { NullPathFindingLayer::getMoveOutOfBuildAreaPath(self.as_NullPathFindingLayer_mut(), geographicMapCellPosition) }
    fn setClosestGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { NullPathFindingLayer::setClosestGeographicMapCellHistory(self.as_NullPathFindingLayer_mut(), pathsList) }
    fn teleportTo(&mut self, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { NullPathFindingLayer::teleportTo(self.as_NullPathFindingLayer_mut(), geographicMapCellPosition) }
    fn setLoad(&mut self, mut resource: i32) { NullPathFindingLayer::setLoad(self.as_NullPathFindingLayer_mut(), resource) }
    fn getSurroundingGeographicMapCellPositionList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { NullPathFindingLayer::getSurroundingGeographicMapCellPositionList(self.as_NullPathFindingLayer_mut()) }
    fn isSelected(&mut self) -> bool { NullPathFindingLayer::isSelected(self.as_NullPathFindingLayer_mut()) }
    fn shouldAddWaypointFromBuilding(&mut self) -> bool { NullPathFindingLayer::shouldAddWaypointFromBuilding(self.as_NullPathFindingLayer_mut()) }
    fn isShowMoreCaptionStates(&mut self) -> bool { NullPathFindingLayer::isShowMoreCaptionStates(self.as_NullPathFindingLayer_mut()) }
    fn trackTo(&mut self, mut reason: String) { NullPathFindingLayer::trackTo(self.as_NullPathFindingLayer_mut(), reason) }
    fn trackToDXY(&mut self, mut dx: i32, mut dy: i32) { NullPathFindingLayer::trackToDXY(self.as_NullPathFindingLayer_mut(), dx, dy) }
    fn buildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) -> bool { NullPathFindingLayer::buildingChase(self.as_NullPathFindingLayer_mut(), allbinaryLayer, cellPosition) }
    fn getTrackingEvent(&mut self) -> Option<Rc<RefCell<TrackingEvent>>> { NullPathFindingLayer::getTrackingEvent(self.as_NullPathFindingLayer_mut()) }
    fn allStop(&mut self) { NullPathFindingLayer::allStop(self.as_NullPathFindingLayer_mut()) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { NullPathFindingLayer::isWaypointListEmptyOrOnlyTargets(self.as_NullPathFindingLayer_mut()) }
    fn getCurrentGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { NullPathFindingLayer::getCurrentGeographicMapCellPosition(self.as_NullPathFindingLayer_mut()) }
    fn getTopLeftGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { NullPathFindingLayer::getTopLeftGeographicMapCellPosition(self.as_NullPathFindingLayer_mut()) }
    fn getCaptionAnimationHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn>>> { NullPathFindingLayer::getCaptionAnimationHelper(self.as_NullPathFindingLayer_mut()) }
    fn getWaypointBehavior(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> { NullPathFindingLayer::getWaypointBehavior(self.as_NullPathFindingLayer_mut()) }
    fn getWaypointLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn>>> { NullPathFindingLayer::getWaypointLogHelper(self.as_NullPathFindingLayer_mut()) }
    fn getWaypoint2LogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> { NullPathFindingLayer::getWaypoint2LogHelper(self.as_NullPathFindingLayer_mut()) }
    fn getWaypointRunnableLogHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointRunnableLogHelper::WaypointRunnableLogHelperDyn>>> { NullPathFindingLayer::getWaypointRunnableLogHelper(self.as_NullPathFindingLayer_mut()) }
    fn getGroupInterface(&mut self) -> Vec<Option<Rc<RefCell<Group>>>> { NullPathFindingLayer::getGroupInterface(self.as_NullPathFindingLayer_mut()) }
    fn isDestroyed(&mut self) -> bool { NullPathFindingLayer::isDestroyed(self.as_NullPathFindingLayer_mut()) }
    fn getSourceId(&mut self) -> i32 { NullPathFindingLayer::getSourceId(self.as_NullPathFindingLayer_mut()) }
}
impl NullPathFindingLayerDyn for NullPathFindingLayer {
    fn as_NullPathFindingLayer(&self) -> &NullPathFindingLayer { self }
    fn as_NullPathFindingLayer_mut(&mut self) -> &mut NullPathFindingLayer { self }
}


