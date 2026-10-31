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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayer;
use crate::org::allbinary::logic::java::character::CharArrayFactory::CharArrayFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::input::motion::button::CommonButtons::CommonButtons;
use super::SelectionHudPaintable::SelectionHudPaintable;
use super::RTSLayerCompositePaintable::RTSLayerCompositePaintable;
use super::UpgradableWideRTSLayerPaintable::UpgradableWideRTSLayerPaintable;
use super::RTSLayer::RTSLayer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpgradableRTSLayerHudPaintable {
    pub base: SelectionHudPaintable,
    pub PERCENT: String,
    pub rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>>,
    pub costY: i32,
    pub costY1: i32,
    pub percentCompleteX2: i32,
    pub rtsLayerCompositePaintableLateInit: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn>>>,
    pub charHeight: i32,
    pub charWidth: i32,
    pub percentComplete: i32,
    pub percentCompleteX: i32,
    pub percentCompleteArray: Vec<char>,
    pub currentTotalDigits: i32,
}

thread_local! { pub static UpgradableRTSLayerHudPaintable_instance: Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>> = UpgradableRTSLayerHudPaintable::new(); }
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for UpgradableRTSLayerHudPaintable {
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for UpgradableRTSLayerHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::init::InitInterface::InitInterface for UpgradableRTSLayerHudPaintable {
    fn init(&mut self) { self.base.init() }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for UpgradableRTSLayerHudPaintable {
    fn update(&mut self) { Self::update(self) }
}

impl UpgradableRTSLayerHudPaintable {
    

    pub fn getInstance() -> Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>> {
        return UpgradableRTSLayerHudPaintable_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: SelectionHudPaintable::new_struct(),
            PERCENT: String::new(),
            rtsLayer: None,
            costY: 0,
            costY1: 0,
            percentCompleteX2: 0,
            rtsLayerCompositePaintableLateInit: None,
            charHeight: 0,
            charWidth: 0,
            percentComplete: 0,
            percentCompleteX: 0,
            percentCompleteArray: Vec::new(),
            currentTotalDigits: 0,
        };
        this.PERCENT = String::from("%");
        this.rtsLayer = CollidableDestroyableDamageableLayer::getNullInstance();
        this.percentCompleteArray = jcall!(CharArrayFactory::getInstance()).getZeroCharArray();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.updateMeasurement(graphics.clone());
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.charHeight = jcall!(font).getHeight();
        self.charWidth = MyFontProcessor::defaultCharWidth(font.clone());
    }
    

    pub fn update(&mut self) {
        self.base.update();
        let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
        self.costY = (self.y + jcall!(commonButtons).STANDARD_BUTTON_SIZE);
        self.costY1 = (self.y + jcall!(commonButtons).STANDARD_BUTTON_SIZE - self.charHeight);
        self.percentCompleteX2 = self.imageX + jcall!(commonButtons).STANDARD_BUTTON_SIZE - self.charWidth;
        let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if jcall!(displayInfoSingleton).getLastWidth() > 320 {
            self.rtsLayerCompositePaintableLateInit = UpgradableWideRTSLayerPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn>>);
        } else {
            self.rtsLayerCompositePaintableLateInit = RTSLayerCompositePaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn>>);
        }
    }
    

    pub fn updateSelectionInfo(&mut self) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.getRtsLayer()).clone() /* as RTSLayer */;
        jcall!(self.rtsLayerCompositePaintableLateInit).update(rtsLayer.clone());
        { let java_arguments = (jcall!(rtsLayer).as_CollidableDestroyableDamageableLayer_mut().getVerticleBuildAnimationInterface(),); self.setAnimationInterface(java_arguments.0) };
        { let java_arguments = (jcall!(rtsLayer).getName(),); self.setName(java_arguments.0) };
    }
    

    pub fn updateInfo(&mut self) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (self.getRtsLayer()).clone() /* as RTSLayer */;
        self.percentComplete = jcall!(rtsLayer).as_CollidableDestroyableDamageableLayer_mut().getPercentComplete();
        if self.percentComplete < 10 {
            self.percentCompleteX = 32;
        } else if self.percentComplete < 100 {
            self.percentCompleteX = 24;
        } else {
            self.percentCompleteX = 16;
        }
        self.percentCompleteArray = jcall!(self.getPrimitiveLongUtil()).getCharArray(self.percentComplete.clone());
        self.currentTotalDigits = jcall!(self.getPrimitiveLongUtil()).getCurrentTotalDigits();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paint(graphics.clone());
        jcall!(self.rtsLayerCompositePaintableLateInit).paint(graphics.clone());
        jcall!(graphics).drawChars(self.percentCompleteArray.clone(), 0, self.currentTotalDigits.clone(), self.imageX + self.percentCompleteX, self.costY.clone(), 0);
        jcall!(graphics).drawString(self.PERCENT.clone(), self.percentCompleteX2.clone(), self.costY.clone(), 0);
        jcall!(self.getAnimationInterface()).paintXY(graphics.clone(), self.imageX.clone(), self.y.clone());
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        self.rtsLayer = rtsLayer.clone();
    }
    

    pub fn getRtsLayer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>> {
        return self.rtsLayer.clone();
    }
}
impl std::ops::Deref for UpgradableRTSLayerHudPaintable {
    type Target = SelectionHudPaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpgradableRTSLayerHudPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn for UpgradableRTSLayerHudPaintable {
    fn as_SelectionHudPaintable(&self) -> &crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &self.base }
    fn as_SelectionHudPaintable_mut(&mut self) -> &mut crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
    fn updateSelectionInfo(&mut self) { Self::updateSelectionInfo(self) }
    fn updateInfo(&mut self) { Self::updateInfo(self) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for UpgradableRTSLayerHudPaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base.base }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for UpgradableRTSLayerHudPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base.base }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintable::setBasicColorP(crate::org::allbinary::game::layer::SelectionHudPaintable::SelectionHudPaintableDyn::as_SelectionHudPaintable_mut(self), p1) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


