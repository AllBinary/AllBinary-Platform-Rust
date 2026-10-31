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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::RTSLayer2LogHelper::RTSLayer2LogHelper;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayer2SelectedLogHelper {
    pub base: RTSLayer2LogHelper,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub TRACK_TO_FIRE_OR_MOVE: String,
    pub BUILDING_CHASE: String,
}

thread_local! { pub static RTSLayer2SelectedLogHelper_instanceC: Option<Rc<RefCell<RTSLayer2SelectedLogHelper>>> = RTSLayer2SelectedLogHelper::new(); }

impl RTSLayer2SelectedLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RTSLayer2LogHelper::new_struct(),
            logUtil: None,
            TRACK_TO_FIRE_OR_MOVE: String::new(),
            BUILDING_CHASE: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.TRACK_TO_FIRE_OR_MOVE = String::from("trackTo:fireOrMove");
        this.BUILDING_CHASE = String::from("buildingChase");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<RTSLayer2SelectedLogHelper>>> {
        return RTSLayer2SelectedLogHelper_instanceC.with(Clone::clone).clone();
    }
    

    pub fn steeringUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Up")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACK_TO_FIRE_OR_MOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn steeringFireOrStop(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Fire/All Stop")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.TRACK_TO_FIRE_OR_MOVE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingAbove(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" Building Above")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingChaseLeft(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Going Left: ")).appendint(angle.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingChaseRight(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Going Right: ")).appendint(angle.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" Building Below")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingChaseDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Going Down: ")).appendint(angle.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingChaseUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" steering - Going Up: ")).appendint(angle.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingLeft(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" Building Left")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn buildingRight(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(pathFindingLayerInterface).getName()).append(String::from(" Building Right")).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.BUILDING_CHASE.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for RTSLayer2SelectedLogHelper {
    type Target = RTSLayer2LogHelper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayer2SelectedLogHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelperDyn for RTSLayer2SelectedLogHelper {
    fn as_RTSLayer2LogHelper(&self) -> &crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelper { &self.base }
    fn as_RTSLayer2LogHelper_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayer2LogHelper::RTSLayer2LogHelper { &mut self.base }
    fn steeringUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::steeringUp(self, pathFindingLayerInterface) }
    fn steeringFireOrStop(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::steeringFireOrStop(self, pathFindingLayerInterface) }
    fn buildingAbove(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::buildingAbove(self, pathFindingLayerInterface) }
    fn buildingChaseLeft(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { Self::buildingChaseLeft(self, pathFindingLayerInterface, angle) }
    fn buildingChaseRight(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { Self::buildingChaseRight(self, pathFindingLayerInterface, angle) }
    fn buildingDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { Self::buildingDown(self, pathFindingLayerInterface) }
    fn buildingChaseDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { Self::buildingChaseDown(self, pathFindingLayerInterface, angle) }
    fn buildingChaseUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { Self::buildingChaseUp(self, pathFindingLayerInterface, angle) }
}


