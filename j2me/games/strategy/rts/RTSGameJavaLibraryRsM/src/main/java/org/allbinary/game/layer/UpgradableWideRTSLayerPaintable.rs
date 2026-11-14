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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::RTSLayerCompositePaintable::RTSLayerCompositePaintable;
use super::UpgradableRTSLayerHudPaintable::UpgradableRTSLayerHudPaintable;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpgradableWideRTSLayerPaintable {
    pub base: RTSLayerCompositePaintable,
    pub cost: String,
}


impl UpgradableWideRTSLayerPaintable {
    

    pub fn new_struct(mut upgradableRTSLayerHudPaintable: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>) -> Self {
        let mut this = Self {
            base: RTSLayerCompositePaintable::new_struct(upgradableRTSLayerHudPaintable.clone()),
            cost: String::new(),
        };
        this.cost = { let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut upgradableRTSLayerHudPaintable: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(upgradableRTSLayerHudPaintable))))
    }
    

    pub fn update(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        { let java_arguments = (rtsLayer.clone(),); self.base.update(java_arguments.0) };
        { let java_arguments = ({ let java_argument = jcall!(StringMaker::new()).append(self.getUpgradeCost()).append({ let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone()).append(self.getDownGradeCost()).toString(); java_argument },); self.setCost(java_arguments.0) };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(graphics).drawString(self.getCost(), { let java_field = jcall!(self.upgradableRTSLayerHudPaintable).textX.clone(); java_field }.clone(), { let java_field = jcall!(self.upgradableRTSLayerHudPaintable).costY.clone(); java_field }.clone(), 0);
    }
    

    pub fn setCost(&mut self, mut cost: String) {
        self.cost = cost.clone();
    }
    

    pub fn getCost(&mut self) -> String {
        return self.cost.clone();
    }
}
impl std::ops::Deref for UpgradableWideRTSLayerPaintable {
    type Target = RTSLayerCompositePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpgradableWideRTSLayerPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn for UpgradableWideRTSLayerPaintable {
    fn as_RTSLayerCompositePaintable(&self) -> &crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintable { &self.base }
    fn as_RTSLayerCompositePaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintable { &mut self.base }
    fn update(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) { Self::update(self, rtsLayer) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for UpgradableWideRTSLayerPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for UpgradableWideRTSLayerPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


