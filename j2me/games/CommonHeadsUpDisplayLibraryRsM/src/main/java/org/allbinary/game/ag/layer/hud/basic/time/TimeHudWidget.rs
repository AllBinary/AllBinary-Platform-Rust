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
// package org.allbinary.game.ag.layer.hud.basic.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::logic::math::PrimitiveLongSingleton::PrimitiveLongSingleton;
use super::Timer::Timer;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeHudWidget {
    pub base: BasicHud,
    pub TIME_STRING: String,
    pub TIME_CHAR_ARRAY: Vec<char>,
    pub offset: i32,
    pub string: Vec<char>,
    pub totalDigits: i32,
    pub timer: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::time::Timer::TimerDyn>>>,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for TimeHudWidget {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl TimeHudWidget {
    

    // throws Exception
    pub fn getInstance(mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut timer: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::time::Timer::TimerDyn>>>) -> Option<Rc<RefCell<TimeHudWidget>>> {
        return TimeHudWidget::new(location.clone(), direction.clone(), 2, basicColor.clone(), timer.clone());
    }
    

    pub fn new_struct(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut timer: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::time::Timer::TimerDyn>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()),
            TIME_STRING: String::new(),
            TIME_CHAR_ARRAY: Vec::new(),
            offset: 0,
            string: Vec::new(),
            totalDigits: 0,
            timer: None,
        };
        this.TIME_STRING = String::from("Time ");
        this.TIME_CHAR_ARRAY = vec!['T', 'i', 'm', 'e', ' '];
        this.string = jcall!(PrimitiveLongSingleton::getInstance()).ZERO.clone();
        this.totalDigits = 1;
        // super(...) is in the struct literal above
        this.timer = timer.clone();
        this.set();
        this.updateMaxHeight = 14;
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut timer: Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::time::Timer::TimerDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(location, direction, bufferZone, basicColor, timer))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.updateMaxWidth = jcall!(font).getSize() * 5;
        self.offset = jcall!(font).stringWidth(self.TIME_STRING.clone()) + MyFontProcessor::defaultStringWidth(font.clone(), 3);
        self.base.updateMeasurement(graphics.clone());
    }
    

    pub fn update(&mut self) {
        jcall!(self.timer).update();
        self.set();
    }
    

    pub fn set(&mut self) {
        self.string = jcall!(self.timer).getTimeChars();
        self.totalDigits = jcall!(self.timer).getCurrentTotalDigits();
    }
    

    pub fn getTimer(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::game::ag::layer::hud::basic::time::Timer::TimerDyn>>> {
        return self.timer.clone();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paintDX(graphics.clone(), self.TIME_CHAR_ARRAY.clone(), 0, (self.TIME_CHAR_ARRAY.len() as i32).clone(), self.string.clone(), 0, self.totalDigits.clone(), self.offset.clone());
    }
}
impl std::ops::Deref for TimeHudWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TimeHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for TimeHudWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}


