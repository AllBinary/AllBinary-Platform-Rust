// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.graphics.font
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use super::MyFontProcessor::MyFontProcessor;
use super::UpdateMyFontInterface::UpdateMyFontInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpdateMyFontProcessor {
    pub base: MyFontProcessor,
    pub updateMeasurementProcessor: Option<Rc<RefCell<dyn UpdateMyFontInterface>>>,
}


impl UpdateMyFontProcessor {
    

    pub fn new_struct(mut keyValueDrawCharArray: Option<Rc<RefCell<dyn UpdateMyFontInterface>>>) -> Self {
        let mut this = Self {
            base: MyFontProcessor::new_struct(),
            updateMeasurementProcessor: None,
        };
        this.updateMeasurementProcessor = keyValueDrawCharArray.clone();
        this
    }

    pub fn new(mut keyValueDrawCharArray: Option<Rc<RefCell<dyn UpdateMyFontInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(keyValueDrawCharArray))))
    }
    

    pub fn process(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.updateMeasurementProcessor).updateMeasurement(graphics.clone());
    }
}
impl std::ops::Deref for UpdateMyFontProcessor {
    type Target = MyFontProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpdateMyFontProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessorDyn for UpdateMyFontProcessor {
    fn as_MyFontProcessor(&self) -> &crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor { &self.base }
    fn as_MyFontProcessor_mut(&mut self) -> &mut crate::org::allbinary::graphics::font::MyFontProcessor::MyFontProcessor { &mut self.base }
    fn process(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { Self::process(self, graphics) }
}


