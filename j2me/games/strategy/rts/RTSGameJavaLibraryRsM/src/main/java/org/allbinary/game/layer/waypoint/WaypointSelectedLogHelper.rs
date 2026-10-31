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
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::WaypointLogHelper::WaypointLogHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointSelectedLogHelper {
    pub base: WaypointLogHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub stringUtil: Option<Rc<RefCell<StringUtil>>>,
}

thread_local! { pub static WaypointSelectedLogHelper_instanceC: Option<Rc<RefCell<WaypointSelectedLogHelper>>> = WaypointSelectedLogHelper::new(); }

impl WaypointSelectedLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: WaypointLogHelper::new_struct(),
            logUtil: None,
            commonSeps: None,
            stringUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.stringUtil = StringUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<WaypointSelectedLogHelper>>> {
        return WaypointSelectedLogHelper_instanceC.with(Clone::clone).clone();
    }
    

    pub fn onWaypointEvent(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" onWaypointEvent: "));
        jcall!(stringBuffer).append(jcall!(advancedRTSGameLayer).getName());
        if jcall!(advancedRTSGameLayer).getParentLayer().is_some() {
            jcall!(stringBuffer).append(String::from(" With Parent: "));
            jcall!(stringBuffer).append(jcall!(jcall!(advancedRTSGameLayer).getParentLayer()).getName());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("onWaypointEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn addWaypointFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Adding Waypoint: "));
        jcall!(stringBuffer).append(jcall!(advancedRTSGameLayer).getName());
        if jcall!(advancedRTSGameLayer).getParentLayer().is_some() {
            jcall!(stringBuffer).append(String::from(" With Parent: "));
            jcall!(stringBuffer).append(jcall!(jcall!(advancedRTSGameLayer).getParentLayer()).getName());
        }
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: onWaypointEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn addWaypointFromBuildingList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" Waypoints: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(list.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: onWaypointEvent"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn insertWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(waypointName.clone());
        jcall!(stringBuffer).append(String::from(" Insert: "));
        jcall!(stringBuffer).append(jcall!(rtsLayer).getName());
        jcall!(stringBuffer).append(String::from(" for Waypoint: "));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: insertWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn insertWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(waypointName.clone());
        jcall!(stringBuffer).append(String::from(" Insert: "));
        jcall!(stringBuffer).append(jcall!(rtsLayer).getName());
        jcall!(stringBuffer).append(String::from(" Waypoints: "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(list.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: insertWaypoint"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setRandomGeographicMapCellHistoryList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut size: i32 = jcall!(pathsList).size();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).START.clone());
        jcall!(stringBuffer).appendint(size.clone());
        jcall!(stringBuffer).append(String::from(" -> "));
        jcall!(stringBuffer).append(jcall!(self.stringUtil).toString(crate::java_into_object(pathsList.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: setRandomGeographicMapCellHistory"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn moveAwayFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" Trying to move away from building")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: moveAwayFromBuilding"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn needToMove(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(jcall!(unitWaypointBehavior).getMovementLogicAsString()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("needToMove"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setRandomGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" pathsList is null")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setRandomGeographicMapCellHistory"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn setCurrentPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingCurrentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if existingCurrentPathGeographicMapCellPosition.is_none() {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" currentPathGeographicMapCellPosition: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(currentPathGeographicMapCellPosition.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setCurrentPathGeographicMapCellPosition"),); let java_result = jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); java_result };
        }
    }
    

    pub fn setNextUnvisitedPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingNextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) {
        if !match (&existingNextUnvisitedPathGeographicMapCellPosition, &nextUnvisitedPathGeographicMapCellPosition) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" nextUnvisitedPathGeographicMapCellPosition: ")).append(jcall!(self.stringUtil).toString(crate::java_into_object(nextUnvisitedPathGeographicMapCellPosition.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setNextUnvisitedPathGeographicMapCellPosition"),); let java_result = jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); java_result };
        }
    }
}
impl std::ops::Deref for WaypointSelectedLogHelper {
    type Target = WaypointLogHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointSelectedLogHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelperDyn for WaypointSelectedLogHelper {
    fn as_WaypointLogHelper(&self) -> &crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelper { &self.base }
    fn as_WaypointLogHelper_mut(&mut self) -> &mut crate::org::allbinary::game::layer::waypoint::WaypointLogHelper::WaypointLogHelper { &mut self.base }
    fn onWaypointEvent(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::onWaypointEvent(self, associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn addWaypointFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::addWaypointFromBuilding(self, associatedAdvancedRTSGameLayer, advancedRTSGameLayer) }
    fn addWaypointFromBuildingList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut advancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::addWaypointFromBuildingList(self, associatedAdvancedRTSGameLayer, advancedRTSGameLayer, list) }
    fn insertWaypoint(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String) { Self::insertWaypoint(self, associatedAdvancedRTSGameLayer, index, rtsLayer, waypointName) }
    fn insertWaypointList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut index: i32, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>, mut waypointName: String, mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insertWaypointList(self, associatedAdvancedRTSGameLayer, index, rtsLayer, waypointName, list) }
    fn setRandomGeographicMapCellHistoryList(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setRandomGeographicMapCellHistoryList(self, associatedAdvancedRTSGameLayer, pathsList) }
    fn moveAwayFromBuilding(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::moveAwayFromBuilding(self, associatedAdvancedRTSGameLayer) }
    fn needToMove(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>>) { Self::needToMove(self, associatedAdvancedRTSGameLayer, unitWaypointBehavior) }
    fn setRandomGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::setRandomGeographicMapCellHistory(self, associatedAdvancedRTSGameLayer) }
    fn setCurrentPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingCurrentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut currentPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::setCurrentPathGeographicMapCellPosition(self, associatedAdvancedRTSGameLayer, existingCurrentPathGeographicMapCellPosition, currentPathGeographicMapCellPosition) }
    fn setNextUnvisitedPathGeographicMapCellPosition(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut existingNextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>) { Self::setNextUnvisitedPathGeographicMapCellPosition(self, associatedAdvancedRTSGameLayer, existingNextUnvisitedPathGeographicMapCellPosition, nextUnvisitedPathGeographicMapCellPosition) }
}


