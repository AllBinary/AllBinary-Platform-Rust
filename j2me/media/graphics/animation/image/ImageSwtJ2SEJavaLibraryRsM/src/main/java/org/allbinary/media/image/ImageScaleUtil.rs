//
// AllBinary Open License Version 1
// Copyright (c) 2011 AllBinary
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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ImageCopyUtil::ImageCopyUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageScaleUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageCopyUtil: Option<Rc<RefCell<ImageCopyUtil>>>,
    pub gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>>,
    pub features: Option<Rc<RefCell<Features>>>,
    pub NO_COPY: String,
}

thread_local! { pub static ImageScaleUtil_instance: Option<Rc<RefCell<ImageScaleUtil>>> = ImageScaleUtil::new(); }

impl ImageScaleUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageScaleUtil>>> {
        return ImageScaleUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            imageCopyUtil: None,
            gameFeatureFactory: None,
            features: None,
            NO_COPY: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageCopyUtil = ImageCopyUtil::getInstance();
        this.gameFeatureFactory = GameFeatureFactory::getInstance();
        this.features = Features::getInstance();
        this.NO_COPY = String::from("SWT should not copy images after initial loading as the alpha is not honored");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImage2(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool) -> Option<Rc<RefCell<Image>>> {
        return self.createImage3(imageCache.clone(), originalImage.clone(), scaleNominatorX.clone(), scaleDenominatorX.clone(), scaleNominatorY.clone(), scaleDenominatorY.clone(), cached.clone(), true);
    }
    

    // throws Exception
    pub fn createImage3(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<Image>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool, mut mutable: bool) -> Option<Rc<RefCell<Image>>> {
        if !jcall!(self.features).isFeature(jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
            return originalImage.clone();
        }
        let mut width: i32 = jcall!(originalImage).getWidth();
        let mut height: i32 = jcall!(originalImage).getHeight();
        let mut scaleX: f32 = scaleNominatorX / scaleDenominatorX;
        let mut scaleY: f32 = scaleNominatorY / scaleDenominatorY;
        let mut scaledImage: Option<Rc<RefCell<Image>>> = jcall!(self.imageCopyUtil).createImageWH(originalImage.clone(), (((scaleX * ((width) as f32))) as i32), (((scaleY * ((height) as f32))) as i32), mutable.clone());
        return scaledImage.clone();
    }
    

    // throws Exception
    pub fn scale2(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut ximageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut unused: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32) {
        self.scale(originalImage.clone(), originalImageArray.clone(), ximageToShowArray.clone(), unused.clone(), scaleX.clone(), scaleY.clone(), maxScaleX.clone(), maxScaleY.clone(), true);
    }
    

    // throws Exception
    pub fn scale(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut originalImageArray: Vec<Option<Rc<RefCell<Image>>>>, mut ximageToShowArray: Vec<Option<Rc<RefCell<Image>>>>, mut unused: i32, mut scaleX: f32, mut scaleY: f32, mut maxScaleX: f32, mut maxScaleY: f32, mut mutable: bool) {
        let mut width: i32 = jcall!(originalImage).getWidth();
        let mut height: i32 = jcall!(originalImage).getHeight();
        let mut scaledImage: Option<Rc<RefCell<Image>>> = jcall!(self.imageCopyUtil).createImageWH(originalImage.clone(), (((scaleX * ((width) as f32))) as i32), (((scaleY * ((height) as f32))) as i32), mutable.clone());
        originalImageArray[(0) as usize] = scaledImage.clone();
    }
}

