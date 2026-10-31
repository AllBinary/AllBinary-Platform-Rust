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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL10::GL10;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESStrings {
}


impl OpenGLESStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn initFont(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut width: i32, mut height: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut width: i32, mut height: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn drawChar(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut character: char, mut x: i32, mut y: i32, mut anchor: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn drawChars(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut data: Vec<char>, mut offset: i32, mut length: i32, mut x: i32, mut y: i32, mut anchor: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn drawString(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut x: i32, mut y: i32, mut anchor: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn drawSubstring(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut offset: i32, mut len: i32, mut x: i32, mut y: i32, mut anchor: i32) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn charWidth(&mut self, mut ch: char) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn charsWidth(&mut self, mut ch: Vec<char>, mut offset: i32, mut length: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn stringWidth(&mut self, mut string: String) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn stringWidth3(&mut self, mut string: String, mut offset: i32, mut len: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait OpenGLESStringsDyn {
    fn as_OpenGLESStrings(&self) -> &OpenGLESStrings;
    fn as_OpenGLESStrings_mut(&mut self) -> &mut OpenGLESStrings;
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>) { OpenGLESStrings::set(self.as_OpenGLESStrings_mut(), gl) }
    fn initFont(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut width: i32, mut height: i32) { OpenGLESStrings::initFont(self.as_OpenGLESStrings_mut(), gl, width, height) }
    fn update(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut width: i32, mut height: i32) { OpenGLESStrings::update(self.as_OpenGLESStrings_mut(), gl, width, height) }
    fn drawChar(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut character: char, mut x: i32, mut y: i32, mut anchor: i32) { OpenGLESStrings::drawChar(self.as_OpenGLESStrings_mut(), gl, graphics, character, x, y, anchor) }
    fn drawChars(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut data: Vec<char>, mut offset: i32, mut length: i32, mut x: i32, mut y: i32, mut anchor: i32) { OpenGLESStrings::drawChars(self.as_OpenGLESStrings_mut(), gl, graphics, data, offset, length, x, y, anchor) }
    fn drawString(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut x: i32, mut y: i32, mut anchor: i32) { OpenGLESStrings::drawString(self.as_OpenGLESStrings_mut(), gl, graphics, string, x, y, anchor) }
    fn drawSubstring(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut offset: i32, mut len: i32, mut x: i32, mut y: i32, mut anchor: i32) { OpenGLESStrings::drawSubstring(self.as_OpenGLESStrings_mut(), gl, graphics, string, offset, len, x, y, anchor) }
    fn charWidth(&mut self, mut ch: char) -> i32 { OpenGLESStrings::charWidth(self.as_OpenGLESStrings_mut(), ch) }
    fn charsWidth(&mut self, mut ch: Vec<char>, mut offset: i32, mut length: i32) -> i32 { OpenGLESStrings::charsWidth(self.as_OpenGLESStrings_mut(), ch, offset, length) }
    fn stringWidth(&mut self, mut string: String) -> i32 { OpenGLESStrings::stringWidth(self.as_OpenGLESStrings_mut(), string) }
    fn stringWidth3(&mut self, mut string: String, mut offset: i32, mut len: i32) -> i32 { OpenGLESStrings::stringWidth3(self.as_OpenGLESStrings_mut(), string, offset, len) }
}
impl OpenGLESStringsDyn for OpenGLESStrings {
    fn as_OpenGLESStrings(&self) -> &OpenGLESStrings { self }
    fn as_OpenGLESStrings_mut(&mut self) -> &mut OpenGLESStrings { self }
}


