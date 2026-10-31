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
// package org.allbinary.media
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
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::AllBinaryVibrationME::AllBinaryVibrationME;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryVisualDebugVibration {
    pub base: AllBinaryVibrationME,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub VIBRATING: String,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub anchor: i32,
    pub width: i32,
}

impl UpdateMyFontInterface for AllBinaryVisualDebugVibration {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl AllBinaryVisualDebugVibration {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AllBinaryVibrationME::new_struct(),
            displayInfoSingleton: None,
            timeDelayHelper: None,
            VIBRATING: String::new(),
            myFontProcessor: None,
            anchor: 0,
            width: 0,
        };
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.timeDelayHelper = TimeDelayHelper::new(i32::MAX.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.VIBRATING = String::from("Vibrating");
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.width = jcall!(font).stringWidth(self.VIBRATING.clone());
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) {
        jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = duration.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        if jcall!(self.timeDelayHelper).isTimeTNT() {
            jcall!(self.timeDelayHelper).as_TimeDelayHelper_mut().delay = i32::MAX.clone();
            jcall!(self.myFontProcessor).process(graphics.clone());
            jcall!(graphics).drawString(self.VIBRATING.clone(), jcall!(self.displayInfoSingleton).getLastHalfWidth() - (self.width >> 1), 0, self.anchor.clone());
        }
    }
}
impl std::ops::Deref for AllBinaryVisualDebugVibration {
    type Target = AllBinaryVibrationME;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryVisualDebugVibration {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationMEDyn for AllBinaryVisualDebugVibration {
    fn as_AllBinaryVibrationME(&self) -> &crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &self.base }
    fn as_AllBinaryVibrationME_mut(&mut self) -> &mut crate::org::allbinary::media::AllBinaryVibrationME::AllBinaryVibrationME { &mut self.base }
    fn vibrate(&mut self, mut duration: i32, mut r#type: i32, mut volume: i32) { Self::vibrate(self, duration, r#type, volume) }
}


