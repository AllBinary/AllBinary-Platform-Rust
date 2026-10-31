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
// package org.allbinary.game.ag.layer.hud.basic.score
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScoreHudWidget {
    pub base: NumberStringHud,
}

impl crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface for ScoreHudWidget {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl ScoreHudWidget {
    

    // throws Exception
    pub fn getInstance(mut maxscore: i32, mut location: i32, mut direction: i32) -> Option<Rc<RefCell<ScoreHudWidget>>> {
        return ScoreHudWidget::new(maxscore.clone(), location.clone(), direction.clone(), 2, jcall!(BasicColorFactory::getInstance()).GREY.clone());
    }
    

    pub fn new_struct(mut maxscore: i32, mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: NumberStringHud::new_struct(String::from("Pts "), maxscore.clone(), location.clone(), direction.clone(), bufferZone.clone(), basicColor.clone()),
        };
        // super(...) is in the struct literal above
        this.updateMaxHeight = 14;
        this
    }

    pub fn new(mut maxscore: i32, mut location: i32, mut direction: i32, mut bufferZone: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(maxscore, location, direction, bufferZone, basicColor))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.updateMaxWidth = jcall!(font).getSize() * 5;
        self.base.updateMeasurement(graphics.clone());
    }
}
impl std::ops::Deref for ScoreHudWidget {
    type Target = NumberStringHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScoreHudWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHudDyn for ScoreHudWidget {
    fn as_NumberStringHud(&self) -> &crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &self.base }
    fn as_NumberStringHud_mut(&mut self) -> &mut crate::org::allbinary::game::ag::layer::hud::basic::NumberStringHud::NumberStringHud { &mut self.base }
}
impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for ScoreHudWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base.base }
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}


