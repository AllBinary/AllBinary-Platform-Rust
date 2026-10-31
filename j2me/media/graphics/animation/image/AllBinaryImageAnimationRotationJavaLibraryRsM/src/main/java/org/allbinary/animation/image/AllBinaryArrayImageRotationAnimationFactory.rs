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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::image::ImageToRotationImageArrayUtil::ImageToRotationImageArrayUtil;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::ImageArrayRotationAnimation::ImageArrayRotationAnimation;
use super::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryArrayImageRotationAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub angleIncrement: i32,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryArrayImageRotationAnimationFactory {
}

impl AllBinaryArrayImageRotationAnimationFactory {
    

    // throws Exception
    pub fn createFactory(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32) -> Option<Rc<RefCell<AllBinaryArrayImageRotationAnimationFactory>>> {
        let mut arrayImageRotationAnimationFactory: Option<Rc<RefCell<AllBinaryArrayImageRotationAnimationFactory>>> = AllBinaryArrayImageRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity(), AnimationBehaviorFactory::getInstance());
        jcall!(arrayImageRotationAnimationFactory).initDXY(dx.clone(), dy.clone());
        return arrayImageRotationAnimationFactory.clone();
    }
    

    // throws Exception
    pub fn createFactoryA(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut angleIncrement: i32) -> Option<Rc<RefCell<AllBinaryArrayImageRotationAnimationFactory>>> {
        let mut arrayImageRotationAnimationFactory: Option<Rc<RefCell<AllBinaryArrayImageRotationAnimationFactory>>> = AllBinaryArrayImageRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), angleIncrement.clone(), AnimationBehaviorFactory::getInstance());
        jcall!(arrayImageRotationAnimationFactory).initDXY(dx.clone(), dy.clone());
        return arrayImageRotationAnimationFactory.clone();
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone()),
            imageArray: Vec::new(),
            angleIncrement: 0,
        };
        this.imageArray = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.angleIncrement = angleIncrement.clone();
        this.init();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, width, height, angleIncrement, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        { let java_arguments = (jcall!(ImageToRotationImageArrayUtil::getInstance()).generate(self.getImage(), self.getAngleIncrement(), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32)),); self.setImageArray(java_arguments.0) };
    }
    

    pub fn initDXY(&mut self, mut dx: i32, mut dy: i32) {
        jcall!(self.animationFactoryInitializationVisitor).dx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).dy = dy.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDy = dy.clone();
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut scaledImageArray: Vec<Option<Rc<RefCell<Image>>>> = vec![None; ((self.imageArray.len() as i32)) as usize];
        let mut size: i32 = (scaledImageArray.len() as i32).clone();
        let mut image: Option<Rc<RefCell<Image>>> = self.getImage();
        for index in (0..size) {
            scaledImageArray[(index) as usize] = { let java_receiver = self.animationFactoryImageScaleUtil.clone(); let java_arguments = (image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        }
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            jcall!(self.animationFactoryImageScaleUtil).processAdjust(None /* Java this: a &mut self method has no Rc of itself */);
            return AdjustedImageArrayRotationAnimation::new(scaledImageArray.clone(), AngleInfo::getInstance(((self.getAngleIncrement()) as i16)), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return ImageArrayRotationAnimation::new(scaledImageArray.clone(), AngleInfo::getInstance(((self.getAngleIncrement()) as i16)), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
    

    pub fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.imageArray = imageArray.clone();
    }
    

    pub fn getAngleIncrement(&mut self) -> i32 {
        return self.angleIncrement.clone();
    }
}
impl std::ops::Deref for AllBinaryArrayImageRotationAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryArrayImageRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryArrayImageRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


