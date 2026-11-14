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
use crate::Runnable;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::thread::ARunnable::ARunnable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPosition;
use crate::org::allbinary::media::graphics::geography::pathfinding::MultipassState::MultipassState;
use crate::org::allbinary::media::graphics::geography::pathfinding::PathFindingInfo::PathFindingInfo;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListUtil::BasicArrayListUtil;
use crate::java::lang::Exception::*;
use crate::java::lang::RuntimeException::*;
use super::WaypointPathRunnableBase::WaypointPathRunnableBase;
use super::WaypointBehaviorBase::WaypointBehaviorBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MultipassWaypointPathRunnable {
    pub base: WaypointPathRunnableBase,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicArrayListUtil: Option<Rc<RefCell<BasicArrayListUtil>>>,
    pub multipassState: Option<Rc<RefCell<MultipassState>>>,
    pub done: bool,
    pub list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub pathFindingInfo: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub FIRST_RUNNABLE: Option<Rc<RefCell<dyn Runnable>>>,
    pub SECOND_RUNNABLE: Option<Rc<RefCell<dyn Runnable>>>,
    pub END_RUNNABLE: Option<Rc<RefCell<dyn Runnable>>>,
    pub ALREADY_ENDED_RUNNABLE: Option<Rc<RefCell<dyn Runnable>>>,
    pub currentPassRunnable: Option<Rc<RefCell<dyn Runnable>>>,
}

impl crate::org::allbinary::thread::PriorityRunnable::PriorityRunnable for MultipassWaypointPathRunnable {
    fn getPriority(&mut self) -> i32 { crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase::getPriority(crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn::as_WaypointPathRunnableBase_mut(self)) }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn reset(&mut self) { Self::reset(self) }
}
impl crate::org::allbinary::thread::RunnableInterface::RunnableInterface for MultipassWaypointPathRunnable {
    fn isRunning(&mut self) -> bool { crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase::isRunning(crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn::as_WaypointPathRunnableBase_mut(self)) }
    fn setRunning(&mut self, mut isRunning: bool) { Self::setRunning(self, isRunning) }
    fn setThread(&mut self, p1: Option<Rc<RefCell<crate::java::lang::Thread::Thread>>>) { crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase::setThread(crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn::as_WaypointPathRunnableBase_mut(self), p1) }
}
impl Runnable for MultipassWaypointPathRunnable {
    fn run(&mut self) { Self::run(self) }
}

impl MultipassWaypointPathRunnable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: WaypointPathRunnableBase::new_struct(),
            logUtil: None,
            basicArrayListUtil: None,
            multipassState: None,
            done: false,
            list: None,
            pathFindingInfo: None,
            FIRST_RUNNABLE: None,
            SECOND_RUNNABLE: None,
            END_RUNNABLE: None,
            ALREADY_ENDED_RUNNABLE: None,
            currentPassRunnable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicArrayListUtil = BasicArrayListUtil::getInstance();
        this.multipassState = MultipassState::new();
        this.done = false;
        this.list = jcall!(this.basicArrayListUtil).getImmutableInstance();
        this.pathFindingInfo = { let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }.clone();
        this.ALREADY_ENDED_RUNNABLE = ARunnable::new() /* Java anonymous class body: implement it as a named type
        

        pub fn run(&mut self) {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
        }
*/.map(|reference| reference as Rc<RefCell<dyn Runnable>>).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        this.currentPassRunnable = this.FIRST_RUNNABLE.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
        jcall!(this).FIRST_RUNNABLE = FirstRunnable::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        jcall!(this).SECOND_RUNNABLE = SecondRunnable::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        jcall!(this).END_RUNNABLE = EndRunnable::new(this.clone()).map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        this
    }
    

    pub fn setRunning(&mut self, mut isRunning: bool) {
        self.runningP = isRunning.clone();
        if self.runningP {
            self.reset();
            self.done = false;
        }
    }
    

    pub fn run(&mut self) {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.currentPassRunnable).run();
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            self.setRunning(false);
            }
        }
    }
    

    pub fn reset2(&mut self) {
        jcall!(self.multipassState).step = 0;
        jcall!(self.multipassState).iteration = 0;
        jcall!(self.multipassState).iteration2 = 0;
        self.pathFindingInfo = { let java_value = ({ let java_field = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone(); java_field }).clone(); java_value }.clone();
    }
    

    pub fn finish(&mut self) {
        self.reset2();
        self.currentPassRunnable = self.ALREADY_ENDED_RUNNABLE.clone();
        self.done = true;
    }
    

    pub fn isDone(&mut self) -> bool {
        return self.done.clone();
    }
    

    pub fn reset(&mut self) {
        self.reset2();
        self.currentPassRunnable = self.FIRST_RUNNABLE.clone();
        self.done = false;
    }
    

    pub fn processFirstRunnable(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(self.pathFindingLayer).getWaypointRunnableLogHelper()).start(self.pathFindingLayer.clone());
            self.reset2();
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.pathFindingLayer).getCurrentGeographicMapCellPosition();
            if geographicMapCellPosition.is_none() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Should never be running here"))));
            }
            self.pathFindingInfo = crate::java_into_object({ let java_value = (jcall!(jcall!(jcall!(self.targetPathFindingLayer).getWaypointBehavior()).getWaypoint()).getPathFindingInfo(geographicMapCellPosition.clone())).clone(); java_value });
            let mut localPathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = { let java_object = (self.pathFindingInfo).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PathFindingInfo>>>>().expect("ClassCastException").clone()) };
            self.list = { let java_value = (jcall!(jcall!(jcall!(self.targetPathFindingLayer).getWaypointBehavior()).getWaypoint()).getPathsList(geographicMapCellPosition.clone(), localPathFindingInfo.clone(), self.multipassState.clone())).clone(); java_value };
            if !match (&self.list, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.END_RUNNABLE).run();
            } else {
                self.currentPassRunnable = self.SECOND_RUNNABLE.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            self.setRunning(false);
            self.finish();
            }
        }
    }
    

    pub fn processSecondRunnable(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut geographicMapCellPosition: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::GeographicMapCellPosition::GeographicMapCellPositionDyn>>> = jcall!(self.pathFindingLayer).getCurrentGeographicMapCellPosition();
            let mut localPathFindingInfo: Option<Rc<RefCell<PathFindingInfo>>> = { let java_object = (self.pathFindingInfo).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<PathFindingInfo>>>>().expect("ClassCastException").clone()) };
            self.list = { let java_value = (jcall!(jcall!(jcall!(self.targetPathFindingLayer).getWaypointBehavior()).getWaypoint()).getPathsList(geographicMapCellPosition.clone(), localPathFindingInfo.clone(), self.multipassState.clone())).clone(); java_value };
            if !match (&self.list, &jcall!(self.basicArrayListUtil).getImmutableInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.END_RUNNABLE).run();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            self.setRunning(false);
            self.finish();
            }
        }
    }
    

    pub fn processEndRunnable(&mut self) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut waypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::WaypointBehaviorBase::WaypointBehaviorBaseDyn>>> = jcall!(self.pathFindingLayer).getWaypointBehavior();
            jcall!(waypointBehavior).setWaypointPathsList(self.list.clone());
            jcall!(jcall!(self.pathFindingLayer).getWaypointRunnableLogHelper()).end(self.pathFindingLayer.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put({ let java_field = jcall!(commonStrings).EXCEPTION.clone(); java_field }.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(commonStrings).RUN.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            self.setRunning(false);
            }
        }
        self.finish();
    }
}
impl std::ops::Deref for MultipassWaypointPathRunnable {
    type Target = WaypointPathRunnableBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for MultipassWaypointPathRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBaseDyn for MultipassWaypointPathRunnable {
    fn as_WaypointPathRunnableBase(&self) -> &crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase { &self.base }
    fn as_WaypointPathRunnableBase_mut(&mut self) -> &mut crate::org::allbinary::game::layer::WaypointPathRunnableBase::WaypointPathRunnableBase { &mut self.base }
    fn setRunning(&mut self, mut isRunning: bool) { Self::setRunning(self, isRunning) }
    fn isDone(&mut self) -> bool { Self::isDone(self) }
    fn run(&mut self) { Self::run(self) }
    fn reset(&mut self) { Self::reset(self) }
}


pub struct FirstRunnable {
    pub base: ARunnable,
    pub multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>,
}


impl FirstRunnable {
        

