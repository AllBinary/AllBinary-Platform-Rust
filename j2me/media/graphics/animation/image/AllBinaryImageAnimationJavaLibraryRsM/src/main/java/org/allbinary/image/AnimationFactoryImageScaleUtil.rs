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
// package org.allbinary.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::image::AnimationFactoryInitializationVisitor::AnimationFactoryInitializationVisitor;
use crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::media::image::ImageJ2MEScaleUtil::ImageJ2MEScaleUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationFactoryImageScaleUtil {
    pub imageScaleUtil: Option<Rc<RefCell<ImageJ2MEScaleUtil>>>,
}

pub static AnimationFactoryImageScaleUtil_instance: crate::JavaStatic<Option<Rc<RefCell<AnimationFactoryImageScaleUtil>>>> = crate::JavaStatic::new(|| AnimationFactoryImageScaleUtil::new());

impl AnimationFactoryImageScaleUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageScaleUtil: None,
        };
        this.imageScaleUtil = ImageJ2MEScaleUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AnimationFactoryImageScaleUtil>>> {
        return AnimationFactoryImageScaleUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut width: i32, mut height: i32, mut scaleWidth: i32, mut scaleHeight: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut scaledImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if scaleWidth != 0 && scaleHeight != 0 {
            let mut scaleX: f32 = (((scaleWidth) as f32)) / (((width) as f32));
            let mut scaleY: f32 = (((scaleHeight) as f32)) / (((height) as f32));
            if (scaleX == 1.0_f32 && scaleY == 1.0_f32) || (scaleX == 0.0_f32 || scaleY == 0.0_f32) {
                scaledImage = image.clone();
            } else {
                let mut scaledWidth: i32 = (((scaleX * ((width) as f32))) as i32);
                let mut scaledHeight: i32 = (((scaleY * ((height) as f32))) as i32);
                scaledImage = { let java_value = (jcall!(self.imageScaleUtil).scale(image.clone(), scaledWidth.clone(), scaledHeight.clone())).clone(); java_value };
            }
        } else {
            scaledImage = image.clone();
        }
        return scaledImage.clone();
    }
    

    // throws Exception
    pub fn processAdjust(&mut self, mut baseImageAnimationFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>) {
        let mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>> = jcall!(baseImageAnimationFactory).getScalePropertiesP();
        if { let java_field = jcall!(scaleProperties).scaleWidth.clone(); java_field } != 0 && { let java_field = jcall!(scaleProperties).scaleHeight.clone(); java_field } != 0 {
            let mut animationFactoryInitializationVisitor: Option<Rc<RefCell<AnimationFactoryInitializationVisitor>>> = jcall!(baseImageAnimationFactory).getAnimationFactoryInitializationVisitorP();
            let mut scaleX: f32 = ((({ let java_field = jcall!(scaleProperties).scaleWidth.clone(); java_field }) as f32)) / ((({ let java_field = jcall!(animationFactoryInitializationVisitor).width.clone(); java_field }) as f32));
            let mut scaleY: f32 = ((({ let java_field = jcall!(scaleProperties).scaleHeight.clone(); java_field }) as f32)) / ((({ let java_field = jcall!(animationFactoryInitializationVisitor).height.clone(); java_field }) as f32));
            if (scaleX == 1.0_f32 && scaleY == 1.0_f32) || (scaleX == 0.0_f32 || scaleY == 0.0_f32) {
            } else {
                jcall!(animationFactoryInitializationVisitor).dx = ((((({ let java_field = jcall!(animationFactoryInitializationVisitor).originalDx.clone(); java_field }) as f32) * scaleX)) as i32);
                jcall!(animationFactoryInitializationVisitor).dy = ((((({ let java_field = jcall!(animationFactoryInitializationVisitor).originalDy.clone(); java_field }) as f32) * scaleY)) as i32);
            }
        } else {
        }
    }
}

