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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::UpgradableRTSLayerHudPaintable::UpgradableRTSLayerHudPaintable;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerCompositePaintable {
    pub base: Paintable,
    pub upgradeCost: String,
    pub downGradeCost: String,
    pub UPGRADE: String,
    pub DOWNGRADE: String,
    pub upgradableRTSLayerHudPaintable: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>,
}


impl RTSLayerCompositePaintable {
    

    pub fn new_struct(mut upgradableRTSLayerHudPaintable: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            upgradeCost: String::new(),
            downGradeCost: String::new(),
            UPGRADE: String::new(),
            DOWNGRADE: String::new(),
            upgradableRTSLayerHudPaintable: None,
        };
        this.upgradeCost = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.downGradeCost = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.UPGRADE = String::from("Up: $");
        this.DOWNGRADE = String::from("Down: $-");
        this.upgradableRTSLayerHudPaintable = upgradableRTSLayerHudPaintable.clone();
        this
    }

    pub fn new(mut upgradableRTSLayerHudPaintable: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(upgradableRTSLayerHudPaintable))))
    }
    

    pub fn update(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        if jcall!(rtsLayer).isUpgradeable() {
            { let java_arguments = (jcall!(StringMaker::new()).append(self.UPGRADE.clone()).appendint(jcall!(rtsLayer).getUpgradeCost()).toString(),); self.setUpgradeCost(java_arguments.0) };
        } else {
            { let java_arguments = (jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); self.setUpgradeCost(java_arguments.0) };
        }
        if jcall!(rtsLayer).isDowngradeable() {
            { let java_arguments = (jcall!(StringMaker::new()).append(self.DOWNGRADE.clone()).appendint(jcall!(rtsLayer).getDowngradeCost()).toString(),); self.setDownGradeCost(java_arguments.0) };
        } else {
            { let java_arguments = (jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(),); self.setDownGradeCost(java_arguments.0) };
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(graphics).drawString(self.getDownGradeCost(), jcall!(self.upgradableRTSLayerHudPaintable).textX.clone(), jcall!(self.upgradableRTSLayerHudPaintable).costY1.clone(), 0);
        jcall!(graphics).drawString(self.getUpgradeCost(), jcall!(self.upgradableRTSLayerHudPaintable).textX.clone(), jcall!(self.upgradableRTSLayerHudPaintable).costY.clone(), 0);
    }
    

    pub fn setUpgradeCost(&mut self, mut upgradeCost: String) {
        self.upgradeCost = upgradeCost.clone();
    }
    

    pub fn getUpgradeCost(&mut self) -> String {
        return self.upgradeCost.clone();
    }
    

    pub fn setDownGradeCost(&mut self, mut downGradeCost: String) {
        self.downGradeCost = downGradeCost.clone();
    }
    

    pub fn getDownGradeCost(&mut self) -> String {
        return self.downGradeCost.clone();
    }
}
impl std::ops::Deref for RTSLayerCompositePaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerCompositePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RTSLayerCompositePaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn {
    fn as_RTSLayerCompositePaintable(&self) -> &RTSLayerCompositePaintable;
    fn as_RTSLayerCompositePaintable_mut(&mut self) -> &mut RTSLayerCompositePaintable;
    fn update(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) { RTSLayerCompositePaintable::update(self.as_RTSLayerCompositePaintable_mut(), rtsLayer) }
    fn setUpgradeCost(&mut self, mut upgradeCost: String) { RTSLayerCompositePaintable::setUpgradeCost(self.as_RTSLayerCompositePaintable_mut(), upgradeCost) }
    fn getUpgradeCost(&mut self) -> String { RTSLayerCompositePaintable::getUpgradeCost(self.as_RTSLayerCompositePaintable_mut()) }
    fn setDownGradeCost(&mut self, mut downGradeCost: String) { RTSLayerCompositePaintable::setDownGradeCost(self.as_RTSLayerCompositePaintable_mut(), downGradeCost) }
    fn getDownGradeCost(&mut self) -> String { RTSLayerCompositePaintable::getDownGradeCost(self.as_RTSLayerCompositePaintable_mut()) }
}
impl RTSLayerCompositePaintableDyn for RTSLayerCompositePaintable {
    fn as_RTSLayerCompositePaintable(&self) -> &RTSLayerCompositePaintable { self }
    fn as_RTSLayerCompositePaintable_mut(&mut self) -> &mut RTSLayerCompositePaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for RTSLayerCompositePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


