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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::math::NamedAngle::NamedAngle;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellHistory::GeographicMapCellHistory;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use super::RTSLayerLogHelper::RTSLayerLogHelper;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerSelectedLogHelper {
    pub base: RTSLayerLogHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
    pub positionStrings: Option<Rc<RefCell<PositionStrings>>>,
    pub TRACKTO_TURNTO: String,
}

thread_local! { pub static RTSLayerSelectedLogHelper_instanceC: Option<Rc<RefCell<RTSLayerSelectedLogHelper>>> = RTSLayerSelectedLogHelper::new(); }

impl RTSLayerSelectedLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RTSLayerLogHelper::new_struct(),
            logUtil: None,
            commonSeps: None,
            positionStrings: None,
            TRACKTO_TURNTO: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonSeps = CommonSeps::getInstance();
        this.positionStrings = PositionStrings::getInstance();
        this.TRACKTO_TURNTO = String::from("trackTo:turnTo");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RTSLayerSelectedLogHelper>>> {
        return RTSLayerSelectedLogHelper_instanceC.with(Clone::clone).clone();
    }
    

    pub fn setClosestGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).START.clone());
        jcall!(stringBuffer).appendint(jcall!(pathsList).size());
        jcall!(stringBuffer).append(String::from(" -> "));
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(pathsList.clone())));
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: setClosestGeographicMapCellHistory"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn trackTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut dx: i32, mut dy: i32, mut reason: String) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(nextUnvisitedPathGeographicMapCellPosition.clone())));
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DX_LABEL.clone());
        jcall!(stringBuffer).appendint(dx.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DY_LABEL.clone());
        jcall!(stringBuffer).appendint(dy.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(reason.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selected: trackTo"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn turnTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut angle: i32, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>, mut evading: bool, mut targetAngle: i32) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(associatedAdvancedRTSGameLayer).getName());
        jcall!(stringBuffer).append(String::from(" steering - "));
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DX_LABEL.clone());
        jcall!(stringBuffer).appendint(dx.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(self.positionStrings).DY_LABEL.clone());
        jcall!(stringBuffer).appendint(dy.clone());
        jcall!(stringBuffer).append(String::from(" angle: "));
        jcall!(stringBuffer).appendint(angle.clone());
        jcall!(stringBuffer).append(String::from(" movementAngle: "));
        jcall!(stringBuffer).append(jcall!(movementAngle).name.clone());
        jcall!(stringBuffer).append(jcall!(self.commonSeps).FORWARD_SLASH.clone());
        jcall!(stringBuffer).appendshort(jcall!(movementAngle).getValue());
        if angleInfo.is_some() {
            let mut angleIncrement: i16 = jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement();
            jcall!(stringBuffer).append(String::from(" angleIncrement: "));
            jcall!(stringBuffer).appendshort(angleIncrement.clone());
        }
        jcall!(stringBuffer).append(String::from(" Evading: "));
        jcall!(stringBuffer).appendboolean(evading.clone());
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn doneMoving(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - done moving 0,0")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn closeEnough(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - close enough")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn movingLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - allow movingLeft")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn movingRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - allow movingRight")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn movingUp(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - allow movingUp")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn movingDown(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - allow movingDown")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn currentMoveEnded(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - correct angle, but movement not needed for current deltas so reseting movement angle")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn evade(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - evade")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn rotateLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - rotating left")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn rotateRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - rotating right")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn handle(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - movementAngle: ")).append(jcall!(movementAngle).name.clone()).append(jcall!(self.commonSeps).FORWARD_SLASH.clone()).appendshort(jcall!(movementAngle).getValue()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn noRotation(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(String::from(" steering - Do not rotate")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACKTO_TURNTO.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn notOnPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(associatedAdvancedRTSGameLayer).getName()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).appendint(jcall!(geographicMapCellHistory).getTotalVisited()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(jcall!(stringUtil).toString(crate::java_into_object(currentGeographicMapCellPosition.clone()))).append(String::from(" - trying to move but not on path: ")).append(jcall!(stringUtil).toString(crate::java_into_object(pathList.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("turnTo"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for RTSLayerSelectedLogHelper {
    type Target = RTSLayerLogHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerSelectedLogHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelperDyn for RTSLayerSelectedLogHelper {
    fn as_RTSLayerLogHelper(&self) -> &crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelper { &self.base }
    fn as_RTSLayerLogHelper_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayerLogHelper::RTSLayerLogHelper { &mut self.base }
    fn setClosestGeographicMapCellHistory(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut pathsList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::setClosestGeographicMapCellHistory(self, associatedAdvancedRTSGameLayer, pathsList) }
    fn trackTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut nextUnvisitedPathGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut dx: i32, mut dy: i32, mut reason: String) { Self::trackTo(self, associatedAdvancedRTSGameLayer, nextUnvisitedPathGeographicMapCellPosition, dx, dy, reason) }
    fn turnTo(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut dx: i32, mut dy: i32, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut angle: i32, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>, mut evading: bool, mut targetAngle: i32) { Self::turnTo(self, associatedAdvancedRTSGameLayer, dx, dy, angleInfo, angle, movementAngle, evading, targetAngle) }
    fn doneMoving(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::doneMoving(self, associatedAdvancedRTSGameLayer) }
    fn closeEnough(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::closeEnough(self, associatedAdvancedRTSGameLayer) }
    fn movingLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::movingLeft(self, associatedAdvancedRTSGameLayer) }
    fn movingRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::movingRight(self, associatedAdvancedRTSGameLayer) }
    fn movingUp(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::movingUp(self, associatedAdvancedRTSGameLayer) }
    fn movingDown(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::movingDown(self, associatedAdvancedRTSGameLayer) }
    fn currentMoveEnded(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::currentMoveEnded(self, associatedAdvancedRTSGameLayer) }
    fn evade(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::evade(self, associatedAdvancedRTSGameLayer) }
    fn rotateLeft(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::rotateLeft(self, associatedAdvancedRTSGameLayer) }
    fn rotateRight(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::rotateRight(self, associatedAdvancedRTSGameLayer) }
    fn handle(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut movementAngle: Option<Rc<RefCell<NamedAngle>>>) { Self::handle(self, associatedAdvancedRTSGameLayer, movementAngle) }
    fn noRotation(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::noRotation(self, associatedAdvancedRTSGameLayer) }
    fn notOnPath(&mut self, mut associatedAdvancedRTSGameLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut geographicMapCellHistory: Option<Rc<RefCell<GeographicMapCellHistory>>>, mut currentGeographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>>, mut pathList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::notOnPath(self, associatedAdvancedRTSGameLayer, geographicMapCellHistory, currentGeographicMapCellPosition, pathList) }
}


