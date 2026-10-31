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
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Waypoint2LogHelper {
}

thread_local! { pub static Waypoint2LogHelper_instance: Option<Rc<RefCell<dyn Waypoint2LogHelperDyn>>> = Waypoint2LogHelper::new().map(|reference| reference as Rc<RefCell<dyn Waypoint2LogHelperDyn>>); }

impl Waypoint2LogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn Waypoint2LogHelperDyn>>> {
        return Waypoint2LogHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn initRange(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut closeRange: i32, mut sensorRange: i32) {
    }
    

    pub fn processPossibleTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32, mut isShorterThanCurrentTargetDistance: bool, mut isCurrentTargetDestroyed: bool) {
    }
    

    pub fn processSetTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
    }
    

    pub fn processPossibleTargetCloser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
    }
    

    pub fn setTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
    }
    

    pub fn setTargetPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn setTargetPathIgnoreNewPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
    }
    

    pub fn processWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut targetLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut size: i32) {
    }
    

    pub fn processWaypointTracked(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
    }
    

    pub fn processWaypointTrackedVisit(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
    

    pub fn processWaypointTrackedWithoutProgress(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) {
    }
    

    pub fn wander(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn targetDestroyed(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn processTargeting(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) {
    }
    

    pub fn processTargetingNonWayPoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) {
    }
    

    pub fn removeWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) {
    }
    

    pub fn removeWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn removeWaypointClear(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn clearTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn addWaypointFromUser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn targetMovedSoRetarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
}
pub trait Waypoint2LogHelperDyn {
    fn as_Waypoint2LogHelper(&self) -> &Waypoint2LogHelper;
    fn as_Waypoint2LogHelper_mut(&mut self) -> &mut Waypoint2LogHelper;
    fn initRange(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut closeRange: i32, mut sensorRange: i32) { Waypoint2LogHelper::initRange(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, closeRange, sensorRange) }
    fn processPossibleTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32, mut isShorterThanCurrentTargetDistance: bool, mut isCurrentTargetDestroyed: bool) { Waypoint2LogHelper::processPossibleTarget(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, layerInterface, anotherTargetDistance, isShorterThanCurrentTargetDistance, isCurrentTargetDestroyed) }
    fn processSetTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Waypoint2LogHelper::processSetTarget(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, layerInterface, anotherTargetDistance) }
    fn processPossibleTargetCloser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Waypoint2LogHelper::processPossibleTargetCloser(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, layerInterface, anotherTargetDistance) }
    fn setTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Waypoint2LogHelper::setTarget(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, layerInterface, anotherTargetDistance) }
    fn setTargetPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::setTargetPath(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn setTargetPathIgnoreNewPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { Waypoint2LogHelper::setTargetPathIgnoreNewPath(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2) }
    fn processWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut targetLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut size: i32) { Waypoint2LogHelper::processWaypoint(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, targetLayer, size) }
    fn processWaypointTracked(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { Waypoint2LogHelper::processWaypointTracked(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2) }
    fn processWaypointTrackedVisit(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Waypoint2LogHelper::processWaypointTrackedVisit(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, geographicMapCellPosition) }
    fn processWaypointTrackedWithoutProgress(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) { Waypoint2LogHelper::processWaypointTrackedWithoutProgress(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, reason) }
    fn wander(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::wander(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn targetDestroyed(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::targetDestroyed(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn processTargeting(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) { Waypoint2LogHelper::processTargeting(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, dx, dy) }
    fn processTargetingNonWayPoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) { Waypoint2LogHelper::processTargetingNonWayPoint(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, dx, dy) }
    fn removeWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) { Waypoint2LogHelper::removeWaypoint(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, waypointLayer, reason) }
    fn removeWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Waypoint2LogHelper::removeWaypointList(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior2, list) }
    fn removeWaypointClear(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::removeWaypointClear(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn clearTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::clearTarget(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn addWaypointFromUser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::addWaypointFromUser(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn targetMovedSoRetarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Waypoint2LogHelper::targetMovedSoRetarget(self.as_Waypoint2LogHelper_mut(), associatedAdvancedRTSGameLayer) }
}
impl Waypoint2LogHelperDyn for Waypoint2LogHelper {
    fn as_Waypoint2LogHelper(&self) -> &Waypoint2LogHelper { self }
    fn as_Waypoint2LogHelper_mut(&mut self) -> &mut Waypoint2LogHelper { self }
}


