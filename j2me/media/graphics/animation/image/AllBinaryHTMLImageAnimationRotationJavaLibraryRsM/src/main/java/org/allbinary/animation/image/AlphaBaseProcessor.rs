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
use crate::org::allbinary::media::image::ImageModifierUtil::ImageModifierUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AlphaBaseProcessor {
}

thread_local! { pub static AlphaBaseProcessor_instance: Option<Rc<RefCell<dyn AlphaBaseProcessorDyn>>> = AlphaBaseProcessor::new().map(|reference| reference as Rc<RefCell<dyn AlphaBaseProcessorDyn>>); }

impl AlphaBaseProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn AlphaBaseProcessorDyn>>> {
        return AlphaBaseProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn setAlpha(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut alpha: i32) {
    }
}
pub trait AlphaBaseProcessorDyn {
    fn as_AlphaBaseProcessor(&self) -> &AlphaBaseProcessor;
    fn as_AlphaBaseProcessor_mut(&mut self) -> &mut AlphaBaseProcessor;
    fn setAlpha(&mut self, mut imageModifierUtil: Option<Rc<RefCell<ImageModifierUtil>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut imageToShow: Option<Rc<RefCell<Image>>>, mut alpha: i32) { AlphaBaseProcessor::setAlpha(self.as_AlphaBaseProcessor_mut(), imageModifierUtil, originalImage, imageToShow, alpha) }
}
impl AlphaBaseProcessorDyn for AlphaBaseProcessor {
    fn as_AlphaBaseProcessor(&self) -> &AlphaBaseProcessor { self }
    fn as_AlphaBaseProcessor_mut(&mut self) -> &mut AlphaBaseProcessor { self }
}


