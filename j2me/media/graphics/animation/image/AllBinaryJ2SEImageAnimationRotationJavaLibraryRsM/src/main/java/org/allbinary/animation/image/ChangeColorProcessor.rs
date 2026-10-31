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
use super::ModifierBaseProcessor::ModifierBaseProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ChangeColorProcessor {
    pub base: ModifierBaseProcessor,
}

thread_local! { pub static ChangeColorProcessor_instanceC: Option<Rc<RefCell<ChangeColorProcessor>>> = ChangeColorProcessor::new(); }

impl ChangeColorProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ModifierBaseProcessor::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ChangeColorProcessor>>> {
        return ChangeColorProcessor_instanceC.with(Clone::clone).clone();
    }
    

    pub fn updateColor(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(imageModifierUtil).changeColor(originalImage.clone(), imageToShow.clone(), index.clone(), basicColor.clone());
    }
}
impl std::ops::Deref for ChangeColorProcessor {
    type Target = ModifierBaseProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ChangeColorProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessorDyn for ChangeColorProcessor {
    fn as_ModifierBaseProcessor(&self) -> &crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessor { &self.base }
    fn as_ModifierBaseProcessor_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ModifierBaseProcessor::ModifierBaseProcessor { &mut self.base }
    fn updateColor(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut index: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::updateColor(self, imageModifierUtil, originalImage, imageToShow, index, basicColor) }
}


