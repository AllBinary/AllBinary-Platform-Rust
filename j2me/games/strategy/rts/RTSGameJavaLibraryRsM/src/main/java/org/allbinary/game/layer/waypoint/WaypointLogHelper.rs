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
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointLogHelper {
}

thread_local! { pub static WaypointLogHelper_instance: Option<Rc<RefCell<dyn WaypointLogHelperDyn>>> = WaypointLogHelper::new().map(|reference| reference as Rc<RefCell<dyn WaypointLogHelperDyn>>); }

impl WaypointLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn WaypointLogHelperDyn>>> {
        return WaypointLogHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn onWaypointEvent(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn addWaypointFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn addWaypointFromBuildingList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn insertWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String) {
    }
    

    pub fn insertWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn setRandomGeographicMapCellHistoryList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn moveAwayFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn needToMove(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
    }
    

    pub fn setRandomGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn setCurrentPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingCurrentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
    

    pub fn setNextUnvisitedPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingNextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
}
pub trait WaypointLogHelperDyn {
    fn as_WaypointLogHelper(&self) -> &WaypointLogHelper;
    fn as_WaypointLogHelper_mut(&mut self) -> &mut WaypointLogHelper;
    fn onWaypointEvent(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointLogHelper::onWaypointEvent(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn addWaypointFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointLogHelper::addWaypointFromBuilding(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn addWaypointFromBuildingList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { WaypointLogHelper::addWaypointFromBuildingList(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, advancedRTSGameLayer, list) }
    fn insertWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String) { WaypointLogHelper::insertWaypoint(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, index, rtsLayer, waypointName) }
    fn insertWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { WaypointLogHelper::insertWaypointList(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, index, rtsLayer, waypointName, list) }
    fn setRandomGeographicMapCellHistoryList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { WaypointLogHelper::setRandomGeographicMapCellHistoryList(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, pathsList) }
    fn moveAwayFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointLogHelper::moveAwayFromBuilding(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn needToMove(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { WaypointLogHelper::needToMove(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, unitWaypointBehavior) }
    fn setRandomGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointLogHelper::setRandomGeographicMapCellHistory(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn setCurrentPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingCurrentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { WaypointLogHelper::setCurrentPathGeographicMapCellPosition(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, existingCurrentPathGeographicMapCellPosition, currentPathGeographicMapCellPosition) }
    fn setNextUnvisitedPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingNextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { WaypointLogHelper::setNextUnvisitedPathGeographicMapCellPosition(self.as_WaypointLogHelper_mut(), associatedAdvancedRTSGameLayer, existingNextUnvisitedPathGeographicMapCellPosition, nextUnvisitedPathGeographicMapCellPosition) }
}
impl WaypointLogHelperDyn for WaypointLogHelper {
    fn as_WaypointLogHelper(&self) -> &WaypointLogHelper { self }
    fn as_WaypointLogHelper_mut(&mut self) -> &mut WaypointLogHelper { self }
}


