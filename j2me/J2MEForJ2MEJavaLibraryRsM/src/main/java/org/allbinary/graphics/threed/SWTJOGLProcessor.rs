// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.graphics.threed
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::midlet::MIDlet::MIDlet;
use super::RatioProcessor::RatioProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//OpenGLNone
pub struct SWTJOGLProcessor {
}

thread_local! { pub static SWTJOGLProcessor_instance: Option<Rc<RefCell<SWTJOGLProcessor>>> = SWTJOGLProcessor::new(); }
impl Runnable for SWTJOGLProcessor {
    fn run(&mut self) { Self::run(self) }
}

impl SWTJOGLProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<SWTJOGLProcessor>>> {
        return SWTJOGLProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn init(&mut self, mut display: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut comp: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn isHolderCreated(&mut self) -> bool {
        return true;
    }
    

    pub fn setRenderer(&mut self, mut renderer: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn isJOGL(&mut self) -> bool {
        return false;
    }
    

    pub fn createSurface(&mut self) {
    }
    

    pub fn onSurfaceCreated(&mut self) {
    }
    

    pub fn onSurfaceChangedForEvent(&mut self, mut event: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn onSurfaceChanged(&mut self) {
    }
    

    pub fn onSurfaceChangedWH(&mut self, mut width: i32, mut height: i32) {
    }
    

    pub fn addListener(&mut self) {
    }
    

    pub fn addListeners(&mut self, mut midlet: Option<Rc<RefCell<MIDlet>>>) {
    }
    

    pub fn run(&mut self) {
    }
    

    pub fn clear(&mut self) {
    }
    

    pub fn setCustom(&mut self, mut width: i32, mut height: i32, mut ratio: f32) {
    }
    

    pub fn setRatioProcessor(&mut self, mut ratioProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::threed::RatioProcessor::RatioProcessorDyn>>>) {
    }
    

    pub fn isPortraitRatioProcessor(&mut self) -> bool {
        return true;
    }
    

    pub fn charWidth(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut ch: char) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn charsWidth(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut ch: Vec<char>, mut offset: i32, mut length: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn stringWidth(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut str: String) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn substringWidth(&mut self, mut font: Option<Rc<RefCell<Font>>>, mut str: String, mut offset: i32, mut len: i32) -> i32 {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

