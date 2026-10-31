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
use crate::Runnable;
use super::*;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::math::LayerDistanceUtil::LayerDistanceUtil;
use crate::org::allbinary::thread::PriorityRunnable::PriorityRunnable;
use crate::org::allbinary::thread::RunnableInterface::RunnableInterface;
use crate::java::lang::Thread::*;
use super::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointPathRunnableBase {
    pub layerDistanceUtil: Option<Rc<RefCell<LayerDistanceUtil>>>,
    pub pathFindingLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub targetPathFindingLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
    pub priorityP: i32,
    pub runningP: bool,
}

impl RunnableInterface for WaypointPathRunnableBase {
    fn isRunning(&mut self) -> bool { Self::isRunning(self) }
    fn setRunning(&mut self, mut isRunning: bool) { Self::setRunning(self, isRunning) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { Self::setThread(self, thread) }
}
impl crate::Runnable for WaypointPathRunnableBase {
    fn run(&mut self) { Self::run(self) }
}
impl PriorityRunnable for WaypointPathRunnableBase {
    fn getPriority(&mut self) -> i32 { Self::getPriority(self) }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn reset(&mut self) { Self::reset(self) }
}
impl crate::Runnable for WaypointPathRunnableBase {
    fn run(&mut self) { Self::run(self) }
}

impl WaypointPathRunnableBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            layerDistanceUtil: None,
            pathFindingLayer: None,
            targetPathFindingLayer: None,
            priorityP: 0,
            runningP: false,
        };
        this.layerDistanceUtil = LayerDistanceUtil::getInstance();
        this.pathFindingLayer = NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this.targetPathFindingLayer = NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this.priorityP = i32::MAX.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isRunning(&mut self) -> bool {
        return self.runningP.clone();
    }
    

    pub fn setRunning(&mut self, mut isRunning: bool) {
        self.runningP = isRunning.clone();
    }
    

    // throws Exception
    pub fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
    }
    

    pub fn setTargetLayer(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        self.targetPathFindingLayer = waypointLayer.clone();
        self.setPriority();
    }
    

    pub fn setPriority(&mut self) {
        let mut pathFindingLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = (self.pathFindingLayer).clone() /* as AllBinaryLayer */;
        let mut targetLayer: Option<Rc<RefCell<dyn crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayerDyn>>> = (self.targetPathFindingLayer).clone() /* as AllBinaryLayer */;
        if targetLayer.is_some() {
            let mut distance: i32 = jcall!(self.layerDistanceUtil).getDistance(targetLayer.clone(), pathFindingLayer.clone());
            let mut distanceCategory: i32 = distance / 70;
            self.priorityP = distanceCategory.clone();
        }
    }
    

    pub fn getTargetLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        return self.targetPathFindingLayer.clone();
    }
    

    pub fn setUnitLayer(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
        self.pathFindingLayer = unitLayer.clone();
    }
    

    pub fn getPriority(&mut self) -> i32 {
        return self.priorityP.clone();
    }
    

    pub fn isDone(&mut self) -> bool {
        return true;
    }
    

    pub fn run(&mut self) {
    }
    

    pub fn reset(&mut self) {
    }
}
pub trait WaypointPathRunnableBaseDyn {
    fn as_WaypointPathRunnableBase(&self) -> &WaypointPathRunnableBase;
    fn as_WaypointPathRunnableBase_mut(&mut self) -> &mut WaypointPathRunnableBase;
    fn isRunning(&mut self) -> bool { WaypointPathRunnableBase::isRunning(self.as_WaypointPathRunnableBase_mut()) }
    fn setRunning(&mut self, mut isRunning: bool) { WaypointPathRunnableBase::setRunning(self.as_WaypointPathRunnableBase_mut(), isRunning) }
    fn setThread(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) { WaypointPathRunnableBase::setThread(self.as_WaypointPathRunnableBase_mut(), thread) }
    fn setTargetLayer(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointPathRunnableBase::setTargetLayer(self.as_WaypointPathRunnableBase_mut(), waypointLayer) }
    fn setPriority(&mut self) { WaypointPathRunnableBase::setPriority(self.as_WaypointPathRunnableBase_mut()) }
    fn getTargetLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> { WaypointPathRunnableBase::getTargetLayer(self.as_WaypointPathRunnableBase_mut()) }
    fn setUnitLayer(&mut self, mut unitLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointPathRunnableBase::setUnitLayer(self.as_WaypointPathRunnableBase_mut(), unitLayer) }
    fn getPriority(&mut self) -> i32 { WaypointPathRunnableBase::getPriority(self.as_WaypointPathRunnableBase_mut()) }
    fn isDone(&mut self) -> bool { WaypointPathRunnableBase::isDone(self.as_WaypointPathRunnableBase_mut()) }
    fn run(&mut self) { WaypointPathRunnableBase::run(self.as_WaypointPathRunnableBase_mut()) }
    fn reset(&mut self) { WaypointPathRunnableBase::reset(self.as_WaypointPathRunnableBase_mut()) }
}
impl WaypointPathRunnableBaseDyn for WaypointPathRunnableBase {
    fn as_WaypointPathRunnableBase(&self) -> &WaypointPathRunnableBase { self }
    fn as_WaypointPathRunnableBase_mut(&mut self) -> &mut WaypointPathRunnableBase { self }
}


