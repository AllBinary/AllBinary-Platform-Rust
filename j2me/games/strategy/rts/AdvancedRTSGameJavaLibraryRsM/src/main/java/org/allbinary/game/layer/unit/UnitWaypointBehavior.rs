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
// package org.allbinary.game.layer.unit
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::input::form::PrimaryWaypointHelper::PrimaryWaypointHelper;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::SensorAction::SensorAction;
use crate::org::allbinary::game::layer::SensorActionFactory::SensorActionFactory;
use crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase;
use crate::org::allbinary::game::layer::building::BuildingLayer::BuildingLayer;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::layer::waypoint::event::WaypointEventListenerInterface::WaypointEventListenerInterface;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::logic::util::event::EventStrings::EventStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use super::UnitLayer::UnitLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UnitWaypointBehavior {
    pub base: WaypointBehaviorBase,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub longWeaponRange: i32,
    pub sensorAction: Option<Rc<RefCell<SensorAction>>>,
    pub completeTimeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub currentGeographicMapCellHistoryP: Option<Rc<RefCell<GeographicMapCellHistory>>>,
    pub lastPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>,
    pub FAKE_WAYPOINT_LAYER: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub targetList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub possibleTargetList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub associatedAdvancedRTSGameLayer: Option<Rc<RefCell<UnitLayer>>>,
    pub moving: bool,
    pub movingFromStopped: bool,
    pub waypointPathsListP: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub currentTargetDistance: i32,
    pub currentTargetLayerInterfaceP: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub trackingWaypoint: bool,
    pub repeatedToLong: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
}

thread_local! { pub static UnitWaypointBehavior_PATHING: String = String::from("Pathing"); }
impl WaypointEventListenerInterface for UnitWaypointBehavior {
    fn onWaypointEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { Self::onWaypointEvent(self, event) }
}
impl crate::org::allbinary::logic::util::event::EventListenerInterface::EventListenerInterface for UnitWaypointBehavior {
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { Self::onEvent(self, eventObject) }
}
impl crate::org::allbinary::layer::NamedInterface::NamedInterface for UnitWaypointBehavior {
    fn getName(&mut self) -> String { self.base.getName() }
}

impl UnitWaypointBehavior {
    

