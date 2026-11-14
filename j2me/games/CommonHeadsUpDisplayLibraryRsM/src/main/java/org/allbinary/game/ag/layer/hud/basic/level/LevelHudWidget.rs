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
// package org.allbinary.game.ag.layer.hud.basic.level
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Font::Font;
use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::paint::PaintableInterface::PaintableInterface;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::PrimitiveLongUtil::PrimitiveLongUtil;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LevelHudWidget {
    pub base: BasicHud,
    pub level: i32,
    pub maxlevel: i32,
    pub levelString: Vec<char>,
    pub levelNumberCharArray: Vec<char>,
    pub levelNumberTotalDigits: i32,
    pub offset: i32,
    pub primitiveLongUtil: Option<Rc<RefCell<PrimitiveLongUtil>>>,
}

impl PaintableInterface for LevelHudWidget {
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paint(self, graphics) }
    fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::paintThreed(self, graphics) }
}

impl LevelHudWidget {
    

    // throws Exception
    pub fn createHud(mut maxlevel: i32, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<LevelHudWidget>>> {
        return LevelHudWidget::new(maxlevel.clone(), location.clone(), direction.clone());
    }
    

    pub fn new_struct(mut maxlevel: i32, mut location: i32, mut direction: i32) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, { let java_field = jcall!(BasicColorFactory::getInstance()).GREY.clone(); java_field }.clone()),
            level: 0,
            maxlevel: 0,
            levelString: Vec::new(),
            levelNumberCharArray: Vec::new(),
            levelNumberTotalDigits: 0,
            offset: 0,
            primitiveLongUtil: None,
        };
        this.levelNumberCharArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_CHAR_ARRAY.clone(); java_field }.clone();
        // super(...) is in the struct literal above
        this.primitiveLongUtil = { let java_value = (PrimitiveLongUtil::createPowerOfTen(1000)).clone(); java_value };
        let mut LEVEL: String = String::from("Lv ");
        this.levelString = { let java_value = (LEVEL.chars().collect::<Vec<char>>()).clone(); java_value };
        this.maxlevel = maxlevel.clone();
        this.level = maxlevel.clone();
        this.update();
        this.updateMaxHeight = 14;
        this
    }

    pub fn new(mut maxlevel: i32, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxlevel, location, direction))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        let mut font: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Font::FontDyn>>> = jcall!(graphics).getFont();
        self.updateMaxWidth = { let java_value = (jcall!(font).getSize() * 4).clone(); java_value };
        self.offset = { let java_value = (jcall!(font).charsWidth(self.levelString.clone(), 0, (self.levelString.len() as i32).clone()) + jcall!(font).getSize()).clone(); java_value };
        { let java_arguments = (graphics.clone(),); self.base.updateMeasurement(java_arguments.0) };
    }
    

    pub fn update(&mut self) {
        self.levelNumberCharArray = { let java_value = (jcall!(self.primitiveLongUtil).getCharArray(self.level.clone())).clone(); java_value };
        self.levelNumberTotalDigits = { let java_value = (jcall!(self.primitiveLongUtil).getCurrentTotalDigits()).clone(); java_value };
    }
    

    pub fn setLevel(&mut self, mut level: i32) {
        self.level = level.clone();
        self.update();
    }
    

    pub fn nextLevel(&mut self) {
        self.level += 1;
        self.update();
    }
    

    pub fn previousLevel(&mut self) {
        self.level -= 1;
        if self.level < 0 {
            self.level = 0;
        }
        self.update();
    }
    

    pub fn isComplete(&mut self) -> bool {
        if self.level <= self.maxlevel {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
        { let java_arguments = (graphics.clone(), self.levelString.clone(), 0, (self.levelString.len() as i32).clone(), self.levelNumberCharArray.clone(), 0, self.levelNumberTotalDigits.clone(), self.offset.clone(),); self.base.paintDX(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5, java_arguments.6, java_arguments.7) };
    }
    

    pub fn paintThreed(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) {
    }
}
impl std::ops::Deref for LevelHudWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LevelHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for LevelHudWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
}
impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for LevelHudWidget {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>) { Self::updateMeasurement(self, graphics) }
}


