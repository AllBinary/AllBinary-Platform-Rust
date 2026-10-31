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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::logic::java::character::CharArrayFactory::CharArrayFactory;
use crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor;
use crate::org::allbinary::graphics::font::UpdateMyFontInterface::UpdateMyFontInterface;
use crate::org::allbinary::graphics::font::UpdateMyFontProcessor::UpdateMyFontProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct KeyValueDrawCharArray {
    pub myFontProcessor: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>>,
    pub LABEL: String,
    pub labelX: i32,
    pub valueX: i32,
    pub value: Vec<char>,
    pub len: i32,
}

impl UpdateMyFontInterface for KeyValueDrawCharArray {
    fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::updateMeasurement(self, graphics) }
}

impl KeyValueDrawCharArray {
    

    pub fn new_struct(mut label: String, mut x: i32) -> Self {
        let mut this = Self {
            myFontProcessor: None,
            LABEL: String::new(),
            labelX: 0,
            valueX: 0,
            value: Vec::new(),
            len: 0,
        };
        this.myFontProcessor = UpdateMyFontProcessor::new(this).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn>>);
        this.value = jcall!(CharArrayFactory::getInstance()).getZeroCharArray();
        this.LABEL = label.clone();
        this.labelX = x.clone();
        this
    }

    pub fn new(mut label: String, mut x: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(label, x))))
    }
    

    pub fn updateMeasurement(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        let mut labelWidth: i32 = jcall!(jcall!(graphics).getFont()).stringWidth(self.LABEL.clone()) - (self.LABEL.chars().count() as i32);
        self.valueX = labelWidth + self.labelX;
        self.myFontProcessor = MyFontProcessor::getInstance();
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut y: i32) {
        jcall!(self.myFontProcessor).process(graphics.clone());
        jcall!(graphics).drawString(self.LABEL.clone(), self.labelX.clone(), y.clone(), 0);
        jcall!(graphics).drawChars(self.value.clone(), 0, self.len.clone(), self.valueX.clone(), y.clone(), 0);
    }
    

    pub fn update(&mut self, mut value: Vec<char>, mut len: i32) {
        self.value = value.clone();
        self.len = len.clone();
    }
}

