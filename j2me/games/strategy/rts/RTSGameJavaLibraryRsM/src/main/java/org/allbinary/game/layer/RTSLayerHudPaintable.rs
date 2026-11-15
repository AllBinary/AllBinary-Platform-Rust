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
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPart;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use super::SelectionHudPaintable::SelectionHudPaintable;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RTSLayerHudPaintable {
    pub base: SelectionHudPaintable,
    pub weaponProperties: Vec<String>,
    pub rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub costY: i32,
    pub costY1: i32,
    pub fontHeight: i32,
}

pub static RTSLayerHudPaintable_instance: crate::JavaStatic<Option<Rc<RefCell<RTSLayerHudPaintable>>>> = crate::JavaStatic::new(|| RTSLayerHudPaintable::new());
impl UpdateMyFontInterface for RTSLayerHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for RTSLayerHudPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for RTSLayerHudPaintable {
    fn init(&mut self) { crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable::init(crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn::as_InitUpdatePaintable_mut(self)) }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for RTSLayerHudPaintable {
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}

impl RTSLayerHudPaintable {
    

    pub fn getInstance() -> Option<Rc<RefCell<RTSLayerHudPaintable>>> {
        return RTSLayerHudPaintable_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SelectionHudPaintable::new_struct(),
            weaponProperties: Vec::new(),
            rtsLayer: None,
            costY: 0,
            costY1: 0,
            fontHeight: 0,
        };
        this.weaponProperties = jcall!(StringUtil::getInstance()).getArrayInstance();
        this.rtsLayer = CollidableDestroyableDamageableLayer::getNullInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.updateMeasurement(java_arguments.0) };
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.fontHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.costY1 = { let java_value = ((self.y + (((self.weaponProperties.len() as i32) + 1) * self.fontHeight))).clone(); java_value };
        if !AndroidUtil::isAndroid() {
            self.costY = self.costY1.clone();
        } else {
            self.costY = { let java_value = ((self.y + (((self.weaponProperties.len() as i32) + 2) * self.fontHeight))).clone(); java_value };
        }
    }
    

    pub fn updateSelectionInfo(&mut self) {
        { let java_arguments = ({ let java_argument = jcall!(self.getRtsLayer()).getName(); java_argument },); self.setName(java_arguments.0) };
        let mut partInterface: Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::part::weapon::BasicWeaponPart::BasicWeaponPartDyn>>> { panic!("ClassCastException: a Rust dyn PartInterface cannot be cast to BasicWeaponPart") })();
        self.weaponProperties = { let java_value = (jcall!(jcall!(partInterface).getWeaponProperties()).toStringArray()).clone(); java_value };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
        let mut size: i32 = (self.weaponProperties.len() as i32).clone();
        for index in (0..size) {
            jcall!(graphics).drawString(self.weaponProperties[(index) as usize].clone(), self.textX.clone(), self.y + ((index + 1) * self.fontHeight), 0);
        }
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        self.rtsLayer = rtsLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>);
    }
    

    pub fn getRtsLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        return self.rtsLayer.clone();
    }
}
impl std::ops::Deref for RTSLayerHudPaintable {
    type Target = SelectionHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RTSLayerHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for RTSLayerHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base }
    fn updateSelectionInfo(&mut self) { Self::updateSelectionInfo(self) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for RTSLayerHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base }
    fn update(&mut self) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::update(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self)) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for RTSLayerHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::setBasicColorP(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self), p1) }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for RTSLayerHudPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


