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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ModifierBaseProcessor {
}

thread_local! { pub static ModifierBaseProcessor_instance: Option<Rc<RefCell<dyn ModifierBaseProcessorDyn>>> = ModifierBaseProcessor::new().map(|reference| reference as Rc<RefCell<dyn ModifierBaseProcessorDyn>>); }

impl ModifierBaseProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ModifierBaseProcessorDyn>>> {
        return ModifierBaseProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn updateColor(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn updateAlpha(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut alpha: i32) {
    }
}
pub trait ModifierBaseProcessorDyn {
    fn as_ModifierBaseProcessor(&self) -> &ModifierBaseProcessor;
    fn as_ModifierBaseProcessor_mut(&mut self) -> &mut ModifierBaseProcessor;
    fn updateColor(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { ModifierBaseProcessor::updateColor(self.as_ModifierBaseProcessor_mut(), imageModifierUtil, originalImage, imageToShow, index, basicColor) }
    fn updateAlpha(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut alpha: i32) { ModifierBaseProcessor::updateAlpha(self.as_ModifierBaseProcessor_mut(), imageModifierUtil, originalImage, imageToShow, index, alpha) }
}
impl ModifierBaseProcessorDyn for ModifierBaseProcessor {
    fn as_ModifierBaseProcessor(&self) -> &ModifierBaseProcessor { self }
    fn as_ModifierBaseProcessor_mut(&mut self) -> &mut ModifierBaseProcessor { self }
}


