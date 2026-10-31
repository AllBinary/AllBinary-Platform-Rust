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
use crate::org::allbinary::game::combat::damage::PtsDamageFloaters::PtsDamageFloaters;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::game::health::HealthBar::HealthBar;
use crate::org::allbinary::game::health::HealthBarTwodAnimation::HealthBarTwodAnimation;
use crate::org::allbinary::game::tracking::TrackingEvent::TrackingEvent;
use crate::org::allbinary::graphics::paint::NullPaintable::NullPaintable;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::layer::AllBinaryLayer::AllBinaryLayer;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use super::BuildingLayer::BuildingLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BuildingPropertiesFactory {
    pub damageFloatersPaintableInterface: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>,
}

thread_local! { pub static BuildingPropertiesFactory_instance: Option<Rc<RefCell<dyn BuildingPropertiesFactoryDyn>>> = BuildingPropertiesFactory::new().map(|reference| reference as Rc<RefCell<dyn BuildingPropertiesFactoryDyn>>); }

impl BuildingPropertiesFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            damageFloatersPaintableInterface: None,
        };
        this.damageFloatersPaintableInterface = NullPaintable::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn BuildingPropertiesFactoryDyn>>> {
        return BuildingPropertiesFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn getEfficiencyPerLevel(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> i32 {
        return 10000 / jcall!(buildingLayer).getMaxLevel() + 10000 % jcall!(buildingLayer).getMaxLevel();
    }
    

    pub fn getDamageFloaters(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> {
        let mut damageFloaters: Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> = DamageFloaters::getInstance();
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).DAMAGE_FLOATERS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            damageFloaters = PtsDamageFloaters::new(buildingLayer.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
            self.damageFloatersPaintableInterface = damageFloaters.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        } else {
            damageFloaters = DamageFloaters::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>);
        }
        return damageFloaters.clone();
    }
    

    // throws Exception
    pub fn getHealthBar(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> {
        let mut healthBar: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> = NullPaintable::getInstance().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        if jcall!(Features::getInstance()).isFeature(jcall!(GameFeatureFactory::getInstance()).HEALTH_BARS.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            healthBar = HealthBar::new(buildingLayer.clone(), jcall!(buildingLayer).getHealthInterface(), HealthBarTwodAnimation::new((buildingLayer).clone() /* as AllBinaryLayer */, jcall!(BasicHudFactory::getInstance()).BOTTOMLEFT.clone()), -1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>);
        }
        return healthBar.clone();
    }
    

    pub fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        return jcall!(StdUtil::getInstance()).createHashtable();
    }
    

    pub fn getTrackingEvent(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<TrackingEvent>>> {
        return TrackingEvent::new(buildingLayer.clone());
    }
}
pub trait BuildingPropertiesFactoryDyn {
    fn as_BuildingPropertiesFactory(&self) -> &BuildingPropertiesFactory;
    fn as_BuildingPropertiesFactory_mut(&mut self) -> &mut BuildingPropertiesFactory;
    fn getEfficiencyPerLevel(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> i32 { BuildingPropertiesFactory::getEfficiencyPerLevel(self.as_BuildingPropertiesFactory_mut(), buildingLayer) }
    fn getDamageFloaters(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::combat::damage::DamageFloaters::DamageFloatersDyn>>> { BuildingPropertiesFactory::getDamageFloaters(self.as_BuildingPropertiesFactory_mut(), buildingLayer) }
    fn getHealthBar(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>> { BuildingPropertiesFactory::getHealthBar(self.as_BuildingPropertiesFactory_mut(), buildingLayer) }
    fn getHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> { BuildingPropertiesFactory::getHashtable(self.as_BuildingPropertiesFactory_mut()) }
    fn getTrackingEvent(&mut self, mut buildingLayer: Option<Rc<RefCell<BuildingLayer>>>) -> Option<Rc<RefCell<TrackingEvent>>> { BuildingPropertiesFactory::getTrackingEvent(self.as_BuildingPropertiesFactory_mut(), buildingLayer) }
}
impl BuildingPropertiesFactoryDyn for BuildingPropertiesFactory {
    fn as_BuildingPropertiesFactory(&self) -> &BuildingPropertiesFactory { self }
    fn as_BuildingPropertiesFactory_mut(&mut self) -> &mut BuildingPropertiesFactory { self }
}


