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
use crate::org::allbinary::AppletUtil::AppletUtil;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::Paintable::Paintable;
use crate::org::allbinary::input::motion::button::TouchScreenFactory::TouchScreenFactory;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PressStartMenuPaintable {
    pub base: Paintable,
    pub displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub PRESS_START: String,
    pub KEY_START: String,
    pub MENU_START: String,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub startString: String,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub flash: bool,
    pub anchor: i32,
    pub beginWidth: i32,
    pub line: i32,
}

impl UpdateMyFontInterface for PressStartMenuPaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl PressStartMenuPaintable {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Paintable::new_struct(),
            displayInfo: None,
            PRESS_START: String::new(),
            KEY_START: String::new(),
            MENU_START: String::new(),
            myFontProcessor: None,
            startString: String::new(),
            timeDelayHelper: None,
            flash: false,
            anchor: 0,
            beginWidth: 0,
            line: 0,
        };
        this.displayInfo = DisplayInfoSingleton::getInstance();
        this.PRESS_START = String::from("Press Screen To Start");
        this.KEY_START = String::from("Press or Click F2 To Begin");
        this.MENU_START = String::from("Press Start From The Menu To Begin");
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.startString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.timeDelayHelper = TimeDelayHelper::new(1100).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct())));
            if jcall!(TouchScreenFactory::getInstance()).isTouch() {
            this.startString = this.PRESS_START.clone();
        } else if AppletUtil::isAppletLoader((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())) {
            this.startString = this.KEY_START.clone();
        } else {
            this.startString = this.MENU_START.clone();
        }
        this
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.beginWidth = (jcall!(jcall!(graphics).getFont()).stringWidth(self.startString.clone()) >> 1);
        self.line = (4 * MyFontProcessor::defaultCharWidth(font.clone())) + (jcall!(font).getHeight() >> 1);
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if jcall!(self.timeDelayHelper).isTimeTNT() {
            if self.isFlash() {
                self.setFlash(false);
            } else {
                self.setFlash(true);
            }
        }
        if self.isFlash() {
            jcall!(self.myFontProcessor).process(graphics.clone());
            jcall!(graphics).drawString(self.startString.clone(), jcall!(self.displayInfo).getLastHalfWidth() - self.beginWidth, jcall!(self.displayInfo).getLastHeight() - self.line, self.anchor.clone());
        }
    }
    

    pub fn setFlash(&mut self, mut flash: bool) {
        self.flash = flash.clone();
    }
    

    pub fn isFlash(&mut self) -> bool {
        return self.flash.clone();
    }
}
impl std::ops::Deref for PressStartMenuPaintable {
    type Target = Paintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PressStartMenuPaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for PressStartMenuPaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, graphics) }
}


