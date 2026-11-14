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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::game::configuration::feature::GameFeatureFactory::GameFeatureFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLUtil::OpenGLUtil;
use crate::org::allbinary::image::opengles::OpenGLESDeviceImageTranslate::OpenGLESDeviceImageTranslate;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::image::opengles::OpenGLESImageExclusionUtil::OpenGLESImageExclusionUtil;
use crate::org::allbinary::image::opengles::OpenGLESImageProperties::OpenGLESImageProperties;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::eclipse::swt::graphics::ImageData::ImageData;
use crate::org::microemu::app::ui::swt::SwtDeviceComponent::SwtDeviceComponent;
use crate::org::microemu::device::swt::PostLoadSwtImmutableImageProcessor::PostLoadSwtImmutableImageProcessor;
use crate::org::microemu::device::swt::PostLoadSwtMutableImageProcessor::PostLoadSwtMutableImageProcessor;
use crate::org::microemu::device::swt::SwtImmutableImage::SwtImmutableImage;
use crate::org::microemu::device::swt::SwtMutableImage::SwtMutableImage;
use crate::java::lang::Exception::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageCopyUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub gameFeatureFactory: Option<Rc<RefCell<GameFeatureFactory>>>,
    pub features: Option<Rc<RefCell<Features>>>,
    pub openGLFeatureFactory: Option<Rc<RefCell<OpenGLFeatureFactory>>>,
    pub openGLUtil: Option<Rc<RefCell<OpenGLUtil>>>,
    pub openGLESImageExclusionUtil: Option<Rc<RefCell<OpenGLESImageExclusionUtil>>>,
    pub NO_COPY: String,
    pub UNABLE: String,
    pub INFORMATION: String,
    pub LEADERBOARD: String,
}

thread_local! { pub static ImageCopyUtil_instance: Option<Rc<RefCell<ImageCopyUtil>>> = ImageCopyUtil::new(); }

