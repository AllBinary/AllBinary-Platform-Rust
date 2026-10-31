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
// package org.allbinary.image.sprite
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
use crate::org::allbinary::game::layer::SpriteFactory::SpriteFactory;
use crate::org::allbinary::graphics::opengles::OpenGLUtil::OpenGLUtil;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::image::opengles::OpenGLESImageExclusionUtil::OpenGLESImageExclusionUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::math::J2SEMath::J2SEMath;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::media::image::ImageScaleUtil::ImageScaleUtil;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationFactorySpriteScaleUtil {
    pub imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>,
    pub imageScaleUtil: Option<Rc<RefCell<ImageScaleUtil>>>,
    pub openGLUtil: Option<Rc<RefCell<OpenGLUtil>>>,
    pub j2seMath: Option<Rc<RefCell<J2SEMath>>>,
    pub openGLESImageExclusionUtil: Option<Rc<RefCell<OpenGLESImageExclusionUtil>>>,
}

thread_local! { pub static AnimationFactorySpriteScaleUtil_instance: Option<Rc<RefCell<AnimationFactorySpriteScaleUtil>>> = AnimationFactorySpriteScaleUtil::new(); }

impl AnimationFactorySpriteScaleUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageCache: None,
            imageScaleUtil: None,
            openGLUtil: None,
            j2seMath: None,
            openGLESImageExclusionUtil: None,
        };
        this.imageCache = ImageCacheFactory::getInstance();
        this.imageScaleUtil = ImageScaleUtil::getInstance();
        this.openGLUtil = OpenGLUtil::getInstance();
        this.j2seMath = J2SEMath::getInstance();
        this.openGLESImageExclusionUtil = OpenGLESImageExclusionUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AnimationFactorySpriteScaleUtil>>> {
        return AnimationFactorySpriteScaleUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut scaleWidth: i32, mut scaleHeight: i32) -> Option<Rc<RefCell<Sprite>>> {
        let mut spriteFactory: Option<Rc<RefCell<SpriteFactory>>> = SpriteFactory::getInstance();
        let mut sprite: Option<Rc<RefCell<Sprite>>> = None;
        let mut scaledImage: Option<Rc<RefCell<Image>>> = None;
        if scaleWidth != 0 && scaleHeight != 0 {
            let mut scaleX: f32 = (((scaleWidth) as f32)) / (((width) as f32));
            let mut scaleY: f32 = (((scaleHeight) as f32)) / (((height) as f32));
            if (scaleX == 1.0_f32 && scaleY == 1.0_f32) || (scaleX == 0.0_f32 || scaleY == 0.0_f32) {
                scaledImage = jcall!(self.openGLUtil).addImage(image.clone());
                sprite = jcall!(spriteFactory).createSprite(scaledImage.clone(), width.clone(), height.clone());
            } else {
                if jcall!(self.openGLESImageExclusionUtil).isCustomScaling(image.clone()) {
                    let mut width2: i32 = jcall!(self.j2seMath).round((((scaleWidth)) as f32) - 0.5_f32);
                    let mut height2: i32 = jcall!(self.j2seMath).round((((scaleHeight)) as f32) - 0.5_f32);
                    let mut multiplesOf16Width: i32 = width2 / 16;
                    let mut by16Width: i32 = multiplesOf16Width * 16;
                    scaleX = (((by16Width) as f32)) / ((width) as f32);
                    let mut multiplesOf16Height: i32 = height2 / 16;
                    let mut by16Height: i32 = multiplesOf16Height * 16;
                    scaleY = (((by16Height) as f32)) / ((height) as f32);
                    if scaleWidth < width {
                        scaleX = scaleX * 2.35_f32;
                    }
                    if scaleHeight < height {
                        scaleY = scaleY * 2.35_f32;
                    }
                }
                scaledImage = jcall!(self.imageScaleUtil).createImage2(self.imageCache.clone(), image.clone(), scaleX.clone(), 1.0_f32, scaleY.clone(), 1.0_f32, true);
                sprite = jcall!(spriteFactory).createSprite(scaledImage.clone(), (((((width) as f32) * scaleX)) as i32), (((((height) as f32) * scaleY)) as i32));
            }
        } else {
            scaledImage = jcall!(self.openGLUtil).addImage(image.clone());
            sprite = jcall!(spriteFactory).createSprite(scaledImage.clone(), width.clone(), height.clone());
        }
        return sprite.clone();
    }
}

