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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::java::lang::Thread::*;
use super::WaypointPathRunnableBase::WaypointPathRunnableBase;
use super::WaypointBehaviorBase::WaypointBehaviorBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointPathRunnable {
    pub base: WaypointPathRunnableBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl crate::org::allbinary::thread::PriorityRunnable::PriorityRunnable for WaypointPathRunnable {
    fn getPriority(&mut self) -> i32 { self.base.getPriority() }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn reset(&mut self) { self.base.reset() }
}
impl crate::Runnable for WaypointPathRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl WaypointPathRunnable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: WaypointPathRunnableBase::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn run(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(self.pathFindingLayer).getWaypointRunnableLogHelper()).start(self.pathFindingLayer.clone());
            let mut waypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = jcall!(self.pathFindingLayer).getWaypointBehavior();
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.pathFindingLayer).getCurrentGeographicMapCellPosition();
            if geographicMapCellPosition.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should never be running here"))));
            }
            jcall!(waypointBehavior).setWaypointPathsList(jcall!(jcall!(jcall!(self.targetPathFindingLayer).getWaypointBehavior()).getWaypoint()).getPathsListRunnable(geographicMapCellPosition.clone()));
            jcall!(jcall!(self.pathFindingLayer).getWaypointRunnableLogHelper()).end(self.pathFindingLayer.clone());
            std::thread::sleep(std::time::Duration::from_millis((15_i64 + (self.priorityP * 2)) as u64));
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
            self.setRunning(false);
            }
        }
    }
    

    pub fn isDone(&mut self) -> bool {
        return true;
    }
}
impl std::ops::Deref for WaypointPathRunnable {
    type Target = WaypointPathRunnableBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WaypointPathRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn for WaypointPathRunnable {
    fn as_WaypointPathRunnableBase(&self) -> &crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase { &self.base }
    fn as_WaypointPathRunnableBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase { &mut self.base }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn run(&mut self) { Self::run(self) }
}


