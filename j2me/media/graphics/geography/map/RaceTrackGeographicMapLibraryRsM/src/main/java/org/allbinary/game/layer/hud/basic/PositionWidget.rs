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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//super.paint(graphics, positionString, currentPostString, offset2, offset);
pub struct PositionWidget {
    pub base: BasicHud,
    pub position: i32,
    pub positionString: String,
}


impl PositionWidget {
    

    pub fn new_struct(mut position: i32, mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: BasicHud::new_struct(location.clone(), direction.clone(), 2, basicColor.clone()),
            position: 0,
            positionString: String::new(),
        };
        this.positionString = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        // super(...) is in the struct literal above
        this.updateMaxHeight = 10;
        this.updateMaxWidth = 40;
        this
    }

    pub fn new(mut position: i32, mut location: i32, mut direction: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(position, location, direction, basicColor))))
    }
    

    pub fn get(&mut self) -> i32 {
        return self.position.clone();
    }
    

    pub fn set(&mut self, mut position: i32) {
        self.position = position.clone();
        self.positionString = PositionWidget::toString(self.get());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        self.base.paintString(graphics.clone(), self.positionString.clone());
    }
    

    pub fn toString(mut position: i32) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append((position.clone()).to_string());
        if position == 1 {
            jcall!(stringBuffer).append(String::from("st"));
        } else if position == 2 {
            jcall!(stringBuffer).append(String::from("nd"));
        } else if position == 3 {
            jcall!(stringBuffer).append(String::from("rd"));
        } else {
            jcall!(stringBuffer).append(String::from("th"));
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for PositionWidget {
    type Target = BasicHud;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PositionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::game::graphics::hud::BasicHud::BasicHudDyn for PositionWidget {
    fn as_BasicHud(&self) -> &crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &self.base }
    fn as_BasicHud_mut(&mut self) -> &mut crate::org::allbinary::game::graphics::hud::BasicHud::BasicHud { &mut self.base }
}


