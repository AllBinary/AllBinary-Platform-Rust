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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
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

//J2SEForJ2ME
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
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
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
        this.ABOUT = jcall!(CanvasStrings::getInstance()).ABOUT.clone();
        this.paintableArray = vec![this];
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.info = info.clone();
        this.infoBeginWidth = vec![0; ((this.info.len() as i32)) as usize];
        this.developers = developers.clone();
        this.developersBeginWidth = vec![0; ((this.developers.len() as i32)) as usize];
        this
    }

    pub fn new(mut info: Vec<String>, mut developers: Vec<String>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(info, developers))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.charHeight = jcall!(font).getHeight();
        self.aboutBeginWidth = (jcall!(font).stringWidth(self.ABOUT.clone()) >> 1);
        let mut infoSize: i32 = (self.info.len() as i32).clone();
        for index in (0..infoSize) {
            self.infoBeginWidth[(index) as usize] = (jcall!(font).stringWidth(self.info[(index) as usize].clone()) >> 1);
        }
        let mut size: i32 = (self.developers.len() as i32).clone();
        for index in (0..size) {
            self.developersBeginWidth[(index) as usize] = (jcall!(font).stringWidth(self.developers[(index) as usize].clone()) >> 1);
        }
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn getPaintableArrayInstance(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::graphics::paint::Paintable::PaintableDyn>>>> {
        return self.paintableArray.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
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
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


