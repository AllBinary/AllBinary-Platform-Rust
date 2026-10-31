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
// package org.allbinary.game.layer.waypoint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::layer::PathFindingLayerInterface::PathFindingLayerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WaypointRunnableLogHelper {
}

thread_local! { pub static WaypointRunnableLogHelper_instance: Option<Rc<RefCell<dyn WaypointRunnableLogHelperDyn>>> = WaypointRunnableLogHelper::new().map(|reference| reference as Rc<RefCell<dyn WaypointRunnableLogHelperDyn>>); }

impl WaypointRunnableLogHelper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn WaypointRunnableLogHelperDyn>>> {
        return WaypointRunnableLogHelper_instance.with(Clone::clone).clone();
    }
    

    pub fn start(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
    

    pub fn end(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) {
    }
}
pub trait WaypointRunnableLogHelperDyn {
    fn as_WaypointRunnableLogHelper(&self) -> &WaypointRunnableLogHelper;
    fn as_WaypointRunnableLogHelper_mut(&mut self) -> &mut WaypointRunnableLogHelper;
    fn start(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointRunnableLogHelper::start(self.as_WaypointRunnableLogHelper_mut(), pathFindingLayerInterface) }
    fn end(&mut self, mut pathFindingLayerInterface: Option<Rc<RefCell<dyn PathFindingLayerInterface>>>) { WaypointRunnableLogHelper::end(self.as_WaypointRunnableLogHelper_mut(), pathFindingLayerInterface) }
}
impl WaypointRunnableLogHelperDyn for WaypointRunnableLogHelper {
    fn as_WaypointRunnableLogHelper(&self) -> &WaypointRunnableLogHelper { self }
    fn as_WaypointRunnableLogHelper_mut(&mut self) -> &mut WaypointRunnableLogHelper { self }
}


