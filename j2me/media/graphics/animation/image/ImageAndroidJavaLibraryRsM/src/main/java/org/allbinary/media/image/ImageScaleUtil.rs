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
use crate::android::graphics::Bitmap::Bitmap;
use crate::android::graphics::Canvas::Canvas;
use crate::android::graphics::Matrix::Matrix;
use crate::javax::microedition::lcdui::Image::Image;
use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::Anchor::Anchor;
use crate::org::allbinary::image::ImageCache::ImageCache;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::microemu::android::device::AndroidImageInterface::AndroidImageInterface;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageScaleUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub anchor: i32,
    pub imageBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub scaledImageBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub scaleXBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub scaleYBasicArrayList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static ImageScaleUtil_instance: Option<Rc<RefCell<ImageScaleUtil>>> = ImageScaleUtil::new(); }

impl ImageScaleUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageScaleUtil>>> {
        return ImageScaleUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            anchor: 0,
            imageBasicArrayList: None,
            scaledImageBasicArrayList: None,
            scaleXBasicArrayList: None,
            scaleYBasicArrayList: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.anchor = unsafe { crate::org::allbinary::graphics::Anchor::Anchor_TOP_LEFT.clone() }.clone();
        this.imageBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.scaledImageBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.scaleXBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.scaleYBasicArrayList = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImage2(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        return self.createImage3(imageCache.clone(), originalImage.clone(), scaleNominatorX.clone(), scaleDenominatorX.clone(), scaleNominatorY.clone(), scaleDenominatorY.clone(), cached.clone(), true);
    }
    

    // throws Exception
    pub fn createImage3(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut scaleNominatorX: f32, mut scaleDenominatorX: f32, mut scaleNominatorY: f32, mut scaleDenominatorY: f32, mut cached: bool, mut mutable: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut scaleX: f32 = scaleNominatorX / scaleDenominatorX;
        let mut scaleY: f32 = scaleNominatorY / scaleDenominatorY;
        return self.createImage(imageCache.clone(), originalImage.clone(), scaleX.clone(), scaleY.clone(), cached.clone());
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut scaleX: f32, mut scaleY: f32, mut cached: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut originalAndroidImage: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })();
        let mut originalBitmap: Option<Rc<RefCell<Bitmap>>> = jcall!(originalAndroidImage).getBitmap();
        let mut width: i32 = (((jcall!(originalBitmap).getWidth() * scaleX)) as i32);
        let mut height: i32 = (((jcall!(originalBitmap).getHeight() * scaleY)) as i32);
        let mut index: i32 = jcall!(self.imageBasicArrayList).indexOf(crate::java_into_object(originalImage.clone()));
        let mut alreadyAvailable: bool = false;
        if index >= 0 {
            let mut scaleX2Float: Option<f32> = { let java_object = (jcall!(self.scaleXBasicArrayList).get(index.clone())).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<f32>().copied().or_else(|| object.downcast_ref::<Option<f32>>().copied().flatten()).expect("ClassCastException") }) };
            let mut scaleX2: f32 = ((scaleX2Float).unwrap() as f32);
            let mut scaleY2Float: Option<f32> = { let java_object = (jcall!(self.scaleYBasicArrayList).get(index.clone())).clone(); java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<f32>().copied().or_else(|| object.downcast_ref::<Option<f32>>().copied().flatten()).expect("ClassCastException") }) };
            let mut scaleY2: f32 = ((scaleY2Float).unwrap() as f32);
            if scaleX2 == scaleX && scaleY2 == scaleY {
                alreadyAvailable = true;
            }
        }
        if alreadyAvailable {
            return { let java_object = (jcall!(self.scaledImageBasicArrayList).get(index.clone())).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) };
        } else {
            let mut scaledImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = self.getScaledImage(imageCache.clone(), originalImage.clone(), scaleX.clone(), scaleY.clone(), width.clone(), height.clone(), cached.clone());
            jcall!(self.imageBasicArrayList).add(crate::java_into_object(originalImage.clone()));
            jcall!(self.scaledImageBasicArrayList).add(crate::java_into_object(scaledImage.clone()));
            jcall!(self.scaleXBasicArrayList).add(crate::java_into_object(scaleX.clone()));
            jcall!(self.scaleYBasicArrayList).add(crate::java_into_object(scaleY.clone()));
            return scaledImage.clone();
        }
    }
    

    // throws Exception
    pub fn getScaledImage(&mut self, mut imageCache: Option<Rc<RefCell<dyn crate::org::allbinary::image::ImageCache::ImageCacheDyn>>>, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut scaleX: f32, mut scaleY: f32, mut width: i32, mut height: i32, mut cached: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        if cached {
            image = { let java_value = (jcall!(imageCache).get({ let java_argument = std::any::type_name_of_val(&self).to_string(); java_argument }, width.clone(), height.clone())).clone(); java_value };
        } else {
            image = { let java_value = (jcall!(imageCache).get({ let java_field = jcall!(self.commonStrings).CREATE_IMAGE.clone(); java_field }.clone(), width.clone(), height.clone())).clone(); java_value };
        }
        if jcall!(image).isMutable() {
            let mut matrix: Option<Rc<RefCell<Matrix>>> = Matrix::new();
            self.scale(image.clone(), matrix.clone(), scaleX.clone(), scaleY.clone());
            jcall!(jcall!(image).getGraphics()).drawImage(originalImage.clone(), 0, 0, self.anchor.clone());
            return image.clone();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Not Mutable"))));
        }
    }
    

    pub fn scale(&mut self, mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut matrix: Option<Rc<RefCell<Matrix>>>, mut scaleX: f32, mut scaleY: f32) {
        jcall!(matrix).setScale(scaleX.clone(), scaleY.clone());
        let mut canvas: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Canvas::CanvasDyn>>> = jcall!(((|| -> Option<Rc<RefCell<dyn AndroidImageInterface>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to AndroidImageInterface") })())).getCanvas();
        jcall!(canvas).as_Canvas_mut().concat(matrix.clone());
    }
}