    pub fn new_struct(mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<UnitLayer>>>, mut fakeWaypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> Self {
        let mut this = Self {
            base: WaypointBehaviorBase::new_struct(),
            commonSeps: None,
            longWeaponRange: 0,
            sensorAction: None,
            completeTimeDelayHelper: None,
            currentGeographicMapCellHistoryP: None,
            lastPathGeographicMapCellPosition: None,
            currentPathGeographicMapCellPosition: None,
            FAKE_WAYPOINT_LAYER: None,
            targetList: None,
            possibleTargetList: None,
            associatedAdvancedRTSGameLayer: None,
            moving: false,
            movingFromStopped: false,
            waypointPathsListP: None,
            currentTargetDistance: 0,
            currentTargetLayerInterfaceP: None,
            trackingWaypoint: false,
            repeatedToLong: None,
        };
        this.commonSeps = CommonSeps::getInstance();
        this.longWeaponRange = 0;
        this.sensorAction = jcall!(SensorActionFactory::getInstance()).ATTACK.clone();
        this.lastPathGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.currentPathGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        this.moving = false;
        this.movingFromStopped = false;
        this.waypointPathsListP = jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance();
        this.currentTargetDistance = i32::MAX.clone();
        this.currentTargetLayerInterfaceP = CollidableDestroyableDamageableLayer::getNullInstance();
        this.repeatedToLong = TimeDelayHelper::new(22000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.associatedAdvancedRTSGameLayer = associatedAdvancedRTSGameLayer.clone();
        this.completeTimeDelayHelper = TimeDelayHelper::new(30000).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.targetList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.possibleTargetList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        { let java_arguments = (jcall!(BasicArrayListUtil::getInstance()).getImmutableInstance(),); this.setWaypointPathsList(java_arguments.0) };
        this.currentGeographicMapCellHistoryP = GeographicMapCellHistory::new();
        this.FAKE_WAYPOINT_LAYER = fakeWaypoint.clone();
        this
    }

    pub fn new(mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<UnitLayer>>>, mut fakeWaypoint: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(associatedAdvancedRTSGameLayer, fakeWaypoint))))
    }
    

    pub fn initRange(&mut self, mut weaponRange: i32) {
        self.longWeaponRange = weaponRange / 2;
    }
    

    pub fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) {
        ForcedLogUtil::log(jcall!(EventStrings::getInstance()).PERFORMANCE_MESSAGE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    // throws Exception
    pub fn onWaypointEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) {
        let mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>> = (jcall!(event).getRtsLayer()).clone() /* as AdvancedRTSGameLayer */;
        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).onWaypointEvent(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), advancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>));
        if jcall!(self.associatedAdvancedRTSGameLayer).isSelected() {
            self.addWaypointFromUser(advancedRTSGameLayer.clone());
        } else if jcall!(advancedRTSGameLayer).shouldAddWaypointFromBuilding() {
            self.addWaypointFromBuilding(advancedRTSGameLayer.clone());
        }
    }
    

    // throws Exception
    pub fn addWaypointFromUser(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
    }
    

    // throws Exception
    pub fn addWaypointFromBuilding(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        if match (&advancedRTSGameLayer, &jcall!(PrimaryWaypointHelper::getInstance()).getWaypointLayer()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || match (&jcall!(advancedRTSGameLayer).getParentLayer(), &jcall!(self.associatedAdvancedRTSGameLayer).getParentLayer()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            if !jcall!(self.targetList).contains(crate::java_into_object(advancedRTSGameLayer.clone())) {
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).addWaypointFromBuilding(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), advancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>));
                if jcall!(advancedRTSGameLayer).isDestroyed() {
                    panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Trying to add a dead: "), advancedRTSGameLayer))));
                }
                jcall!(self.targetList).add(crate::java_into_object(advancedRTSGameLayer.clone()));
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).addWaypointFromBuildingList(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), advancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), self.targetList.clone());
            }
        }
    }
    

    pub fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool {
        return false;
    }
    

    // throws Exception
    pub fn insertWaypoint(&mut self, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> bool {
        if self.canInsertWaypoint(index.clone(), rtsLayer.clone()) {
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).insertWaypoint(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), index.clone(), rtsLayer.clone(), self.getName());
            if jcall!(rtsLayer).isDestroyed() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (format!("{}{}", String::from("Trying to add a dead: "), rtsLayer))));
            }
            jcall!(self.targetList).addAt(index.clone(), crate::java_into_object(rtsLayer.clone()));
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).insertWaypointList(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), index.clone(), rtsLayer.clone(), self.getName(), self.targetList.clone());
            return true;
        }
        return false;
    }
    

    pub fn r#move(&mut self) {
        if self.isMoving() {
            self.setMovingFromStopped(false);
        } else {
            self.setMovingFromStopped(true);
        }
        self.setMoving(false);
    }
    

    // throws Exception
    pub fn setRandomGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(pathsList).size();
        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).setRandomGeographicMapCellHistoryList(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), pathsList.clone());
        if size > 0 {
            let mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = { let java_object = (jcall!(BasicArrayListUtil::getInstance()).getRandom(pathsList.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>>().expect("ClassCastException").clone()) };
            self.setGeographicMapCellHistoryPath(geographicMapCellPositionBasicArrayList.clone());
        }
    }
    

    // throws Exception
    pub fn setGeographicMapCellHistoryPath(&mut self, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.lastPathGeographicMapCellPosition = crate::org::allbinary::media::graphics::geography::map::SimpleGeographicMapCellPositionFactory::SimpleGeographicMapCellPositionFactory_NULL_GEOGRAPHIC_MAP_CELL_POSITION.with(Clone::clone).clone();
        if jcall!(self.associatedAdvancedRTSGameLayer).showMoreCaptionStates {
            jcall!(jcall!(self.associatedAdvancedRTSGameLayer).getCaptionAnimationHelper()).update(UnitWaypointBehavior_PATHING.with(Clone::clone).clone(), jcall!(BasicColorFactory::getInstance()).GREEN.clone());
        }
        jcall!(self.currentGeographicMapCellHistoryP).init();
        jcall!(self.associatedAdvancedRTSGameLayer).init(self.currentGeographicMapCellHistoryP.clone(), geographicMapCellPositionBasicArrayList.clone());
        self.setTrackingWaypoint(true);
        jcall!(self.getCompleteTimeDelayHelper()).setStartTimeTNT();
    }
    

    pub fn canInsertWaypoint(&mut self, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> bool {
        if jcall!(self.targetList).size() > 4 {
        } else if jcall!(self.targetList).contains(crate::java_into_object(rtsLayer.clone())) {
        } else {
            return true;
        }
        return false;
    }
    

    // throws Exception
    pub fn addBuildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
    }
    

    // throws Exception
    pub fn moveAwayFromBuilding(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) {
        let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.associatedAdvancedRTSGameLayer).getCurrentGeographicMapCellPosition();
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(buildingLayer).geographicMapCellPositionAreaBase).getOccupyingGeographicMapCellPositionList();
        if jcall!(list).contains(crate::java_into_object(geographicMapCellPosition.clone())) {
            if self.insertWaypoint(0, self.FAKE_WAYPOINT_LAYER.clone()) {
                jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).moveAwayFromBuilding(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>));
                self.setCurrentTargetLayerInterface((self.FAKE_WAYPOINT_LAYER).clone() /* as CollidableDestroyableDamageableLayer */);
                let mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(buildingLayer).getMoveOutOfBuildAreaPath(geographicMapCellPosition.clone());
                jcall!(self.associatedAdvancedRTSGameLayer).setClosestGeographicMapCellHistory(pathsList.clone());
            }
        }
    }
    

    pub fn needToMove(&mut self) -> bool {
        jcall!(jcall!(self.associatedAdvancedRTSGameLayer).waypointLogHelperP).needToMove(self.associatedAdvancedRTSGameLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface>>), None /* Java this: a &mut self method has no Rc of itself */);
        if self.isTrackingWaypoint() || match (&self.sensorAction, &jcall!(SensorActionFactory::getInstance()).EVADE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } || (!match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } && self.getCurrentTargetDistance() >= self.longWeaponRange + jcall!(self.currentTargetLayerInterfaceP).getHalfHeight()) {
            jcall!(self.repeatedToLong).setStartTimeTNT();
            return true;
        }
        if jcall!(self.repeatedToLong).isTimeTNT() {
            let mut message: String = format!("{}{}", String::from("Repeating too long: "), self.getMovementLogicAsString());
            ForcedLogUtil::log(message.clone(), self.associatedAdvancedRTSGameLayer.clone());
        }
        return false;
    }
    

    pub fn getMovementLogicAsString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("isTrackingWaypoint: "));
        jcall!(stringBuffer).appendboolean(self.isTrackingWaypoint());
        jcall!(stringBuffer).append(String::from(" sensorAction: "));
        jcall!(stringBuffer).append(jcall!(self.sensorAction).name.clone());
        jcall!(stringBuffer).append(String::from(" getCurrentTargetLayerInterface: "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(self.currentTargetLayerInterfaceP.clone())));
        if !match (&self.currentTargetLayerInterfaceP, &CollidableDestroyableDamageableLayer::getNullInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            jcall!(stringBuffer).append(String::from(" Target Range: "));
            jcall!(stringBuffer).appendint(self.getCurrentTargetDistance());
            jcall!(stringBuffer).append(String::from(" >= "));
            jcall!(stringBuffer).appendint(self.longWeaponRange + jcall!(self.currentTargetLayerInterfaceP).getHalfHeight());
        }
        return jcall!(stringBuffer).toString();
    }
    

    pub fn isMovingFromStopped(&mut self) -> bool {
        return self.movingFromStopped.clone();
    }
    

    pub fn setMovingFromStopped(&mut self, mut movingFromStopped: bool) {
        self.movingFromStopped = movingFromStopped.clone();
    }
    

    pub fn setWaypointPathsList(&mut self, mut waypointPathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        self.waypointPathsListP = waypointPathsList.clone();
    }
    

    pub fn getWaypointPathsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.waypointPathsListP.clone();
    }
    

    pub fn isMoving(&mut self) -> bool {
        return self.moving.clone();
    }
    

    pub fn setMoving(&mut self, mut moving: bool) {
        self.moving = moving.clone();
    }
    

    pub fn isTrackingWaypoint(&mut self) -> bool {
        return self.trackingWaypoint.clone();
    }
    

    pub fn getPossibleTargetList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.possibleTargetList.clone();
    }
    

    pub fn setLastPathGeographicMapCellPosition(&mut self, mut lastPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        self.lastPathGeographicMapCellPosition = lastPathGeographicMapCellPosition.clone();
    }
    

    pub fn getLastPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.lastPathGeographicMapCellPosition.clone();
    }
    

    pub fn setCurrentPathGeographicMapCellPosition(&mut self, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        self.currentPathGeographicMapCellPosition = currentPathGeographicMapCellPosition.clone();
    }
    

    pub fn getCurrentPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> {
        return self.currentPathGeographicMapCellPosition.clone();
    }
    

    pub fn setSensorAction(&mut self, mut sensorAction: Option<Rc<RefCell<SensorAction>>>) {
        self.sensorAction = sensorAction.clone();
    }
    

    pub fn getSensorAction(&mut self) -> Option<Rc<RefCell<SensorAction>>> {
        return self.sensorAction.clone();
    }
    

    pub fn setCurrentTargetLayerInterface(&mut self, mut currentTargetLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) {
        self.currentTargetLayerInterfaceP = currentTargetLayerInterface.clone();
    }
    

    pub fn getCurrentTargetLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        return self.currentTargetLayerInterfaceP.clone();
    }
    

    pub fn setCurrentTargetDistance(&mut self, mut currentTargetDistance: i32) {
        self.currentTargetDistance = currentTargetDistance.clone();
    }
    

    pub fn getCurrentTargetDistance(&mut self) -> i32 {
        return self.currentTargetDistance.clone();
    }
    

    pub fn getCompleteTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> {
        return self.completeTimeDelayHelper.clone();
    }
    

    pub fn getCurrentGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<GeographicMapCellHistory>>> {
        return self.currentGeographicMapCellHistoryP.clone();
    }
    

    pub fn setTrackingWaypoint(&mut self, mut trackingWaypoint: bool) {
        self.trackingWaypoint = trackingWaypoint.clone();
    }
}
impl std::ops::Deref for UnitWaypointBehavior {
    type Target = WaypointBehaviorBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UnitWaypointBehavior {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait UnitWaypointBehaviorDyn: crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn {
    fn as_UnitWaypointBehavior(&self) -> &UnitWaypointBehavior;
    fn as_UnitWaypointBehavior_mut(&mut self) -> &mut UnitWaypointBehavior;
    fn initRange(&mut self, mut weaponRange: i32) { UnitWaypointBehavior::initRange(self.as_UnitWaypointBehavior_mut(), weaponRange) }
    fn onEvent(&mut self, mut eventObject: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>) { UnitWaypointBehavior::onEvent(self.as_UnitWaypointBehavior_mut(), eventObject) }
    fn onWaypointEvent(&mut self, mut event: Option<Rc<RefCell<RTSLayerEvent>>>) { UnitWaypointBehavior::onWaypointEvent(self.as_UnitWaypointBehavior_mut(), event) }
    fn addWaypointFromUser(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) { UnitWaypointBehavior::addWaypointFromUser(self.as_UnitWaypointBehavior_mut(), advancedRTSGameLayer) }
    fn addWaypointFromBuilding(&mut self, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) { UnitWaypointBehavior::addWaypointFromBuilding(self.as_UnitWaypointBehavior_mut(), advancedRTSGameLayer) }
    fn insertWaypoint(&mut self, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> bool { UnitWaypointBehavior::insertWaypoint(self.as_UnitWaypointBehavior_mut(), index, rtsLayer) }
    fn r#move(&mut self) { UnitWaypointBehavior::r#move(self.as_UnitWaypointBehavior_mut()) }
    fn setRandomGeographicMapCellHistory(&mut self, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { UnitWaypointBehavior::setRandomGeographicMapCellHistory(self.as_UnitWaypointBehavior_mut(), pathsList) }
    fn setGeographicMapCellHistoryPath(&mut self, mut geographicMapCellPositionBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { UnitWaypointBehavior::setGeographicMapCellHistoryPath(self.as_UnitWaypointBehavior_mut(), geographicMapCellPositionBasicArrayList) }
    fn canInsertWaypoint(&mut self, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) -> bool { UnitWaypointBehavior::canInsertWaypoint(self.as_UnitWaypointBehavior_mut(), index, rtsLayer) }
    fn addBuildingChase(&mut self, mut allbinaryLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>>, mut cellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { UnitWaypointBehavior::addBuildingChase(self.as_UnitWaypointBehavior_mut(), allbinaryLayer, cellPosition) }
    fn moveAwayFromBuilding(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) { UnitWaypointBehavior::moveAwayFromBuilding(self.as_UnitWaypointBehavior_mut(), buildingLayer) }
    fn isMovingFromStopped(&mut self) -> bool { UnitWaypointBehavior::isMovingFromStopped(self.as_UnitWaypointBehavior_mut()) }
    fn setMovingFromStopped(&mut self, mut movingFromStopped: bool) { UnitWaypointBehavior::setMovingFromStopped(self.as_UnitWaypointBehavior_mut(), movingFromStopped) }
    fn isMoving(&mut self) -> bool { UnitWaypointBehavior::isMoving(self.as_UnitWaypointBehavior_mut()) }
    fn setMoving(&mut self, mut moving: bool) { UnitWaypointBehavior::setMoving(self.as_UnitWaypointBehavior_mut(), moving) }
    fn getPossibleTargetList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { UnitWaypointBehavior::getPossibleTargetList(self.as_UnitWaypointBehavior_mut()) }
    fn setLastPathGeographicMapCellPosition(&mut self, mut lastPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { UnitWaypointBehavior::setLastPathGeographicMapCellPosition(self.as_UnitWaypointBehavior_mut(), lastPathGeographicMapCellPosition) }
    fn getLastPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { UnitWaypointBehavior::getLastPathGeographicMapCellPosition(self.as_UnitWaypointBehavior_mut()) }
    fn setCurrentPathGeographicMapCellPosition(&mut self, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { UnitWaypointBehavior::setCurrentPathGeographicMapCellPosition(self.as_UnitWaypointBehavior_mut(), currentPathGeographicMapCellPosition) }
    fn setSensorAction(&mut self, mut sensorAction: Option<Rc<RefCell<SensorAction>>>) { UnitWaypointBehavior::setSensorAction(self.as_UnitWaypointBehavior_mut(), sensorAction) }
    fn getSensorAction(&mut self) -> Option<Rc<RefCell<SensorAction>>> { UnitWaypointBehavior::getSensorAction(self.as_UnitWaypointBehavior_mut()) }
    fn setCurrentTargetLayerInterface(&mut self, mut currentTargetLayerInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>) { UnitWaypointBehavior::setCurrentTargetLayerInterface(self.as_UnitWaypointBehavior_mut(), currentTargetLayerInterface) }
    fn setCurrentTargetDistance(&mut self, mut currentTargetDistance: i32) { UnitWaypointBehavior::setCurrentTargetDistance(self.as_UnitWaypointBehavior_mut(), currentTargetDistance) }
    fn getCurrentTargetDistance(&mut self) -> i32 { UnitWaypointBehavior::getCurrentTargetDistance(self.as_UnitWaypointBehavior_mut()) }
    fn getCompleteTimeDelayHelper(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>> { UnitWaypointBehavior::getCompleteTimeDelayHelper(self.as_UnitWaypointBehavior_mut()) }
    fn setTrackingWaypoint(&mut self, mut trackingWaypoint: bool) { UnitWaypointBehavior::setTrackingWaypoint(self.as_UnitWaypointBehavior_mut(), trackingWaypoint) }
}
impl UnitWaypointBehaviorDyn for UnitWaypointBehavior {
    fn as_UnitWaypointBehavior(&self) -> &UnitWaypointBehavior { self }
    fn as_UnitWaypointBehavior_mut(&mut self) -> &mut UnitWaypointBehavior { self }
}

impl crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn for UnitWaypointBehavior {
    fn as_WaypointBehaviorBase(&self) -> &crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase { &self.base }
    fn as_WaypointBehaviorBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBase { &mut self.base }
    fn getMovementLogicAsString(&mut self) -> String { Self::getMovementLogicAsString(self) }
    fn isWaypointListEmptyOrOnlyTargets(&mut self) -> bool { Self::isWaypointListEmptyOrOnlyTargets(self) }
    fn getCurrentTargetLayerInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> { Self::getCurrentTargetLayerInterface(self) }
    fn isTrackingWaypoint(&mut self) -> bool { Self::isTrackingWaypoint(self) }
    fn getCurrentPathGeographicMapCellPosition(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> { Self::getCurrentPathGeographicMapCellPosition(self) }
    fn getCurrentGeographicMapCellHistory(&mut self) -> Option<Rc<RefCell<GeographicMapCellHistory>>> { Self::getCurrentGeographicMapCellHistory(self) }
    fn needToMove(&mut self) -> bool { Self::needToMove(self) }
    fn setWaypointPathsList(&mut self, mut waypointPathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setWaypointPathsList(self, waypointPathsList) }
    fn getWaypointPathsList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getWaypointPathsList(self) }
}


