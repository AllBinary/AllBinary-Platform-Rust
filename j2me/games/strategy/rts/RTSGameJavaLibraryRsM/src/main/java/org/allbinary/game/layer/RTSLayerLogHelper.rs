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
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::math::NamedAngle::NamedAngle;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerLogHelper {
}

thread_local! { pub static RTSLayerLogHelper_instance: Option<Rc<RefCell<dyn RTSLayerLogHelperDyn>>> = RTSLayerLogHelper::new().map(|reference| reference as Rc<RefCell<dyn RTSLayerLogHelperDyn>>); }

impl RTSLayerLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn RTSLayerLogHelperDyn>>> {
        return RTSLayerLogHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn setClosestGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
    

    pub fn trackTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut dx: i32, mut dy: i32, mut reason: String) {
    }
    

    pub fn turnTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut angle: i32, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>, mut evading: bool, mut targetAngle: i32) {
    }
    

    pub fn doneMoving(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn closeEnough(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn movingLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn movingRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn movingUp(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn movingDown(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn currentMoveEnded(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn evade(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn rotateLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn rotateRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn handle(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>) {
    }
    

    pub fn noRotation(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn notOnPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
    }
}
pub trait RTSLayerLogHelperDyn {
    fn as_RTSLayerLogHelper(&self) -> &RTSLayerLogHelper;
    fn as_RTSLayerLogHelper_mut(&mut self) -> &mut RTSLayerLogHelper;
    fn setClosestGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RTSLayerLogHelper::setClosestGeographicMapCellHistory(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer, pathsList) }
    fn trackTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut dx: i32, mut dy: i32, mut reason: String) { RTSLayerLogHelper::trackTo(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer, nextUnvisitedPathGeographicMapCellPosition, dx, dy, reason) }
    fn turnTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut angle: i32, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>, mut evading: bool, mut targetAngle: i32) { RTSLayerLogHelper::turnTo(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer, dx, dy, angleInfo, angle, movementAngle, evading, targetAngle) }
    fn doneMoving(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::doneMoving(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn closeEnough(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::closeEnough(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn movingLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::movingLeft(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn movingRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::movingRight(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn movingUp(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::movingUp(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn movingDown(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::movingDown(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn currentMoveEnded(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::currentMoveEnded(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn evade(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::evade(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn rotateLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::rotateLeft(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn rotateRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::rotateRight(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn handle(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>) { RTSLayerLogHelper::handle(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer, movementAngle) }
    fn noRotation(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayerLogHelper::noRotation(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer) }
    fn notOnPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { RTSLayerLogHelper::notOnPath(self.as_RTSLayerLogHelper_mut(), associatedAdvancedRTSGameLayer, geographicMapCellHistory, currentGeographicMapCellPosition, pathList) }
}
impl RTSLayerLogHelperDyn for RTSLayerLogHelper {
    fn as_RTSLayerLogHelper(&self) -> &RTSLayerLogHelper { self }
    fn as_RTSLayerLogHelper_mut(&mut self) -> &mut RTSLayerLogHelper { self }
}


