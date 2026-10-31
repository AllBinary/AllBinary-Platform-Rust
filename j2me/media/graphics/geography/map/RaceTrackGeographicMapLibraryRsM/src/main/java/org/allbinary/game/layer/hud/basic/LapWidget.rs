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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud;
use crate::org::allbinary::game::layer::hud::LapInfo::LapInfo;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LapWidget {
    pub base: BasicHud,
    pub LAP_STR: String,
    pub OF_STR: String,
    pub string: String,
}


impl LapWidget {
    

    pub fn new_struct(mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, basicColor.clone()),
            LAP_STR: String::new(),
            OF_STR: String::new(),
            string: String::new(),
        };
        this.LAP_STR = String::from("Lap ");
        this.OF_STR = String::from(" of ");
        this.string = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        // super(...) is in the struct literal above
        this.updateMaxHeight = 15;
        this.updateMaxWidth = 40;
        this
    }

    pub fn new(mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(location, direction, basicColor))))
    }
    

    pub fn update(&mut self, mut lapInfo: Option<Rc<RefCell<LapInfo>>>) {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(self.LAP_STR.clone());
        jcall!(stringBuffer).appendint(jcall!(lapInfo).getCurrentLap());
        jcall!(stringBuffer).append(self.OF_STR.clone());
        jcall!(stringBuffer).appendint(jcall!(lapInfo).getTotalLaps());
        self.string = jcall!(stringBuffer).toString();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paintString(graphics.clone(), self.string.clone());
    }
}
impl std::ops::Deref for LapWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LapWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for LapWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
}


