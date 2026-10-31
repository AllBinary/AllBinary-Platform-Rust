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
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayer2LogHelper {
}

thread_local! { pub static RTSLayer2LogHelper_instance: Option<Rc<RefCell<dyn RTSLayer2LogHelperDyn>>> = RTSLayer2LogHelper::new().map(|reference| reference as Rc<RefCell<dyn RTSLayer2LogHelperDyn>>); }

impl RTSLayer2LogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn RTSLayer2LogHelperDyn>>> {
        return RTSLayer2LogHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn steeringUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn steeringFireOrStop(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn buildingAbove(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn buildingChaseLeft(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
    }
    

    pub fn buildingChaseRight(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
    }
    

    pub fn buildingDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn buildingChaseDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
    }
    

    pub fn buildingChaseUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) {
    }
    

    pub fn buildingLeft(&mut self) {
    }
    

    pub fn buildingRight(&mut self) {
    }
}
pub trait RTSLayer2LogHelperDyn {
    fn as_RTSLayer2LogHelper(&self) -> &RTSLayer2LogHelper;
    fn as_RTSLayer2LogHelper_mut(&mut self) -> &mut RTSLayer2LogHelper;
    fn steeringUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer2LogHelper::steeringUp(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface) }
    fn steeringFireOrStop(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer2LogHelper::steeringFireOrStop(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface) }
    fn buildingAbove(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer2LogHelper::buildingAbove(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface) }
    fn buildingChaseLeft(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { RTSLayer2LogHelper::buildingChaseLeft(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface, angle) }
    fn buildingChaseRight(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { RTSLayer2LogHelper::buildingChaseRight(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface, angle) }
    fn buildingDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { RTSLayer2LogHelper::buildingDown(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface) }
    fn buildingChaseDown(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { RTSLayer2LogHelper::buildingChaseDown(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface, angle) }
    fn buildingChaseUp(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>, mut angle: i32) { RTSLayer2LogHelper::buildingChaseUp(self.as_RTSLayer2LogHelper_mut(), pathFindingLayerInterface, angle) }
    fn buildingLeft(&mut self) { RTSLayer2LogHelper::buildingLeft(self.as_RTSLayer2LogHelper_mut()) }
    fn buildingRight(&mut self) { RTSLayer2LogHelper::buildingRight(self.as_RTSLayer2LogHelper_mut()) }
}
impl RTSLayer2LogHelperDyn for RTSLayer2LogHelper {
    fn as_RTSLayer2LogHelper(&self) -> &RTSLayer2LogHelper { self }
    fn as_RTSLayer2LogHelper_mut(&mut self) -> &mut RTSLayer2LogHelper { self }
}