        pub fn new_struct(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Self {
            let mut this = Self {
            base: ARunnable::new_struct(),
            multipassWaypointPathRunnable: None,
            };
            this.multipassWaypointPathRunnable = multipassWaypointPathRunnable.clone();
            this
        }

        pub fn new(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(multipassWaypointPathRunnable))))
        }
        

        pub fn run(&mut self) {
            jcall!(self.multipassWaypointPathRunnable).processFirstRunnable();
        }
}
impl std::ops::Deref for FirstRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FirstRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for FirstRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for FirstRunnable {
    fn run(&mut self) { Self::run(self) }
}


//    private boolean first = true;
pub struct SecondRunnable {
    pub base: ARunnable,
    pub multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>,
}


impl SecondRunnable {
        

        pub fn new_struct(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Self {
            let mut this = Self {
            base: ARunnable::new_struct(),
            multipassWaypointPathRunnable: None,
            };
            this.multipassWaypointPathRunnable = multipassWaypointPathRunnable.clone();
            this
        }

        pub fn new(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(multipassWaypointPathRunnable))))
        }
        

        pub fn run(&mut self) {
            jcall!(self.multipassWaypointPathRunnable).processSecondRunnable();
        }
}
impl std::ops::Deref for SecondRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SecondRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for SecondRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for SecondRunnable {
    fn run(&mut self) { Self::run(self) }
}


pub struct EndRunnable {
    pub base: ARunnable,
    pub multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>,
}


impl EndRunnable {
        

        pub fn new_struct(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Self {
            let mut this = Self {
            base: ARunnable::new_struct(),
            multipassWaypointPathRunnable: None,
            };
            this.multipassWaypointPathRunnable = multipassWaypointPathRunnable.clone();
            this
        }

        pub fn new(mut multipassWaypointPathRunnable: Option<Rc<RefCell<MultipassWaypointPathRunnable>>>) -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct(multipassWaypointPathRunnable))))
        }
        

        pub fn run(&mut self) {
            jcall!(self.multipassWaypointPathRunnable).processEndRunnable();
        }
}
impl std::ops::Deref for EndRunnable {
    type Target = ARunnable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EndRunnable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ARunnable::ARunnableDyn for EndRunnable {
    fn as_ARunnable(&self) -> &crate::org::allbinary::thread::ARunnable::ARunnable { &self.base }
    fn as_ARunnable_mut(&mut self) -> &mut crate::org::allbinary::thread::ARunnable::ARunnable { &mut self.base }
}
impl crate::Runnable for EndRunnable {
    fn run(&mut self) { Self::run(self) }
}


