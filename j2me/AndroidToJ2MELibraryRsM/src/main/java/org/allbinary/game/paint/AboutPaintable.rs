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
// package org.allbinary.game.paint
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::displayable::CanvasStrings::CanvasStrings;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//AndroidToJ2ME
pub struct AboutPaintable {
    pub base: Paintable,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub ABOUT: String,
    pub info: Vec<String>,
    pub developers: Vec<String>,
    pub paintableArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub charHeight: i32,
    pub aboutBeginWidth: i32,
    pub infoBeginWidth: Vec<i32>,
    pub developersBeginWidth: Vec<i32>,
    pub anchor: i32,
}

impl UpdateMyFontInterface for AboutPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}

impl AboutPaintable {
    

    pub fn getInstance(mut info: Vec<String>, mut developers: Vec<String>) -> Option<Rc<RefCell<AboutPaintable>>> {
        return AboutPaintable::new(info.clone(), developers.clone());
    }
    

    pub fn new_struct(mut info: Vec<String>, mut developers: Vec<String>) -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            displayInfoSingleton: None,
            ABOUT: String::new(),
            info: Vec::new(),
            developers: Vec::new(),
            paintableArray: Vec::new(),
            myFontProcessor: None,
            charHeight: 0,
            aboutBeginWidth: 0,
            infoBeginWidth: Vec::new(),
            developersBeginWidth: Vec::new(),
            anchor: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.ABOUT = { let java_field = jcall!(CanvasStrings::getInstance()).ABOUT.clone(); java_field }.clone();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.info = info.clone();
        this.infoBeginWidth = { let java_value = (vec![0; ((this.info.len() as i32)) as usize]).clone(); java_value };
        this.developers = developers.clone();
        this.developersBeginWidth = { let java_value = (vec![0; ((this.developers.len() as i32)) as usize]).clone(); java_value };
        this
    }

    pub fn new(mut info: Vec<String>, mut developers: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(info.clone(), developers.clone()))));
        jcall!(this).paintableArray = vec![this.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>)];
        jcall!(this).myFontProcessor = UpdateMyFontProcessor::new(this.clone().map(|reference| reference as Rc<RefCell<dyn UpdateMyFontInterface>>)).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.charHeight = { let java_value = (jcall!(font).getHeight()).clone(); java_value };
        self.aboutBeginWidth = { let java_value = ((jcall!(font).stringWidth(self.ABOUT.clone()) >> 1)).clone(); java_value };
        let mut infoSize: i32 = (self.info.len() as i32).clone();
        for index in (0..infoSize) {
            { let java_index = (index) as usize; self.infoBeginWidth[java_index] = (jcall!(font).stringWidth(self.info[(index) as usize].clone()) >> 1); };
        }
        let mut size: i32 = (self.developers.len() as i32).clone();
        for index in (0..size) {
            { let java_index = (index) as usize; self.developersBeginWidth[java_index] = (jcall!(font).stringWidth(self.developers[(index) as usize].clone()) >> 1); };
        }
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn getPaintableArrayInstance(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>> {
        return self.paintableArray.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfoSingleton).getLastHalfWidth();
        jcall!(graphics).drawString(self.ABOUT.clone(), halfWidth - self.aboutBeginWidth, 2 * self.charHeight, self.anchor.clone());
        let mut infoSize: i32 = (self.info.len() as i32).clone();
        for index in (0..infoSize) {
            jcall!(graphics).drawString(self.info[(index) as usize].clone(), halfWidth - self.infoBeginWidth[(index) as usize], (4 + index) * self.charHeight, self.anchor.clone());
        }
        let mut size: i32 = (self.developers.len() as i32).clone();
        for index in (0..size) {
            jcall!(graphics).drawString(self.developers[(index) as usize].clone(), halfWidth - self.developersBeginWidth[(index) as usize], (5 + infoSize + index) * self.charHeight, self.anchor.clone());
        }
    }
}
impl std::ops::Deref for AboutPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AboutPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for AboutPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for AboutPaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


