// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
//  
// By agreeing to this license you and any business entity you represent are 
// legally bound to the AllBinary Open License Version 1 legal agreement. 
//  
// You may obtain the AllBinary Open License Version 1 legal agreement from 
// AllBinary or the root directory of AllBinary's AllBinary Platform repository. 
//  
// Created By: Travis Berthelot  
// Generated Code Do Not Modify
// package org.allbinary.game.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::waypoint::WorkWaypoint::WorkWaypoint;
use crate::org::allbinary::logic::math::SmallIntegerSingletonFactory::SmallIntegerSingletonFactory;
use crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMap;
use crate::org::allbinary::util::ABHashtable::ABHashtable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvancedRTSLayerUtil {
}

thread_local! { pub static AdvancedRTSLayerUtil_instance: Option<Rc<RefCell<AdvancedRTSLayerUtil>>> = AdvancedRTSLayerUtil::new(); }

impl AdvancedRTSLayerUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AdvancedRTSLayerUtil>>> {
        return AdvancedRTSLayerUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn createFakeRTSLayerHashtable(&mut self, mut baseRaceTrackGeographicMap: Option<Rc<RefCell<dyn crate::org::allbinary::media::graphics::geography::map::racetrack::BaseRaceTrackGeographicMap::BaseRaceTrackGeographicMapDyn>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut hashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(RTSLayerUtil::RTSLayerUtil::getInstance()).createFakeRTSLayerHashtable(baseRaceTrackGeographicMap.clone());
        jcall!(hashtable).put(crate::java_into_object(crate::org::allbinary::game::layer::waypoint::WorkWaypoint::WorkWaypoint_ID.with(Clone::clone).clone()), crate::java_into_object(jcall!(SmallIntegerSingletonFactory::getInstance()).getAt(50)));
        return hashtable.clone();
    }
}

