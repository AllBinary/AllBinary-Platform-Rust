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
// package org.allbinary.game.input.form
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayer;
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::RTSLayer::RTSLayer;
use crate::org::allbinary::game::layer::RTSLayerEvent::RTSLayerEvent;
use crate::org::allbinary::game::layer::unit::UnitLayer::UnitLayer;
use crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehavior;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AssignWaypointsUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub WAYPOINT_EVENT: Option<Rc<RefCell<RTSLayerEvent>>>,
}

thread_local! { pub static AssignWaypointsUtil_instance: Option<Rc<RefCell<AssignWaypointsUtil>>> = AssignWaypointsUtil::new(); }

impl AssignWaypointsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            WAYPOINT_EVENT: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.WAYPOINT_EVENT = RTSLayerEvent::new(crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AssignWaypointsUtil>>> {
        return AssignWaypointsUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn set(&mut self, mut unitLayer: Option<Rc<RefCell<UnitLayer>>>, mut ownerAdvancedRTSGameLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::AdvancedRTSGameLayer::AdvancedRTSGameLayerDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(jcall!(ownerAdvancedRTSGameLayer).getWaypointBehavior()).getOwnedWaypointList();
            let mut unitWaypointBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::unit::UnitWaypointBehavior::UnitWaypointBehaviorDyn>>> = (jcall!(unitLayer).getWaypointBehavior()).clone() /* as UnitWaypointBehavior */;
            for index in (0..=jcall!(list).size() - 1).rev() {
                let mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = { let java_object = (jcall!(list).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>>().expect("ClassCastException").clone()) };
                jcall!(self.WAYPOINT_EVENT).setRtsLayer(waypointLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
                jcall!(unitWaypointBehavior).onWaypointEvent(self.WAYPOINT_EVENT.clone());
            }
            let mut waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>> = jcall!(PrimaryWaypointHelper::PrimaryWaypointHelper::getInstance()).getWaypointLayer();
            if !match (&waypointLayer, &crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.WAYPOINT_EVENT).setRtsLayer(waypointLayer.clone().map(|reference| reference as Rc<RefCell<dyn PathFindingLayerInterface>>));
                jcall!(unitWaypointBehavior).onWaypointEvent(self.WAYPOINT_EVENT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            jcall!(self.logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("set"), crate::java_into_object(e.clone()));
            }
        }
    }
}

