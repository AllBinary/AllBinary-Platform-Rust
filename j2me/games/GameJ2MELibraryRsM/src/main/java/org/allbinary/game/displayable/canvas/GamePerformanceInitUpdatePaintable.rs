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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::canvas::GameStatisticsFactory::GameStatisticsFactory;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct GamePerformanceInitUpdatePaintable {
    pub base: InitUpdatePaintable,
    pub halfHeight: i32,
    pub yArray: Vec<i32>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub baseRefreshHelperCharArray: Vec<Vec<char>>,
    pub defaultStringWidth: i32,
    pub RED: i32,
}

impl UpdateMyFontInterface for GamePerformanceInitUpdatePaintable {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}
impl crate::org::allbinary::graphics::paint::InitUpdatePaintableInterface::InitUpdatePaintableInterface for GamePerformanceInitUpdatePaintable {
}
impl crate::org::allbinary::init::InitInterface::InitInterface for GamePerformanceInitUpdatePaintable {
    fn init(&mut self) { Self::init(self) }
}
impl crate::org::allbinary::init::UpdateInterface::UpdateInterface for GamePerformanceInitUpdatePaintable {
    fn update(&mut self) { Self::update(self) }
}

impl GamePerformanceInitUpdatePaintable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: InitUpdatePaintable::new_struct(),
            halfHeight: 0,
            yArray: Vec::new(),
            myFontProcessor: None,
            baseRefreshHelperCharArray: Vec::new(),
            defaultStringWidth: 0,
            RED: 0,
        };
        this.halfHeight = jcall!(DisplayInfoSingleton::getInstance()).getLastHalfHeight();
        this.yArray = vec![this.halfHeight + 30, this.halfHeight + 30, this.halfHeight + 30 + 15, this.halfHeight + 30 + 15, this.halfHeight + 30 + 30, this.halfHeight + 30 + 30, this.halfHeight + 30 + 45, this.halfHeight + 30 + 45];
        this.baseRefreshHelperCharArray = vec![vec!['\0'; (0) as usize]; (0) as usize];
        this.RED = jcall!({ let java_field = jcall!(BasicColorFactory::getInstance()).RED.clone(); java_field }).intValue();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.defaultStringWidth = { let java_value = (MyFontProcessor::defaultStringWidth(font.clone(), 2)).clone(); java_value };
        self.myFontProcessor = { let java_value = (MyFontProcessor::getInstance()).clone(); java_value };
    }
    

    pub fn init(&mut self) {
    }
    

    pub fn update(&mut self) {
        self.baseRefreshHelperCharArray = { let java_value = (jcall!(GameStatisticsFactory::getInstance()).to2DCharArray()).clone(); java_value };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).setColor(self.RED.clone());
        let mut charArray: Vec<char> = Vec::new();
        let mut charArray2: Vec<char> = Vec::new();
        let mut size2: i32 = 0;
        let mut size3: i32 = 0;
        let mut size: i32 = (self.baseRefreshHelperCharArray.len() as i32) - 2;
        for index in (0..=size).rev().step_by((2) as usize) {
            charArray = { let java_value = (self.baseRefreshHelperCharArray[(index) as usize]).clone(); java_value }.clone();
            charArray2 = { let java_value = (self.baseRefreshHelperCharArray[(index + 1) as usize]).clone(); java_value }.clone();
            size2 = { let java_value = ((charArray.len() as i32)).clone(); java_value }.clone();
            size3 = { let java_value = ((charArray2.len() as i32)).clone(); java_value }.clone();
            jcall!(graphics).drawChars(charArray.clone(), 0, size2.clone(), 0, self.yArray[(index) as usize].clone(), 0);
            jcall!(graphics).drawChars(charArray2.clone(), 0, size3.clone(), size2 * self.defaultStringWidth, self.yArray[(index + 1) as usize].clone(), 0);
        }
    }
}
impl std::ops::Deref for GamePerformanceInitUpdatePaintable {
    type Target = InitUpdatePaintable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for GamePerformanceInitUpdatePaintable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintableDyn for GamePerformanceInitUpdatePaintable {
    fn as_InitUpdatePaintable(&self) -> &crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &self.base }
    fn as_InitUpdatePaintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::InitUpdatePaintable::InitUpdatePaintable { &mut self.base }
    fn init(&mut self) { Self::init(self) }
    fn update(&mut self) { Self::update(self) }
}
impl crate::org::allbinary::graphics::paint::Paintable::PaintableDyn for GamePerformanceInitUpdatePaintable {
    fn as_Paintable(&self) -> &crate::org::allbinary::graphics::paint::Paintable::Paintable { &self.base.base }
    fn as_Paintable_mut(&mut self) -> &mut crate::org::allbinary::graphics::paint::Paintable::Paintable { &mut self.base.base }
}
impl crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface for GamePerformanceInitUpdatePaintable {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { crate::org::allbinary::graphics::paint::Paintable::Paintable::paintThreed(crate::org::allbinary::graphics::paint::Paintable::PaintableDyn::as_Paintable_mut(self), p1) }
}


