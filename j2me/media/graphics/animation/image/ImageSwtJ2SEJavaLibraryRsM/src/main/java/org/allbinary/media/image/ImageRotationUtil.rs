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
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::eclipse::swt::graphics::GC::GC;
use crate::org::microemu::device::swt::SwtImmutableImage::SwtImmutableImage;
use crate::org::microemu::device::swt::SwtMutableImage::SwtMutableImage;
use super::ImageSwtRotationUtil::ImageSwtRotationUtil;
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

pub struct ImageRotationUtil {
    pub imageSwtRotationUtil: Option<Rc<RefCell<ImageSwtRotationUtil>>>,
    pub features: Option<Rc<RefCell<Features>>>,
}

thread_local! { pub static ImageRotationUtil_instance: Option<Rc<RefCell<ImageRotationUtil>>> = ImageRotationUtil::new(); }

impl ImageRotationUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ImageRotationUtil>>> {
        return ImageRotationUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageSwtRotationUtil: None,
            features: None,
        };
        this.imageSwtRotationUtil = ImageSwtRotationUtil::getInstance();
        this.features = Features::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn rotateImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut image: Option<Rc<RefCell<Image>>>, mut totalAngle: i32) {
        let mut originalImage2: Option<Rc<RefCell<Image>>> = originalImage.clone();
        if { let java_receiver = self.features.clone(); let java_arguments = (jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>),); let java_result = jcall!(java_receiver).isFeature(java_arguments.0); java_result } {
            if jcall!(image).getType() >= crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImage_TYPE.with(Clone::clone) {
                let mut openGLESImage: Option<Rc<RefCell<dyn crate::org::allbinary::image::opengles::OpenGLESImage::OpenGLESImageDyn>>> = ((image).clone() /* as OpenGLESImage */);
                jcall!(jcall!(openGLESImage).as_OpenGLESImage_mut().openGLESImageProperties).angle = totalAngle.clone();
                return;
            } else {
                PreLogUtil::put(format!("{}{}", format!("{}{}", format!("{}{}", String::from("OpenGL but image is: "), image), String::from(" type: ")), jcall!(image).getType()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("rotateImage"));
            }
        }
        let mut originalSwtImage: Option<Rc<RefCell<Image>>> = ((jcall!(originalImage2).getImage()).clone() /* as Image */);
        let mut gc: Option<Rc<RefCell<GC>>> = None;
        let mut copySwtImage: Option<Rc<RefCell<Image>>> = None;
        if jcall!(image).isMutable() {
            let mut swtImage: Option<Rc<RefCell<SwtMutableImage>>> = (image).clone() /* as SwtMutableImage */;
            copySwtImage = (jcall!(swtImage).getImage()).clone() /* as Image */;
            gc = jcall!(swtImage).getGc();
            jcall!(self.imageSwtRotationUtil).rotateImage(originalSwtImage.clone(), copySwtImage.clone(), gc.clone(), totalAngle.clone());
        } else {
            PreLogUtil::put(format!("{}{}", String::from("No rotation for: "), jcall!(image).toString()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("rotateImage"));
        }
    }
    

    // throws Exception
    pub fn createRotatedImage(&mut self, mut originalImage: Option<Rc<RefCell<Image>>>, mut rotationInDegrees: i32) -> Option<Rc<RefCell<Image>>> {
        return originalImage.clone();
    }
}

