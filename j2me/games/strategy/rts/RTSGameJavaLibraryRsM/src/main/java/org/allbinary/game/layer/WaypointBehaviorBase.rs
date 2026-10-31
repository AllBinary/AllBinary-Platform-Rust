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
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::game::tick::TickableInterface::TickableInterface;
use crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManager;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayListS::BasicArrayListS;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointBehaviorBase {
    pub ownedWaypointList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub waypointOverridesAttacking: bool,
    pub waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>>,
    pub steeringVisitorList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

impl TickableInterface for WaypointBehaviorBase {
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { Self::processTick(self, allBinaryLayerManager) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for WaypointBehaviorBase {
    fn getName(&mut self) -> String { Self::getName(self) }
}

impl WaypointBehaviorBase {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            ownedWaypointList: None,
            waypointOverridesAttacking: false,
            waypoint: None,
            steeringVisitorList: None,
        };
        this.waypointOverridesAttacking = true;
        this.waypoint = unsafe { crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBase_NULL_WAYPOINT_BASE.clone() }.clone();
        this.steeringVisitorList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.ownedWaypointList = BasicArrayListS::new(1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getName(&mut self) -> String {
        return std::any::type_name_of_val(&self).to_string();
    }
    

    pub fn isRunning(&mut self) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) {
    }
    

    pub fn getOwnedWaypointList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.ownedWaypointList.clone();
    }
    

    pub fn getWaypoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> {
        return self.waypoint.clone();
    }
    

    pub fn setWaypoint(&mut self, mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>>) {
        self.waypoint = waypoint.clone();
    }
    

    pub fn getSteeringVisitorList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.steeringVisitorList.clone();
    }
    

    pub fn getMovementLogicAsString(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isInSensorRange(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut targetDistance: i32) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCurrentTargetLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCurrentTargetingStateString(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isTrackingWaypoint(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCurrentPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getCurrentGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<GeographicMapCellHistory>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn isWaypointOverridesAttacking(&mut self) -> bool {
        return self.waypointOverridesAttacking.clone();
    }
    

    pub fn getNextUnvisitedPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn needToMove(&mut self) -> bool {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn setWaypointPathsList(&mut self, mut waypointPathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getWaypointPathsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setTarget(&mut self, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn setTargetWithDistance(&mut self, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn clearTarget(&mut self) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn updatePathOnTargetMove(&mut self, mut reason: String) {
    }
}
pub trait WaypointBehaviorBaseDyn {
    fn as_WaypointBehaviorBase(&self) -> &WaypointBehaviorBase;
    fn as_WaypointBehaviorBase_mut(&mut self) -> &mut WaypointBehaviorBase;
    fn getName(&mut self) -> String { WaypointBehaviorBase::getName(self.as_WaypointBehaviorBase_mut()) }
    fn isRunning(&mut self) -> bool { WaypointBehaviorBase::isRunning(self.as_WaypointBehaviorBase_mut()) }
    fn processTick(&mut self, mut allBinaryLayerManager: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayerManager::AllBinaryLayerManagerDyn>>>) { WaypointBehaviorBase::processTick(self.as_WaypointBehaviorBase_mut(), allBinaryLayerManager) }
    fn getOwnedWaypointList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBehaviorBase::getOwnedWaypointList(self.as_WaypointBehaviorBase_mut()) }
    fn getWaypoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>> { WaypointBehaviorBase::getWaypoint(self.as_WaypointBehaviorBase_mut()) }
    fn setWaypoint(&mut self, mut waypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointBase::WaypointBaseDyn>>>) { WaypointBehaviorBase::setWaypoint(self.as_WaypointBehaviorBase_mut(), waypoint) }
    fn getSteeringVisitorList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBehaviorBase::getSteeringVisitorList(self.as_WaypointBehaviorBase_mut()) }
    fn getMovementLogicAsString(&mut self) -> String { WaypointBehaviorBase::getMovementLogicAsString(self.as_WaypointBehaviorBase_mut()) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { WaypointBehaviorBase::isWaypointListEmptyOrOnlyTargets(self.as_WaypointBehaviorBase_mut()) }
    fn isInSensorRange(&mut self, mut layerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut targetDistance: i32) -> bool { WaypointBehaviorBase::isInSensorRange(self.as_WaypointBehaviorBase_mut(), layerInterface, targetDistance) }
    fn getCurrentTargetLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> { WaypointBehaviorBase::getCurrentTargetLayerInterface(self.as_WaypointBehaviorBase_mut()) }
    fn getCurrentTargetingStateString(&mut self) -> String { WaypointBehaviorBase::getCurrentTargetingStateString(self.as_WaypointBehaviorBase_mut()) }
    fn isTrackingWaypoint(&mut self) -> bool { WaypointBehaviorBase::isTrackingWaypoint(self.as_WaypointBehaviorBase_mut()) }
    fn getCurrentPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { WaypointBehaviorBase::getCurrentPathGeographicMapCellPosition(self.as_WaypointBehaviorBase_mut()) }
    fn getCurrentGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<GeographicMapCellHistory>>> { WaypointBehaviorBase::getCurrentGeographicMapCellHistory(self.as_WaypointBehaviorBase_mut()) }
    fn isWaypointOverridesAttacking(&mut self) -> bool { WaypointBehaviorBase::isWaypointOverridesAttacking(self.as_WaypointBehaviorBase_mut()) }
    fn getNextUnvisitedPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { WaypointBehaviorBase::getNextUnvisitedPathGeographicMapCellPosition(self.as_WaypointBehaviorBase_mut()) }
    fn needToMove(&mut self) -> bool { WaypointBehaviorBase::needToMove(self.as_WaypointBehaviorBase_mut()) }
    fn setWaypointPathsList(&mut self, mut waypointPathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { WaypointBehaviorBase::setWaypointPathsList(self.as_WaypointBehaviorBase_mut(), waypointPathsList) }
    fn getWaypointPathsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { WaypointBehaviorBase::getWaypointPathsList(self.as_WaypointBehaviorBase_mut()) }
    fn setTarget(&mut self, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointBehaviorBase::setTarget(self.as_WaypointBehaviorBase_mut(), layerInterface) }
    fn setTargetWithDistance(&mut self, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { WaypointBehaviorBase::setTargetWithDistance(self.as_WaypointBehaviorBase_mut(), layerInterface, anotherTargetDistance) }
    fn clearTarget(&mut self) { WaypointBehaviorBase::clearTarget(self.as_WaypointBehaviorBase_mut()) }
    fn updatePathOnTargetMove(&mut self, mut reason: String) { WaypointBehaviorBase::updatePathOnTargetMove(self.as_WaypointBehaviorBase_mut(), reason) }
}
impl WaypointBehaviorBaseDyn for WaypointBehaviorBase {
    fn as_WaypointBehaviorBase(&self) -> &WaypointBehaviorBase { self }
    fn as_WaypointBehaviorBase_mut(&mut self) -> &mut WaypointBehaviorBase { self }
}


