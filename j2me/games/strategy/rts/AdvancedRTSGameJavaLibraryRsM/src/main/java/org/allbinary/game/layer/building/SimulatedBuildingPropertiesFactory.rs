// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.game.layer.building
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloaters;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::game::tracking::TrackingEventHandler::TrackingEventHandler;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BuildingPropertiesFactory::BuildingPropertiesFactory;
use super::BuildingLayer::BuildingLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimulatedBuildingPropertiesFactory {
    pub base: BuildingPropertiesFactory,
}

thread_local! { pub static SimulatedBuildingPropertiesFactory_instanceC: Option<Rc<RefCell<SimulatedBuildingPropertiesFactory>>> = SimulatedBuildingPropertiesFactory::new(); }

impl SimulatedBuildingPropertiesFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BuildingPropertiesFactory::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SimulatedBuildingPropertiesFactory>>> {
        return SimulatedBuildingPropertiesFactory_instanceC.with(Clone::clone).clone();
    }
    

    pub fn getEfficiencyPerLevel(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> i32 {
        return 0;
    }
    

    pub fn getDamageFloaters(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> {
        let mut damageFloaters: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> = DamageFloaters::getInstance();
        self.damageFloatersPaintableInterface = damageFloaters.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        return damageFloaters.clone();
    }
    

    // throws Exception
    pub fn getHealthBar(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        return NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
    }
    

    pub fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).NULL_TABLE.clone();
    }
    

    pub fn getTrackingEvent(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<TrackingEvent>>> {
        return TrackingEvent::new(TrackingEventHandler::getInstance());
    }
}
impl std::ops::Deref for SimulatedBuildingPropertiesFactory {
    type Target = BuildingPropertiesFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimulatedBuildingPropertiesFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::building::BuildingPropertiesFactory::BuildingPropertiesFactoryDyn for SimulatedBuildingPropertiesFactory {
    fn as_BuildingPropertiesFactory(&self) -> &crate::org::allbinary::game::layer::building::BuildingPropertiesFactory::BuildingPropertiesFactory { &self.base }
    fn as_BuildingPropertiesFactory_mut(&mut self) -> &mut crate::org::allbinary::game::layer::building::BuildingPropertiesFactory::BuildingPropertiesFactory { &mut self.base }
    fn getEfficiencyPerLevel(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> i32 { Self::getEfficiencyPerLevel(self, buildingLayer) }
    fn getDamageFloaters(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> { Self::getDamageFloaters(self, buildingLayer) }
    fn getHealthBar(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { Self::getHealthBar(self, buildingLayer) }
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { Self::getHashtable(self) }
    fn getTrackingEvent(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<TrackingEvent>>> { Self::getTrackingEvent(self, buildingLayer) }
}


