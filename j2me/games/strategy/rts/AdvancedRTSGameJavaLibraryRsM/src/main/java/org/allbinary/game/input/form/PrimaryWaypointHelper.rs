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
use crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;
use crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PrimaryWaypointHelper {
    pub waypointLayer: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>,
}

thread_local! { pub static PrimaryWaypointHelper_instance: Option<Rc<RefCell<PrimaryWaypointHelper>>> = PrimaryWaypointHelper::new(); }

impl PrimaryWaypointHelper {
    

    pub fn getInstance() -> Option<Rc<RefCell<PrimaryWaypointHelper>>> {
        return PrimaryWaypointHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            waypointLayer: None,
        };
        this.waypointLayer = crate::org::allbinary::game::layer::NullPathFindingLayer::NullPathFindingLayer_NULL_PATH_FINDING_LAYER.with(Clone::clone).clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getWaypointLayer(&mut self) -> Option<Rc<RefCell<dyn PathFindingLayerInterface>>> {
        return self.waypointLayer.clone();
    }
    

    pub fn setWaypointLayer(&mut self, mut waypointLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::waypoint::WaypointLayer::WaypointLayerDyn>>>) {
        self.waypointLayer = waypointLayer.clone();
    }
}

