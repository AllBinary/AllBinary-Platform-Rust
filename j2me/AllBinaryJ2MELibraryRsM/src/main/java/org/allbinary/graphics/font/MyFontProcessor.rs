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
// use crate::javax::microedition::lcdui::Font::Font;
// use crate::javax::microedition::lcdui::Graphics::Graphics;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MyFontProcessor {
}

thread_local! { pub static MyFontProcessor_instance: Option<Rc<RefCell<dyn MyFontProcessorDyn>>> = MyFontProcessor::new().map(|reference| reference as Rc<RefCell<dyn MyFontProcessorDyn>>); }

impl MyFontProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn defaultCharWidth(mut font: Option<Rc<RefCell<Font>>>) -> i32 {
        return jcall!(font).charWidth('C');
    }
    

    pub fn defaultStringWidth(mut font: Option<Rc<RefCell<Font>>>, mut size: i32) -> i32 {
        return MyFontProcessor::defaultCharWidth(font.clone()) * size;
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn MyFontProcessorDyn>>> {
        return MyFontProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn process(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
    }
}
pub trait MyFontProcessorDyn {
    fn as_MyFontProcessor(&self) -> &MyFontProcessor;
    fn as_MyFontProcessor_mut(&mut self) -> &mut MyFontProcessor;
    fn process(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { MyFontProcessor::process(self.as_MyFontProcessor_mut(), graphics) }
}
impl MyFontProcessorDyn for MyFontProcessor {
    fn as_MyFontProcessor(&self) -> &MyFontProcessor { self }
    fn as_MyFontProcessor_mut(&mut self) -> &mut MyFontProcessor { self }
}


