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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::eclipse::swt::SWT::SWT;
use crate::org::eclipse::swt::graphics::GC::GC;
use crate::org::eclipse::swt::graphics::ImageData::ImageData;
use super::ImageUtil::ImageUtil;
use super::Image::Image;
use super::graphics::graphics;
use super::swt::swt;
use super::eclipse::eclipse;
use super::org::org;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageSwtRotationUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub imageUtil: Option<Rc<RefCell<ImageUtil>>>,
    pub gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>>,
    pub features: Option<Rc<RefCell<Features>>>,
    pub NO_COPY: String,
}

thread_local! { pub static ImageSwtRotationUtil_instance: Option<Rc<RefCell<ImageSwtRotationUtil>>> = ImageSwtRotationUtil::new(); }

impl ImageSwtRotationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageSwtRotationUtil>>> {
        return ImageSwtRotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            imageUtil: None,
            gameFeatureFactory: None,
            features: None,
            NO_COPY: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.imageUtil = ImageUtil::getInstance();
        this.gameFeatureFactory = GameFeatureFactory::getInstance();
        this.features = Features::getInstance();
        this.NO_COPY = String::from("SWT should not copy images after initial loading as the alpha is not honored");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getRotatedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        if !jcall!(self.features).isFeature(jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
            return bufferedImage.clone();
        }
        return bufferedImage.clone();
    }
    

    pub fn getRotatedImage2(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut newBufferedImage: Option<Rc<RefCell<Image>>>, mut gc: Option<Rc<RefCell<GC>>>, mut angle: f32) -> Option<Rc<RefCell<Image>>> {
        if !jcall!(self.features).isFeature(jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).CONSTRUCTOR.clone());
            return bufferedImage.clone();
        }
        return bufferedImage.clone();
    }
    

    pub fn rotateImage(&mut self, mut originalSwtImage: Option<Rc<RefCell<Image>>>, mut newSwtImage: Option<Rc<RefCell<Image>>>, mut gc: Option<Rc<RefCell<GC>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        return self.getRotatedImage2(originalSwtImage.clone(), newSwtImage.clone(), gc.clone(), totalAngle.clone());
    }
    

    pub fn getRotatedImageSWT(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut newSwtImage: Option<Rc<RefCell<Image>>>, mut gc: Option<Rc<RefCell<GC>>>, mut totalAngle: i32) -> Option<Rc<RefCell<Image>>> {
        return self.getRotatedImage2(bufferedImage.clone(), newSwtImage.clone(), gc.clone(), totalAngle.clone());
    }
    

    pub fn getRotatedImages(&mut self, mut bufferedImage: Option<Rc<RefCell<Image>>>, mut numberOfFrames: i32, mut totalAngle: i32) -> Vec<Option<Rc<RefCell<Image>>>> {
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; (numberOfFrames) as usize];
        let mut angle: i32 = 0;
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        for index in (0..size) {
            angle = (totalAngle / size) * index;
            bufferedImageArray[(index) as usize] = self.getRotatedImage(bufferedImage.clone(), angle.clone());
        }
        return bufferedImageArray.clone();
    }
    

    pub fn createSpriteImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<Image>>>>) -> Option<Rc<RefCell<Image>>> {
        let mut columns: i32 = 9;
        let mut rows: i32 = 0;
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        if size < columns {
            columns = size.clone();
        }
        rows = (size / columns);
        if size % columns != 0 {
            rows += 1;
        }
        let mut firstBufferedImage: Option<Rc<RefCell<Image>>> = bufferedImageArray[(0) as usize].clone();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(firstBufferedImage).getImageData();
        let mut bufferedImage: Option<Rc<RefCell<Image>>> = jcall!(self.imageUtil).create(jcall!(imageData).width * columns, jcall!(imageData).height * rows);
        let mut gc: Option<Rc<RefCell<GC>>> = GC::new(bufferedImage.clone());
        jcall!(gc).setAntialias(unsafe { crate::org::eclipse::swt::SWT::SWT_ON.clone() }.clone());
        jcall!(gc).setInterpolation(unsafe { crate::org::eclipse::swt::SWT::SWT_HIGH.clone() }.clone());
        let mut columnIndex: i32 = 0;
        let mut rowIndex: i32 = 0;
        let mut nextBufferedImage: Option<Rc<RefCell<Image>>> = None;
        for index in (0..size) {
            if index / 9 != 0 && index % 9 == 0 {
                rowIndex += 1;
                columnIndex = 0;
            }
            nextBufferedImage = bufferedImageArray[(index) as usize].clone();
            imageData = jcall!(nextBufferedImage).getImageData();
            jcall!(gc).drawImage(bufferedImage.clone(), 0, 0, jcall!(jcall!(bufferedImage).getBounds()).width.clone(), jcall!(jcall!(bufferedImage).getBounds()).height.clone(), jcall!(imageData).width * columnIndex, jcall!(imageData).height * rowIndex, jcall!(imageData).width.clone(), jcall!(imageData).height.clone());
            columnIndex += 1;
        }
        jcall!(gc).dispose();
        return bufferedImage.clone();
    }
}

