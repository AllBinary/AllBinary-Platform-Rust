//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
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
// package org.allbinary.game.paint.help
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HelpPaintable {
    pub base: Paintable,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub updateMyFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub anchor: i32,
    pub title: String,
    pub inputInfo: Vec<String>,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub titleBeginWidth: i32,
    pub beginWidthArray: Vec<i32>,
    pub charHeight: i32,
}

impl UpdateMyFontInterface for HelpPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl HelpPaintable {
    

    pub fn new_struct(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            displayInfo: None,
            updateMyFontProcessor: None,
            myFontProcessor: None,
            anchor: 0,
            title: String::new(),
            inputInfo: Vec::new(),
            basicColor: None,
            titleBeginWidth: 0,
            beginWidthArray: Vec::new(),
            charHeight: 0,
        };
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.myFontProcessor = this.updateMyFontProcessor.clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.inputInfo = jcall!(StringUtil::getInstance()).getArrayInstance();
        this.beginWidthArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone(); java_field }.clone();
        this.title = title.clone();
        this.basicColor = basicColor.clone();
        this
    }

    pub fn new(mut title: String, mut backgroundBasicColor: Option<Rc<RefCell<BasicColor>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(title.clone(), backgroundBasicColor.clone(), basicColor.clone()))));
        jcall!(this).updateMyFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.titleBeginWidth = { let java_value = ((jcall!(font).stringWidth(self.title.clone()) >> 1)).clone(); java_value };
        self.charHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        let mut size: i32 = (self.inputInfo.len() as i32).clone();
        for index in (0..size) {
            { let java_index = (index) as usize; self.beginWidthArray[java_index] = (jcall!(font).stringWidth(self.inputInfo[(index) as usize].clone()) >> 1); };
        }
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn setInputInfoP(&mut self, mut inputInfo: Vec<String>) {
        self.inputInfo = inputInfo.clone();
        self.beginWidthArray = { let java_value = (vec![0; ((self.inputInfo.len() as i32)) as usize]).clone(); java_value };
        self.myFontProcessor = self.updateMyFontProcessor.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        jcall!(graphics).setColor(jcall!(self.basicColor).intValue());
        jcall!(graphics).drawString(self.title.clone(), halfWidth - self.titleBeginWidth, self.charHeight.clone(), self.anchor.clone());
        let mut size: i32 = (self.inputInfo.len() as i32).clone();
        for index in (0..size) {
            jcall!(graphics).drawString(self.inputInfo[(index) as usize].clone(), halfWidth - self.beginWidthArray[(index) as usize], (index + 3) * self.charHeight, self.anchor.clone());
        }
    }
}
impl std::ops::Deref for HelpPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HelpPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HelpPaintableDyn: crate::org::allbinary::graphics::paint::Paintable::PaintableDyn + crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface {
    fn as_HelpPaintable(&self) -> &HelpPaintable;
    fn as_HelpPaintable_mut(&mut self) -> &mut HelpPaintable;
    fn setInputInfoP(&mut self, mut inputInfo: Vec<String>) { HelpPaintable::setInputInfoP(self.as_HelpPaintable_mut(), inputInfo) }
}
impl HelpPaintableDyn for HelpPaintable {
    fn as_HelpPaintable(&self) -> &HelpPaintable { self }
    fn as_HelpPaintable_mut(&mut self) -> &mut HelpPaintable { self }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for HelpPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for HelpPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


