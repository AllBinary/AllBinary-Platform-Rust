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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::Waypoint2LogHelper::Waypoint2LogHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Waypoint2SelectedLogHelper {
    pub base: Waypoint2LogHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub positionStrings: Option<Rc<RefCell<PositionStrings>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
    pub ENEMY_POSSIBLE_TARGET: String,
    pub CLOSE_RANGE: String,
    pub SENSOR_RANGE: String,
    pub ONLY_TARGETS: String,
    pub SHORTER_THAN_CURRENT_TARGET_DISTANCE: String,
    pub CURRENT_TARGET_DESTROYED: String,
    pub CURRENT_TARGET_NOT_AVAILABLE: String,
    pub PROCESS_POSSIBLE_TARGET: String,
}

thread_local! { pub static Waypoint2SelectedLogHelper_instanceC: Option<Rc<RefCell<Waypoint2SelectedLogHelper>>> = Waypoint2SelectedLogHelper::new(); }

impl Waypoint2SelectedLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Waypoint2LogHelper::new_struct(),
            logUtil: None,
            commonSeps: None,
            positionStrings: None,
            stringUtil: None,
            ENEMY_POSSIBLE_TARGET: String::new(),
            CLOSE_RANGE: String::new(),
            SENSOR_RANGE: String::new(),
            ONLY_TARGETS: String::new(),
            SHORTER_THAN_CURRENT_TARGET_DISTANCE: String::new(),
            CURRENT_TARGET_DESTROYED: String::new(),
            CURRENT_TARGET_NOT_AVAILABLE: String::new(),
            PROCESS_POSSIBLE_TARGET: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.positionStrings = PositionStrings::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this.ENEMY_POSSIBLE_TARGET = String::from(" Enemy Possible Target: ");
        this.CLOSE_RANGE = String::from("closeRange: ");
        this.SENSOR_RANGE = String::from(" sensorRange: ");
        this.ONLY_TARGETS = String::from(" Only Targets: ");
        this.SHORTER_THAN_CURRENT_TARGET_DISTANCE = String::from(" isShorterThanCurrentTargetDistance: ");
        this.CURRENT_TARGET_DESTROYED = String::from(" isCurrentTargetDestroyed: ");
        this.CURRENT_TARGET_NOT_AVAILABLE = String::from(" isCurrentTargetNotAvailable: ");
        this.PROCESS_POSSIBLE_TARGET = String::from("selected: processPossibleTarget1");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn>>> {
        return Waypoint2SelectedLogHelper_instanceC.with(Clone::clone).clone();
    }
    

    pub fn initRange(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut closeRange: i32, mut sensorRange: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(self.CLOSE_RANGE.clone());
        jcall!(stringBuffer).appendint(closeRange.clone());
        jcall!(stringBuffer).append(self.SENSOR_RANGE.clone());
        jcall!(stringBuffer).appendint(sensorRange.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: initRange"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processPossibleTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32, mut isShorterThanCurrentTargetDistance: bool, mut isCurrentTargetDestroyed: bool) {
        if jcall!(waypointBehaviorBase).isWaypointListEmptyOrOnlyTargets() && jcall!(waypointBehaviorBase).isInSensorRange((layerInterface).clone() /* as CollidableDestroyableDamageableLayer */, anotherTargetDistance.clone()) {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
            jcall!(stringBuffer).append(self.ONLY_TARGETS.clone());
            jcall!(stringBuffer).appendboolean(jcall!(waypointBehaviorBase).isWaypointListEmptyOrOnlyTargets());
            jcall!(stringBuffer).append(self.SHORTER_THAN_CURRENT_TARGET_DISTANCE.clone());
            jcall!(stringBuffer).appendboolean(isShorterThanCurrentTargetDistance.clone());
            jcall!(stringBuffer).append(self.CURRENT_TARGET_DESTROYED.clone());
            jcall!(stringBuffer).appendboolean(isCurrentTargetDestroyed.clone());
            jcall!(stringBuffer).append(self.CURRENT_TARGET_NOT_AVAILABLE.clone());
            jcall!(stringBuffer).appendboolean((jcall!(waypointBehaviorBase).getCurrentTargetLayerInterface().is_none()).clone());
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.PROCESS_POSSIBLE_TARGET.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        }
    }
    

    pub fn processSetTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(waypointBehaviorBase).getCurrentTargetingStateString());
        jcall!(stringBuffer).append(self.ENEMY_POSSIBLE_TARGET.clone());
        jcall!(stringBuffer).append(jcall!(layerInterface).getName());
        jcall!(stringBuffer).append(String::from(" anotherTargetDistance: "));
        jcall!(stringBuffer).appendint(anotherTargetDistance.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processPossibleTarget2"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processPossibleTargetCloser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(waypointBehaviorBase).getCurrentTargetingStateString());
        jcall!(stringBuffer).append(self.ENEMY_POSSIBLE_TARGET.clone());
        jcall!(stringBuffer).append(jcall!(layerInterface).getName());
        jcall!(stringBuffer).append(String::from(" anotherTargetDistance: "));
        jcall!(stringBuffer).appendint(anotherTargetDistance.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processPossibleTarget3"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(waypointBehaviorBase).getName());
        jcall!(stringBuffer).append(String::from(" targeting: "));
        jcall!(stringBuffer).append(jcall!(layerInterface).getName());
        jcall!(stringBuffer).append(String::from(" at: "));
        jcall!(stringBuffer).appendint(anotherTargetDistance.clone());
        jcall!(stringBuffer).append(String::from(" instead of: "));
        jcall!(stringBuffer).append(jcall!(waypointBehaviorBase).getCurrentTargetingStateString());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: setTarget"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setTargetPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        jcall!(self.logUtil).putF(String::from("Target Destroyed"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: setTargetPath"));
    }
    

    pub fn setTargetPathIgnoreNewPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Set Path To Target: "));
        jcall!(stringBuffer).append(jcall!(jcall!(waypointBehaviorBase).getCurrentTargetLayerInterface()).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(jcall!(waypointBehaviorBase).getCurrentTargetLayerInterface()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result });
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: setTargetPath"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut targetLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut size: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Size: "));
        jcall!(stringBuffer).appendint(size.clone());
        jcall!(stringBuffer).append(String::from(" Waypoint: "));
        jcall!(stringBuffer).append(jcall!(targetLayer).getName());
        jcall!(stringBuffer).append(String::from(" Target: "));
        jcall!(stringBuffer).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(jcall!(waypointBehaviorBase).getCurrentTargetLayerInterface()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result });
        jcall!(stringBuffer).append(String::from(" isTrackingWaypoint: "));
        jcall!(stringBuffer).appendboolean(jcall!(waypointBehaviorBase).isTrackingWaypoint());
        jcall!(stringBuffer).append(String::from(" WaypointOverridesAttacking: "));
        jcall!(stringBuffer).appendboolean(jcall!(waypointBehaviorBase).isWaypointOverridesAttacking());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processWaypointTracked(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" currentPathGeographicMapCellPosition: "));
        jcall!(stringBuffer).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(jcall!(waypointBehaviorBase).getCurrentPathGeographicMapCellPosition()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result });
        jcall!(stringBuffer).append(String::from(" nextUnvisitedPathGeographicMapCellPosition: "));
        jcall!(stringBuffer).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(jcall!(waypointBehaviorBase).getNextUnvisitedPathGeographicMapCellPosition()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result });
        jcall!(stringBuffer).append(String::from(" Tracked: "));
        jcall!(stringBuffer).append({ let java_receiver = self.stringUtil.clone(); let java_arguments = (crate::java_into_object(jcall!(jcall!(waypointBehaviorBase).getCurrentGeographicMapCellHistory()).getTracked()),); let java_result = jcall!(java_receiver).toString(java_arguments.0); java_result });
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processWaypointTrackedVisit(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Visited: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(geographicMapCellPosition.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processWaypointTrackedWithoutProgress(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(reason.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("turnTo"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn wander(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(associatedAdvancedRTSGameLayer).getName(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("wander"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn targetDestroyed(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" Target Destroyed")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processTargeting"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processTargeting(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DX_LABEL.clone());
        jcall!(stringBuffer).appendint(dx.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DY_LABEL.clone());
        jcall!(stringBuffer).appendint(dy.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processTargeting"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn processTargetingNonWayPoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Targeting Non Waypoint "));
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DX_LABEL.clone());
        jcall!(stringBuffer).appendint(dx.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DY_LABEL.clone());
        jcall!(stringBuffer).appendint(dy.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: processTargeting"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn removeWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(unitWaypointBehavior2).getName());
        jcall!(stringBuffer).append(String::from(" remove: "));
        jcall!(stringBuffer).append(jcall!(waypointLayer).getName());
        jcall!(stringBuffer).append(String::from(" because: "));
        jcall!(stringBuffer).append(reason.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: removeWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn removeWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(unitWaypointBehavior2).getName());
        jcall!(stringBuffer).append(String::from(" Waypoints: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(list.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: removeWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn removeWaypointClear(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" Clearing removed waypoint")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: removeWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn clearTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" Cleared Target")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: clearTarget"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn addWaypointFromUser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Adding Waypoint to Selected: "));
        jcall!(stringBuffer).append(jcall!(advancedRTSGameLayer).getName());
        if jcall!(advancedRTSGameLayer).getParentLayer().is_some() {
            jcall!(stringBuffer).append(String::from(" With Parent: "));
            jcall!(stringBuffer).append(jcall!(jcall!(advancedRTSGameLayer).getParentLayer()).getName());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: onWaypointEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn targetMovedSoRetarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" - target moved so retargeting")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("turnTo"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for Waypoint2SelectedLogHelper {
    type Target = Waypoint2LogHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for Waypoint2SelectedLogHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelperDyn for Waypoint2SelectedLogHelper {
    fn as_Waypoint2LogHelper(&self) -> &crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelper { &self.base }
    fn as_Waypoint2LogHelper_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::Waypoint2LogHelper::Waypoint2LogHelper { &mut self.base }
    fn initRange(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut closeRange: i32, mut sensorRange: i32) { Self::initRange(self, associatedAdvancedRTSGameLayer, closeRange, sensorRange) }
    fn processPossibleTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32, mut isShorterThanCurrentTargetDistance: bool, mut isCurrentTargetDestroyed: bool) { Self::processPossibleTarget(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase, layerInterface, anotherTargetDistance, isShorterThanCurrentTargetDistance, isCurrentTargetDestroyed) }
    fn processSetTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Self::processSetTarget(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase, layerInterface, anotherTargetDistance) }
    fn processPossibleTargetCloser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Self::processPossibleTargetCloser(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase, layerInterface, anotherTargetDistance) }
    fn setTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut layerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut anotherTargetDistance: i32) { Self::setTarget(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase, layerInterface, anotherTargetDistance) }
    fn setTargetPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::setTargetPath(self, associatedAdvancedRTSGameLayer) }
    fn setTargetPathIgnoreNewPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { Self::setTargetPathIgnoreNewPath(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase) }
    fn processWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut targetLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut size: i32) { Self::processWaypoint(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase, targetLayer, size) }
    fn processWaypointTracked(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut waypointBehaviorBase: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { Self::processWaypointTracked(self, associatedAdvancedRTSGameLayer, waypointBehaviorBase) }
    fn processWaypointTrackedVisit(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::processWaypointTrackedVisit(self, associatedAdvancedRTSGameLayer, geographicMapCellPosition) }
    fn processWaypointTrackedWithoutProgress(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) { Self::processWaypointTrackedWithoutProgress(self, associatedAdvancedRTSGameLayer, reason) }
    fn wander(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::wander(self, associatedAdvancedRTSGameLayer) }
    fn targetDestroyed(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::targetDestroyed(self, associatedAdvancedRTSGameLayer) }
    fn processTargeting(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) { Self::processTargeting(self, associatedAdvancedRTSGameLayer, dx, dy) }
    fn processTargetingNonWayPoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32) { Self::processTargetingNonWayPoint(self, associatedAdvancedRTSGameLayer, dx, dy) }
    fn removeWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut reason: String) { Self::removeWaypoint(self, associatedAdvancedRTSGameLayer, unitWaypointBehavior2, waypointLayer, reason) }
    fn removeWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior2: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::removeWaypointList(self, associatedAdvancedRTSGameLayer, unitWaypointBehavior2, list) }
    fn removeWaypointClear(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::removeWaypointClear(self, associatedAdvancedRTSGameLayer) }
    fn clearTarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::clearTarget(self, associatedAdvancedRTSGameLayer) }
    fn addWaypointFromUser(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::addWaypointFromUser(self, associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn targetMovedSoRetarget(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::targetMovedSoRetarget(self, associatedAdvancedRTSGameLayer) }
}


