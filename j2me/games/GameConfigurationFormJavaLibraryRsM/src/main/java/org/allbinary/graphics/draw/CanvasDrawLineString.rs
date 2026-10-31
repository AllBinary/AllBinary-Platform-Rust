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
// package org.allbinary.graphics.draw
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;
use super::DrawStringUtil::DrawStringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasDrawLineString {
    pub drawStringUtil: Option<Rc<RefCell<DrawStringUtil>>>,
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub x: i32,
    pub y: i32,
    pub fontHeight: i32,
}

thread_local! { pub static CanvasDrawLineString_NULL_CANVAS_DRAW_LINE_STRING: Option<Rc<RefCell<CanvasDrawLineString>>> = CanvasDrawLineString::new(0, 0); }
impl UpdateMyFontInterface for CanvasDrawLineString {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl CanvasDrawLineString {
    

    pub fn new_struct(mut x: i32, mut y: i32) -> Self {
        let mut this = Self {
            drawStringUtil: None,
            myFontProcessor: None,
            x: 0,
            y: 0,
            fontHeight: 0,
        };
        this.drawStringUtil = DrawStringUtil::getInstance();
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.fontHeight = 0;
        this.x = x.clone();
        this.y = y.clone();
        this
    }

    pub fn new(mut x: i32, mut y: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(x, y))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut font: Option<Rc<RefCell<Font>>> = jcall!(graphics).getFont();
        self.fontHeight = jcall!(font).getHeight();
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut string: String, mut line: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        { let java_receiver = self.drawStringUtil.clone(); let java_arguments = (graphics.clone(), string.clone(), 0, (string.chars().count() as i32), self.x.clone(), self.y + (line * self.fontHeight),); jcall!(java_receiver).drawCenterString(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4, java_arguments.5); };
    }
}

