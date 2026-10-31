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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//J2SEForJ2ME
pub struct OwnershipPaintable {
    pub base: Paintable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub COPYRIGHT: String,
    pub basicColor: Option<Rc<RefCell<BasicColor>>>,
    pub color: i32,
    pub anchor: i32,
    pub COPYRIGHT_Y: i32,
    pub beginWidth: i32,
}

impl UpdateMyFontInterface for OwnershipPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl OwnershipPaintable {
    

    pub fn getInstance() -> Option<Rc<RefCell<OwnershipPaintable>>> {
        return OwnershipPaintable::new();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            logUtil: None,
            displayInfo: None,
            myFontProcessor: None,
            COPYRIGHT: String::new(),
            basicColor: None,
            color: 0,
            anchor: 0,
            COPYRIGHT_Y: 0,
            beginWidth: 0,
        };
        this.logUtil = LogUtil::getInstance();
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.COPYRIGHT = String::from("AllBinary Copyright (c) 2011");
        this.basicColor = jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        this.color = jcall!(this.basicColor).intValue();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.COPYRIGHT_Y = 2 * jcall!(font).getHeight();
        self.beginWidth = (jcall!(font).stringWidth(self.COPYRIGHT.clone()) >> 1);
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        self.basicColor = basicColor.clone();
        self.color = jcall!(basicColor).intValue();
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColor.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(self.color.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        let mut height: i32 = jcall!(self.displayInfo).getLastHeight();
        jcall!(graphics).drawString(self.COPYRIGHT.clone(), halfWidth - self.beginWidth, height - self.COPYRIGHT_Y, self.anchor.clone());
    }
}
impl std::ops::Deref for OwnershipPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OwnershipPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for OwnershipPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


