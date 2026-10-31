// Generated Code Do Not Modify
// package org.allbinary.graphics.font
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Font::Font;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//OpenGLESBaseJ2MEJavaLibrary
pub struct FontDebugBase {
    pub SET_FONT: String,
}

thread_local! { pub static FontDebugBase_instanceBase: Option<Rc<RefCell<dyn FontDebugBaseDyn>>> = FontDebugBase::new().map(|reference| reference as Rc<RefCell<dyn FontDebugBaseDyn>>); }

impl FontDebugBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SET_FONT: String::new(),
        };
        this.SET_FONT = String::from("setFont");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn FontDebugBaseDyn>>> {
        return FontDebugBase_instanceBase.with(Clone::clone).clone();
    }
    

    pub fn debugMetrics(&mut self, mut currentFont: Option<Rc<RefCell<Font>>>, mut convertedTextSize: f32, mut displayMetrics: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
    }
    

    pub fn debugDpi(&mut self, mut densityDpi: i32, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
    }
    

    pub fn debugDimension(&mut self, mut currentFont: Option<Rc<RefCell<Font>>>, mut convertedTextSize: f32, mut longestDimensionTotalPixels: i32, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) {
    }
}
pub trait FontDebugBaseDyn {
    fn as_FontDebugBase(&self) -> &FontDebugBase;
    fn as_FontDebugBase_mut(&mut self) -> &mut FontDebugBase;
    fn debugMetrics(&mut self, mut currentFont: Option<Rc<RefCell<Font>>>, mut convertedTextSize: f32, mut displayMetrics: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { FontDebugBase::debugMetrics(self.as_FontDebugBase_mut(), currentFont, convertedTextSize, displayMetrics, stringBuffer) }
    fn debugDpi(&mut self, mut densityDpi: i32, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { FontDebugBase::debugDpi(self.as_FontDebugBase_mut(), densityDpi, stringBuffer) }
    fn debugDimension(&mut self, mut currentFont: Option<Rc<RefCell<Font>>>, mut convertedTextSize: f32, mut longestDimensionTotalPixels: i32, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>) { FontDebugBase::debugDimension(self.as_FontDebugBase_mut(), currentFont, convertedTextSize, longestDimensionTotalPixels, stringBuffer) }
}
impl FontDebugBaseDyn for FontDebugBase {
    fn as_FontDebugBase(&self) -> &FontDebugBase { self }
    fn as_FontDebugBase_mut(&mut self) -> &mut FontDebugBase { self }
}


