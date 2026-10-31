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

//SwtToJ2ME
pub struct FullScreenPaintable {
    pub base: Paintable,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub FULLSCREEN_TEXT: String,
    pub anchor: i32,
    pub Y: i32,
    pub beginWidth: i32,
}

impl UpdateMyFontInterface for FullScreenPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl FullScreenPaintable {
    

    pub fn getInstance() -> Option<Rc<RefCell<FullScreenPaintable>>> {
        return FullScreenPaintable::new();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            displayInfo: None,
            myFontProcessor: None,
            FULLSCREEN_TEXT: String::new(),
            anchor: 0,
            Y: 0,
            beginWidth: 0,
        };
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.FULLSCREEN_TEXT = String::from("F11 - Toggle Fullscreen");
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.Y = 4 * jcall!(font).getHeight();
        self.beginWidth = (jcall!(font).stringWidth(self.FULLSCREEN_TEXT.clone()) >> 1);
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut halfWidth: i32 = jcall!(self.displayInfo).getLastHalfWidth();
        let mut height: i32 = jcall!(self.displayInfo).getLastHeight();
        jcall!(graphics).drawString(self.FULLSCREEN_TEXT.clone(), halfWidth - self.beginWidth, height - self.Y, self.anchor.clone());
    }
}
impl std::ops::Deref for FullScreenPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FullScreenPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for FullScreenPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


