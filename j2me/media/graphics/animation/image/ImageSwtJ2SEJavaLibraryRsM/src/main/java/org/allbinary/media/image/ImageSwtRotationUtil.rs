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

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

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
    

    pub fn getRotatedImage(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut totalAngle: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return bufferedImage.clone();
        }
        return bufferedImage.clone();
    }
    

    pub fn getRotatedImage2(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut gc: Option<Rc<RefCell<GC>>>, mut angle: f32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return bufferedImage.clone();
        }
        return bufferedImage.clone();
    }
    

    pub fn rotateImage(&mut self, mut originalSwtImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newSwtImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut gc: Option<Rc<RefCell<GC>>>, mut totalAngle: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.getRotatedImage2(originalSwtImage.clone(), newSwtImage.clone(), gc.clone(), totalAngle.clone());
    }
    

    pub fn getRotatedImageSWT(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut newSwtImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut gc: Option<Rc<RefCell<GC>>>, mut totalAngle: i32) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.getRotatedImage2(bufferedImage.clone(), newSwtImage.clone(), gc.clone(), totalAngle.clone());
    }
    

    pub fn getRotatedImages(&mut self, mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut numberOfFrames: i32, mut totalAngle: i32) -> Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> {
        let mut bufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>> = vec![None; (numberOfFrames) as usize];
        let mut angle: i32 = 0;
        let mut size: i32 = (bufferedImageArray.len() as i32).clone();
        for index in (0..size) {
            angle = (totalAngle / size) * index;
            bufferedImageArray[(index) as usize] = self.getRotatedImage(bufferedImage.clone(), angle.clone());
        }
        return bufferedImageArray.clone();
    }
    

    pub fn createSpriteImage(&mut self, mut bufferedImageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
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
        let mut firstBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = bufferedImageArray[(0) as usize].clone();
        let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(firstBufferedImage).as_Image_mut().getImageData();
        let mut bufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = jcall!(self.imageUtil).create({ let java_field = jcall!(imageData).width.clone(); java_field } * columns, { let java_field = jcall!(imageData).height.clone(); java_field } * rows);
        let mut gc: Option<Rc<RefCell<GC>>> = GC::new(bufferedImage.clone());
        jcall!(gc).setAntialias(unsafe { crate::org::eclipse::swt::SWT::SWT_ON.clone() }.clone());
        jcall!(gc).setInterpolation(unsafe { crate::org::eclipse::swt::SWT::SWT_HIGH.clone() }.clone());
        let mut columnIndex: i32 = 0;
        let mut rowIndex: i32 = 0;
        let mut nextBufferedImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        for index in (0..size) {
            if index / 9 != 0 && index % 9 == 0 {
                rowIndex += 1;
                columnIndex = 0;
            }
            nextBufferedImage = { let java_value = (bufferedImageArray[(index) as usize]).clone(); java_value }.clone();
            imageData = { let java_value = (jcall!(nextBufferedImage).as_Image_mut().getImageData()).clone(); java_value };
            jcall!(gc).drawImage(bufferedImage.clone(), 0, 0, { let java_field = jcall!(jcall!(bufferedImage).as_Image_mut().getBounds()).width.clone(); java_field }.clone(), { let java_field = jcall!(jcall!(bufferedImage).as_Image_mut().getBounds()).height.clone(); java_field }.clone(), { let java_field = jcall!(imageData).width.clone(); java_field } * columnIndex, { let java_field = jcall!(imageData).height.clone(); java_field } * rowIndex, { let java_field = jcall!(imageData).width.clone(); java_field }.clone(), { let java_field = jcall!(imageData).height.clone(); java_field }.clone());
            columnIndex += 1;
        }
        jcall!(gc).dispose();
        return bufferedImage.clone();
    }
}

