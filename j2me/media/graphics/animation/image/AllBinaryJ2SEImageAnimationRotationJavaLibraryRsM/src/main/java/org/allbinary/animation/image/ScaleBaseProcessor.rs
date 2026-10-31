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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScaleBaseProcessor {
}

thread_local! { pub static ScaleBaseProcessor_instance: Option<Rc<RefCell<dyn ScaleBaseProcessorDyn>>> = ScaleBaseProcessor::new().map(|reference| reference as Rc<RefCell<dyn ScaleBaseProcessorDyn>>); }

impl ScaleBaseProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ScaleBaseProcessorDyn>>> {
        return ScaleBaseProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn update(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut imageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) {
    }
}
pub trait ScaleBaseProcessorDyn {
    fn as_ScaleBaseProcessor(&self) -> &ScaleBaseProcessor;
    fn as_ScaleBaseProcessor_mut(&mut self) -> &mut ScaleBaseProcessor;
    fn update(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut imageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut index: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) { ScaleBaseProcessor::update(self.as_ScaleBaseProcessor_mut(), originalImage, originalImageArray, imageToShowArray, index, scaleX, scaleY, maxScaleX, maxScaleY) }
}
impl ScaleBaseProcessorDyn for ScaleBaseProcessor {
    fn as_ScaleBaseProcessor(&self) -> &ScaleBaseProcessor { self }
    fn as_ScaleBaseProcessor_mut(&mut self) -> &mut ScaleBaseProcessor { self }
}


