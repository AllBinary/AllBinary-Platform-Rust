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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::media::image::ImageScaleUtil::ImageScaleUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ScaleBaseProcessor::ScaleBaseProcessor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ScaleProcessor {
    pub base: ScaleBaseProcessor,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageScaleUtil: Option<Rc<RefCell<ImageScaleUtil>>>,
}

thread_local! { pub static ScaleProcessor_instanceC: Option<Rc<RefCell<ScaleProcessor>>> = ScaleProcessor::new(); }

impl ScaleProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ScaleBaseProcessor::new_struct(),
            logUtil: None,
            commonStrings: None,
            imageScaleUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageScaleUtil = ImageScaleUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::ScaleBaseProcessor::ScaleBaseProcessorDyn>>> {
        return ScaleProcessor_instanceC.with(Clone::clone).clone();
    }
    

    pub fn update(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut ximageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut unused: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(self.imageScaleUtil).scale2(originalImage.clone(), originalImageArray.clone(), ximageToShowArray.clone(), unused.clone(), scaleX.clone(), scaleY.clone(), maxScaleX.clone(), maxScaleY.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).UPDATE.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
}
impl std::ops::Deref for ScaleProcessor {
    type Target = ScaleBaseProcessor;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ScaleProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ScaleBaseProcessor::ScaleBaseProcessorDyn for ScaleProcessor {
    fn as_ScaleBaseProcessor(&self) -> &crate::org::allbinary::animation::image::ScaleBaseProcessor::ScaleBaseProcessor { &self.base }
    fn as_ScaleBaseProcessor_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ScaleBaseProcessor::ScaleBaseProcessor { &mut self.base }
    fn update(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut ximageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut unused: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) { Self::update(self, originalImage, originalImageArray, ximageToShowArray, unused, scaleX, scaleY, maxScaleX, maxScaleY) }
}


