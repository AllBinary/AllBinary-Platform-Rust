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
// package org.allbinary.game.ag.layer.hud.basic
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::game::graphics::hud::BasicHudFactory::BasicHudFactory;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::logic::math::PrimitiveLongSingleton::PrimitiveLongSingleton;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::java::lang::RuntimeException::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NumberStringHud {
    pub base: BasicHud,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
    pub prependString: String,
    pub PREPEND_STRING: Vec<char>,
    pub value: i32,
    pub max: i32,
    pub offset: i32,
    pub valueString: Vec<char>,
    pub valueTotalDigits: i32,
}

impl PaintableInterface for NumberStringHud {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}

impl NumberStringHud {
    

    pub fn new_struct(mut prependString: String, mut max: i32, mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()),
            primitiveLongUtil: None,
            prependString: String::new(),
            PREPEND_STRING: Vec::new(),
            value: 0,
            max: 0,
            offset: 0,
            valueString: Vec::new(),
            valueTotalDigits: 0,
        };
        // super(...) is in the struct literal above
        this.prependString = prependString.clone();
        this.PREPEND_STRING = { let java_value = (prependString.chars().collect::<Vec<char>>()).clone(); java_value };
        this.valueString = { let java_value = (jcall!(PrimitiveLongSingleton::getInstance()).NUMBER_CHAR_ARRAYS[(0) as usize]).clone(); java_value }.clone();
        this.primitiveLongUtil = { let java_value = (PrimitiveLongUtil::createPowerOfTen(max + 1)).clone(); java_value };
        this.max = max.clone();
        this.value = 0;
        if direction == 0 {
            panic!("{:?}", format!("{}: {:?}", "RuntimeException", ({ let java_field = jcall!(BasicHudFactory::getInstance()).DIRECTION_EXCEPTION.clone(); java_field }.clone())));
        }
        this
    }

    pub fn new(mut prependString: String, mut max: i32, mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(prependString, max, location, direction, bufferZone, basicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.offset = { let java_value = (jcall!(font).stringWidth(self.prependString.clone()) + MyFontProcessor::defaultCharWidth(font.clone())).clone(); java_value };
        { let java_arguments = (graphics.clone(),); self.base.updateMeasurement(java_arguments.0) };
    }
    

    pub fn get(&mut self) -> i32 {
        return self.value.clone();
    }
    

    pub fn add(&mut self, mut value: i32) {
        { let java_arguments = (self.value + value,); self.set(java_arguments.0) };
    }
    

    pub fn set(&mut self, mut value: i32) {
        self.value = value.clone();
        if self.value > self.max {
            self.value = 0;
        }
        self.valueString = { let java_value = (jcall!(self.primitiveLongUtil).getCharArray(self.value.clone())).clone(); java_value };
        self.valueTotalDigits = { let java_value = (jcall!(self.primitiveLongUtil).getCurrentTotalDigits()).clone(); java_value };
    }
    

    pub fn reduce(&mut self, mut value: i32) {
        { let java_arguments = (self.value - value,); self.set(java_arguments.0) };
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(), self.PREPEND_STRING.clone(), 0, (self.PREPEND_STRING.len() as i32).clone(), self.valueString.clone(), 0, self.valueTotalDigits.clone(), self.offset.clone(),); self.base.paintDX(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6, java_arguments.7) };
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        let mut charArray: Vec<char> = self.PREPEND_STRING.clone();
        let mut len: i32 = (self.PREPEND_STRING.len() as i32).clone();
        let mut charArray2: Vec<char> = self.valueString.clone();
        let mut len2: i32 = self.valueTotalDigits.clone();
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(),); jcall!(java_receiver).setBasicColorP(java_arguments.0, java_arguments.1); };
        jcall!(graphics).drawChars(charArray.clone(), 0, len.clone(), x.clone(), y.clone(), 0);
        jcall!(graphics).drawChars(charArray2.clone(), 0, len2.clone(), x - self.offset, y.clone(), 0);
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
}
impl std::ops::Deref for NumberStringHud {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NumberStringHud {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait NumberStringHudDyn: crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn + crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface {
    fn as_NumberStringHud(&self) -> &NumberStringHud;
    fn as_NumberStringHud_mut(&mut self) -> &mut NumberStringHud;
    fn get(&mut self) -> i32 { NumberStringHud::get(self.as_NumberStringHud_mut()) }
    fn add(&mut self, mut value: i32) { NumberStringHud::add(self.as_NumberStringHud_mut(), value) }
    fn set(&mut self, mut value: i32) { NumberStringHud::set(self.as_NumberStringHud_mut(), value) }
    fn reduce(&mut self, mut value: i32) { NumberStringHud::reduce(self.as_NumberStringHud_mut(), value) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { NumberStringHud::paintXY(self.as_NumberStringHud_mut(), graphics, x, y) }
}
impl NumberStringHudDyn for NumberStringHud {
    fn as_NumberStringHud(&self) -> &NumberStringHud { self }
    fn as_NumberStringHud_mut(&mut self) -> &mut NumberStringHud { self }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for NumberStringHud {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for NumberStringHud {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}