impl ImageCopyUtil {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageCopyUtil>>> {
        return ImageCopyUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            gameFeatureFactory: None,
            features: None,
            openGLFeatureFactory: None,
            openGLUtil: None,
            openGLESImageExclusionUtil: None,
            NO_COPY: String::new(),
            UNABLE: String::new(),
            INFORMATION: String::new(),
            LEADERBOARD: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.gameFeatureFactory = GameFeatureFactory::getInstance();
        this.features = Features::getInstance();
        this.openGLFeatureFactory = OpenGLFeatureFactory::getInstance();
        this.openGLUtil = OpenGLUtil::getInstance();
        this.openGLESImageExclusionUtil = OpenGLESImageExclusionUtil::getInstance();
        this.NO_COPY = String::from("SWT should not copy images after initial loading as the alpha is not honored");
        this.UNABLE = String::from("Unable to scale image without dimensions");
        this.INFORMATION = String::from("about_");
        this.LEADERBOARD = String::from("leaderboard");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn createImageForRotation(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        if jcall!(self.features).isFeature({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = originalImage.clone();
            image = { let java_value = (jcall!(self.openGLUtil).addImage(image.clone())).clone(); java_value };
            return image.clone();
        } else {
            return self.createImage(originalImage.clone());
        }
    }
    

    // throws Exception
    pub fn createImage(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return originalImage.clone();
        }
        let mut originalImage2: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = originalImage.clone();
        if jcall!(self.features).isFeature({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            if jcall!(originalImage).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })());
                originalImage2 = { let java_value = (jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLBitmap.clone(); java_field }).getImage()).clone(); java_value };
            }
        }
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(originalImage2).isMutable() {
            let mut originalMutableImage: Option<Rc<RefCell<SwtMutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            image = { let java_value = (SwtMutableImage::new1(SwtDeviceComponent::createImage(jcall!({ let java_field = jcall!(originalMutableImage).image.clone(); java_field }).as_Image_mut().getImageData())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
        } else {
            let mut originalImmutableImage: Option<Rc<RefCell<SwtImmutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            image = { let java_value = (SwtMutableImage::new1(SwtDeviceComponent::createImage(jcall!({ let java_field = jcall!(originalImmutableImage).image.clone(); java_field }).as_Image_mut().getImageData())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
        }
        image = { let java_value = (jcall!(self.openGLUtil).addImage(image.clone())).clone(); java_value };
        return image.clone();
    }
    

    // throws Exception
    pub fn createImageWH(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut width: i32, mut height: i32, mut mutable: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut originalImage2: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = originalImage.clone();
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return originalImage.clone();
        }
        if jcall!(self.features).isFeature({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            if jcall!(self.openGLESImageExclusionUtil).isCustomScaling(originalImage.clone()) {
                return self.createImage2(originalImage.clone(), width.clone(), height.clone(), mutable.clone());
            }
            if jcall!(originalImage).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })());
                originalImage2 = { let java_value = (jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLBitmap.clone(); java_field }).getImage()).clone(); java_value };
            }
        }
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(originalImage2).isMutable() && mutable {
            let mut originalMutableImage: Option<Rc<RefCell<SwtMutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            if jcall!(originalMutableImage).getImage().is_some() {
                let mut imageData2: Option<Rc<RefCell<ImageData>>> = jcall!({ let java_field = jcall!(originalMutableImage).image.clone(); java_field }).as_Image_mut().getImageData();
                let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(imageData2).scaledTo(width.clone(), height.clone());
                image = { let java_value = (SwtMutableImage::new1(SwtDeviceComponent::createImage_ImageData(imageData.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
            } else {
                image = { let java_value = (SwtMutableImage::new2(jcall!(originalMutableImage).getName(), PostLoadSwtMutableImageProcessor::new(originalMutableImage.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
            }
        } else {
            let mut originalImmutableImage: Option<Rc<RefCell<SwtImmutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            if jcall!(originalImmutableImage).getImage().is_some() {
                let mut imageData2: Option<Rc<RefCell<ImageData>>> = jcall!({ let java_field = jcall!(originalImmutableImage).image.clone(); java_field }).as_Image_mut().getImageData();
                if width > 0 && height > 0 {
                    let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(imageData2).scaledTo(width.clone(), height.clone());
                    image = { let java_value = (SwtImmutableImage::new2(jcall!(originalImage).getName(), SwtDeviceComponent::createImage_ImageData(imageData.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                } else {
                    jcall!(self.logUtil).putF(self.UNABLE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
                    image = { let java_value = (SwtImmutableImage::new2(jcall!(originalImmutableImage).getName(), PostLoadSwtImmutableImageProcessor::new(originalImmutableImage.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                }
            } else {
                image = { let java_value = (SwtImmutableImage::new2(jcall!(originalImmutableImage).getName(), PostLoadSwtImmutableImageProcessor::new(originalImmutableImage.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
            }
        }
        image = { let java_value = (jcall!(self.openGLUtil).addImage(image.clone())).clone(); java_value };
        return image.clone();
    }
    

    // throws Exception
    pub fn createImage2(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut width: i32, mut height: i32, mut mutable: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        let mut originalImage2: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = originalImage.clone();
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return originalImage.clone();
        }
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = None;
        if jcall!(self.features).isFeature({ let java_field = jcall!(self.openGLFeatureFactory).OPENGL.clone(); java_field }.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            let mut displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
            if jcall!(originalImage).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })());
                let mut openGLESImageProperties: Option<Rc<RefCell<OpenGLESImageProperties>>> = { let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }.clone();
                let mut openGLWidth: i32 = jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLBitmap.clone(); java_field }).getWidth();
                let mut openGLHeight: i32 = jcall!({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLBitmap.clone(); java_field }).getHeight();
                jcall!(openGLESImageProperties).scaleX2 = ((((width) as f32)) / ((openGLWidth) as f32));
                jcall!(openGLESImageProperties).scaleX = jcall!(openGLESImageProperties).scaleX2;
                jcall!(openGLESImageProperties).scaleY2 = ((((height) as f32)) / ((openGLHeight) as f32));
                jcall!(openGLESImageProperties).scaleY = jcall!(openGLESImageProperties).scaleY2;
                jcall!(openGLESImageProperties).scaleX = { let java_field = jcall!(openGLESImageProperties).scaleX.clone(); java_field } * 0.75_f32;
                jcall!(openGLESImageProperties).scaleY = { let java_field = jcall!(openGLESImageProperties).scaleY.clone(); java_field } * 0.75_f32;
                if jcall!(originalImage).getHeight() % 2 != 0 {
                    jcall!(openGLESImageProperties).scaleX2 += 1.0_f32 / ((openGLWidth) as f32);
                    jcall!(openGLESImageProperties).scaleY2 += 1.0_f32 / ((openGLHeight) as f32);
                }
                image = openGLESImage.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>);
            } else {
                image = originalImage.clone();
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = (|| -> Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> { panic!("ClassCastException: a Rust dyn Image cannot be cast to OpenGLESImage") })();
                let mut openGLESImageProperties: Option<Rc<RefCell<OpenGLESImageProperties>>> = { let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties.clone(); java_field }.clone();
                jcall!(openGLESImageProperties).scaleX2 = ((((width) as f32)) / ((jcall!(openGLESImage).getWidth()) as f32));
                jcall!(openGLESImageProperties).scaleX = jcall!(openGLESImageProperties).scaleX2;
                jcall!(openGLESImageProperties).scaleY2 = ((((height) as f32)) / ((jcall!(openGLESImage).getHeight()) as f32));
                jcall!(openGLESImageProperties).scaleY = jcall!(openGLESImageProperties).scaleY2;
                if jcall!(image).getName().starts_with(&self.INFORMATION.clone()) {
                    jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageTranslate = OpenGLESDeviceImageTranslate::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslateDyn>>);
                    let mut openGLESDeviceImageTranslate: Option<Rc<RefCell<OpenGLESDeviceImageTranslate>>> = ({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageTranslate.clone(); java_field }).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLESDeviceImageTranslate>() }, "ClassCastException: not a OpenGLESDeviceImageTranslate"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLESDeviceImageTranslate>) } });
                    jcall!(openGLESDeviceImageTranslate).translateX = ((jcall!(displayInfoSingleton).getLastWidth()) as f32) / 1.4_f32 / { let java_field = jcall!(openGLESImageProperties).scaleX.clone(); java_field };
                    jcall!(openGLESDeviceImageTranslate).translateY = ((jcall!(displayInfoSingleton).getLastHeight() / 4) as f32) / { let java_field = jcall!(openGLESImageProperties).scaleY.clone(); java_field };
                    jcall!(openGLESImageProperties).scaleX = { let java_field = jcall!(openGLESImageProperties).scaleX.clone(); java_field } * 0.25_f32;
                    jcall!(openGLESImageProperties).scaleY = { let java_field = jcall!(openGLESImageProperties).scaleY.clone(); java_field } * 0.25_f32;
                } else {
                    jcall!(openGLESImageProperties).scaleX = { let java_field = jcall!(openGLESImageProperties).scaleX.clone(); java_field } * 0.75_f32;
                    jcall!(openGLESImageProperties).scaleY = { let java_field = jcall!(openGLESImageProperties).scaleY.clone(); java_field } * 0.75_f32;
                    if jcall!(image).getName().starts_with(&self.LEADERBOARD.clone()) {
                        jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageTranslate = OpenGLESDeviceImageTranslate::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImageTranslate::OpenGLESImageTranslateDyn>>);
                        let mut openGLESDeviceImageTranslate: Option<Rc<RefCell<OpenGLESDeviceImageTranslate>>> = ({ let java_field = jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageTranslate.clone(); java_field }).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<OpenGLESDeviceImageTranslate>() }, "ClassCastException: not a OpenGLESDeviceImageTranslate"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<OpenGLESDeviceImageTranslate>) } });
                        jcall!(openGLESDeviceImageTranslate).translateX = ((-jcall!(displayInfoSingleton).getLastWidth() / 40) as f32) / { let java_field = jcall!(openGLESImageProperties).scaleX.clone(); java_field };
                    }
                    if jcall!(image).getName().starts_with(&self.LEADERBOARD.clone()) {
                    } else {
                        jcall!(openGLESImageProperties).scaleX = 1.0_f32;
                        jcall!(openGLESImageProperties).scaleY = 1.0_f32;
                    }
                }
                image = openGLESImage.clone().map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>);
            }
        } else {
            if jcall!(originalImage2).isMutable() && mutable {
                let mut originalMutableImage: Option<Rc<RefCell<SwtMutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
                if jcall!(originalMutableImage).getImage().is_some() {
                    let mut imageData2: Option<Rc<RefCell<ImageData>>> = jcall!({ let java_field = jcall!(originalMutableImage).image.clone(); java_field }).as_Image_mut().getImageData();
                    let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(imageData2).scaledTo(width.clone(), height.clone());
                    image = { let java_value = (SwtMutableImage::new1(SwtDeviceComponent::createImage_ImageData(imageData.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                } else {
                    image = { let java_value = (SwtMutableImage::new2(jcall!(originalMutableImage).getName(), PostLoadSwtMutableImageProcessor::new(originalMutableImage.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                }
            } else {
                let mut originalImmutableImage: Option<Rc<RefCell<SwtImmutableImage>>> = (originalImage2).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
                if jcall!(originalImmutableImage).getImage().is_some() {
                    let mut imageData2: Option<Rc<RefCell<ImageData>>> = jcall!({ let java_field = jcall!(originalImmutableImage).image.clone(); java_field }).as_Image_mut().getImageData();
                    let mut imageData: Option<Rc<RefCell<ImageData>>> = jcall!(imageData2).scaledTo(width.clone(), height.clone());
                    image = { let java_value = (SwtImmutableImage::new2(jcall!(originalImage).getName(), SwtDeviceComponent::createImage_ImageData(imageData.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                } else {
                    image = { let java_value = (SwtImmutableImage::new2(jcall!(originalImmutableImage).getName(), PostLoadSwtImmutableImageProcessor::new(originalImmutableImage.clone())).map(|reference| reference as Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>)).clone(); java_value };
                }
            }
            image = { let java_value = (jcall!(self.openGLUtil).addImage(image.clone())).clone(); java_value };
        }
        return image.clone();
    }
    

    // throws Exception
    pub fn createImageScale(&mut self, mut originalImage: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>, mut canvasScale: f32, mut resize: bool) -> Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> {
        if !jcall!(self.features).isFeature({ let java_field = jcall!(self.gameFeatureFactory).POST_IMAGE_LOADING_MODIFICATION.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(self.NO_COPY.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).CONSTRUCTOR.clone(); java_field }.clone());
            return originalImage.clone();
        }
        let mut newWidth: i32 = (((((jcall!(originalImage).getWidth()) as f32) * canvasScale)) as i32);
        let mut newHeight: i32 = (((((jcall!(originalImage).getHeight()) as f32) * canvasScale)) as i32);
        if resize {
            if newWidth < newHeight {
                newWidth = newHeight.clone();
            }
            if newHeight < newWidth {
                newHeight = newWidth.clone();
            }
        }
        let mut image: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>> = self.createImageWH(originalImage.clone(), newWidth.clone(), newHeight.clone(), true);
        if jcall!(image).isMutable() {
            let mut originalImageData: Option<Rc<RefCell<ImageData>>> = jcall!(({ let java_object = (jcall!(originalImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) })).as_Image_mut().getImageData();
            let mut mutableImage2: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtMutableImage>() }, "ClassCastException: not a SwtMutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtMutableImage>) } });
            let mut halfWidthDelta: i32 = (newWidth - jcall!(originalImage).getWidth()) / 2;
            let mut halfHeightDelta: i32 = (newHeight - jcall!(originalImage).getHeight()) / 2;
            let mut originalPixelArray: Vec<i32> = vec![0; (jcall!(originalImage).getWidth() * jcall!(originalImage).getHeight()) as usize];
            let mut newPixelArray: Vec<i32> = vec![0; (jcall!(image).getWidth() * jcall!(image).getHeight()) as usize];
            let mut width: i32 = jcall!(originalImage).getWidth();
            let mut height: i32 = jcall!(originalImage).getHeight();
            for i in (0..height) {
                jcall!(originalImageData).getPixels(0, i.clone(), width.clone(), originalPixelArray.clone(), (i * width).clone());
            }
            for index in (halfWidthDelta..width) {
                for index2 in (halfHeightDelta..height) {
                    newPixelArray[(index + (index2 * width)) as usize] = { let java_value = (originalPixelArray[((index - halfWidthDelta) + ((index2 - halfHeightDelta) * width)) as usize]).clone(); java_value }.clone();
                }
            }
            jcall!(jcall!(({ let java_object = (jcall!(mutableImage2).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) })).as_Image_mut().getImageData()).setPixels(0, 0, jcall!(image).getWidth(), newPixelArray.clone(), 0);
            return image.clone();
        } else {
            let mut originalImmutableImage: Option<Rc<RefCell<SwtImmutableImage>>> = (image).clone().map(|reference| { assert!({ let java_borrowed = reference.borrow(); let java_any: &dyn std::any::Any = &*java_borrowed; java_any.is::<SwtImmutableImage>() }, "ClassCastException: not a SwtImmutableImage"); unsafe { Rc::from_raw(Rc::into_raw(reference) as *const RefCell<SwtImmutableImage>) } });
            let mut originalImageData: Option<Rc<RefCell<ImageData>>> = jcall!(({ let java_object = (jcall!(originalImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) })).as_Image_mut().getImageData();
            let mut halfWidthDelta: i32 = (newWidth - jcall!(originalImage).getWidth()) / 2;
            let mut halfHeightDelta: i32 = (newHeight - jcall!(originalImage).getHeight()) / 2;
            let mut originalPixelArray: Vec<i32> = vec![0; (jcall!(originalImage).getWidth() * jcall!(originalImage).getHeight()) as usize];
            let mut newPixelArray: Vec<i32> = vec![0; (jcall!(image).getWidth() * jcall!(image).getHeight()) as usize];
            let mut width: i32 = jcall!(originalImage).getWidth();
            let mut height: i32 = jcall!(originalImage).getHeight();
            for i in (0..height) {
                jcall!(originalImageData).getPixels(0, i.clone(), width.clone(), originalPixelArray.clone(), (i * width).clone());
            }
            for index in (halfWidthDelta..width) {
                for index2 in (halfHeightDelta..height) {
                    newPixelArray[(index + (index2 * width)) as usize] = { let java_value = (originalPixelArray[((index - halfWidthDelta) + ((index2 - halfHeightDelta) * width)) as usize]).clone(); java_value }.clone();
                }
            }
            jcall!(jcall!(({ let java_object = (jcall!(originalImmutableImage).getImage()).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>().expect("ClassCastException").clone()) })).as_Image_mut().getImageData()).setPixels(0, 0, jcall!(image).getWidth(), newPixelArray.clone(), 0);
            return image.clone();
        }
    }
}

