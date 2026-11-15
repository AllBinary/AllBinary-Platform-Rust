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

pub static UpgradableRTSLayerHudPaintable_instance: crate::JavaStatic<Option<Rc<RefCell<UpgradableRTSLayerHudPaintable>>>> = crate::JavaStatic::new(|| UpgradableRTSLayerHudPaintable::new());
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for UpgradableRTSLayerHudPaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for UpgradableRTSLayerHudPaintable {
    fn init(&mut self) { crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable::init(crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn::as_InitUpdatePaintable_mut(self)) }
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
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.updateMeasurement(java_arguments.0) };
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.charHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.charWidth = { let java_value = (MyFontProcessor::defaultCharWidth(font.clone())).clone(); java_value };
    }
    

    pub fn update(&mut self) {
        self.base.update();
        let mut commonButtons: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::getInstance();
        self.costY = { let java_value = ((self.y + { let java_field = jcall!(commonButtons).STANDARD_BUTTON_SIZE.clone(); java_field })).clone(); java_value };
        self.costY1 = { let java_value = ((self.y + { let java_field = jcall!(commonButtons).STANDARD_BUTTON_SIZE.clone(); java_field } - self.charHeight)).clone(); java_value };
        self.percentCompleteX2 = { let java_value = (self.imageX + { let java_field = jcall!(commonButtons).STANDARD_BUTTON_SIZE.clone(); java_field } - self.charWidth).clone(); java_value };
        let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        if jcall!(displayInfoSingleton).getLastWidth() > 320 {
            self.rtsLayerCompositePaintableLateInit = UpgradableWideRTSLayerPaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn>>);
        } else {
            self.rtsLayerCompositePaintableLateInit = RTSLayerCompositePaintable::new(None /* Java this: a &mut self method has no Rc of itself */).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayerCompositePaintable::RTSLayerCompositePaintableDyn>>);
        }
    }
    

    pub fn updateSelectionInfo(&mut self) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to RTSLayer") })();
        jcall!(self.rtsLayerCompositePaintableLateInit).update(rtsLayer.clone());
        { let java_arguments = (jcall!(rtsLayer).getVerticleBuildAnimationInterface(),); self.setAnimationInterface(java_arguments.0) };
        { let java_arguments = (jcall!(rtsLayer).getName(),); self.setName(java_arguments.0) };
    }
    

    pub fn updateInfo(&mut self) {
        let mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>> { panic!("ClassCastException: a Rust dyn CollidableDestroyableDamageableLayer cannot be cast to RTSLayer") })();
        self.percentComplete = { let java_value = (jcall!(rtsLayer).as_RTSLayer_mut().getPercentComplete()).clone(); java_value };
        if self.percentComplete < 10 {
            self.percentCompleteX = 32;
        } else if self.percentComplete < 100 {
            self.percentCompleteX = 24;
        } else {
            self.percentCompleteX = 16;
        }
        self.percentCompleteArray = { let java_value = (jcall!(self.getPrimitiveLongUtil()).getCharArray(self.percentComplete.clone())).clone(); java_value };
        self.currentTotalDigits = { let java_value = (jcall!(self.getPrimitiveLongUtil()).getCurrentTotalDigits()).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(),); self.base.paint(java_arguments.0) };
        jcall!(self.rtsLayerCompositePaintableLateInit).as_RTSLayerCompositePaintable_mut().paint(graphics.clone());
        jcall!(graphics).drawChars(self.percentCompleteArray.clone(), 0, self.currentTotalDigits.clone(), self.imageX + self.percentCompleteX, self.costY.clone(), 0);
        jcall!(graphics).drawString(self.PERCENT.clone(), self.percentCompleteX2.clone(), self.costY.clone(), 0);
        jcall!(self.getAnimationInterface()).paintXY(graphics.clone(), self.imageX.clone(), self.y.clone());
    }
    

    pub fn setRtsLayer(&mut self, mut rtsLayer: Option<Rc<RefCell<dyn crate::org::allbinary::game::layer::RTSLayer::RTSLayerDyn>>>) {
        self.rtsLayer = rtsLayer.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::layer::special::CollidableDestroyableDamageableLayer::CollidableDestroyableDamageableLayerDyn>>);
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
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for UpgradableRTSLayerHudPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for UpgradableRTSLayerHudPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}


