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
// package org.allbinary.game.layer.hud.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureUtil::OpenGLFeatureUtil;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::logic::math::MathUtil::MathUtil;
use crate::org::allbinary::logic::math::PrimitiveLongSingleton::PrimitiveLongSingleton;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VelocityWidget {
    pub base: BasicHud,
    pub KILOMETERS_PER_HOUR_STR: Vec<char>,
    pub totalChars: i32,
    pub velocity: i32,
    pub maxVelocity: i32,
    pub string: Vec<char>,
    pub totalDigits: i32,
    pub powerOfTenVelocity: i32,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub offset: i32,
    pub offset2: i32,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for VelocityWidget {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl VelocityWidget {
    

    pub fn new_struct(mut powerOfTenVelocity: i32, mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, basicColor.clone()),
            KILOMETERS_PER_HOUR_STR: Vec::new(),
            totalChars: 0,
            velocity: 0,
            maxVelocity: 0,
            string: Vec::new(),
            totalDigits: 0,
            powerOfTenVelocity: 0,
            primitiveLongUtil: None,
            offset: 0,
            offset2: 0,
        };
        this.KILOMETERS_PER_HOUR_STR = vec![' ', 'k', 'm', '/', 'h'];
        this.totalChars = (this.KILOMETERS_PER_HOUR_STR.len() as i32).clone();
        this.string = jcall!(PrimitiveLongSingleton::getInstance()).ZERO.clone();
        this.totalDigits = 1;
        this.offset = 0;
        this.offset2 = 0;
        // super(...) is in the struct literal above
        this.powerOfTenVelocity = powerOfTenVelocity.clone();
        this.maxVelocity = powerOfTenVelocity.clone();
        this.velocity = 0;
        this.primitiveLongUtil = PrimitiveLongUtil::createPowerOfTen(powerOfTenVelocity.clone());
        this.updateMaxHeight = 14;
        this
    }

    pub fn new(mut powerOfTenVelocity: i32, mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(powerOfTenVelocity, location, direction, basicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.updateMaxWidth = jcall!(font).getSize() * (5 + jcall!(MathUtil::getInstance()).getTotalDigits(self.powerOfTenVelocity.clone()) + 1);
        self.base.updateMeasurement(graphics.clone());
        self.offset = MyFontProcessor::defaultStringWidth(font.clone(), jcall!(self.primitiveLongUtil).getMaxDigits()) + MyFontProcessor::defaultStringWidth(font.clone(), 2);
        self.offset2 = self.offset - MyFontProcessor::defaultStringWidth(font.clone(), self.totalDigits.clone()) - MyFontProcessor::defaultStringWidth(font.clone(), 2);
    }
    

    pub fn get(&mut self) -> i32 {
        return self.velocity.clone();
    }
    

    pub fn add(&mut self, mut value: i32) {
        self.set(self.velocity + value);
    }
    

    pub fn set(&mut self, mut value: i32) {
        let mut lastVelocity: i32 = self.velocity.clone();
        self.velocity = value.clone();
        if self.velocity > self.maxVelocity {
            self.velocity = 0;
        }
        if lastVelocity != self.velocity {
            self.string = jcall!(self.primitiveLongUtil).getCharArray(self.velocity * 18);
            if jcall!(OpenGLFeatureUtil::getInstance()).isAnyThreed() {
                self.totalDigits = jcall!(self.primitiveLongUtil).getCurrentTotalDigits() + 1;
            } else {
                self.totalDigits = jcall!(self.primitiveLongUtil).getCurrentTotalDigits();
            }
            self.myFontProcessor = self.updateMyFontProcessor.clone();
        }
    }
    

    pub fn reduce(&mut self, mut value: i32) {
        self.set(self.velocity - value);
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paintDXY(graphics.clone(), self.string.clone(), 0, self.totalDigits.clone(), self.KILOMETERS_PER_HOUR_STR.clone(), 0, self.totalChars.clone(), self.offset2.clone(), self.offset.clone());
    }
}
impl std::ops::Deref for VelocityWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VelocityWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for VelocityWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}


