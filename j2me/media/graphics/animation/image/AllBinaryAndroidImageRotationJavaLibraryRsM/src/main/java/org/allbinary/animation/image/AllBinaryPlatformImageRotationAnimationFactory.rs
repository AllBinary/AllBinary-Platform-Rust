// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use super::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryPlatformImageRotationAnimationFactory {
    pub base: AllBinaryAndroidImageRotationAnimationFactory,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryPlatformImageRotationAnimationFactory {
}

impl AllBinaryPlatformImageRotationAnimationFactory {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Self {
        let mut this = Self {
            base: AllBinaryAndroidImageRotationAnimationFactory::new_struct(image.clone(), width.clone(), height.clone(), angleIncrement.clone(), animationBehaviorFactory.clone(), resizeCanvasForRotation.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, width, height, angleIncrement, animationBehaviorFactory, resizeCanvasForRotation))))
    }
}
impl std::ops::Deref for AllBinaryPlatformImageRotationAnimationFactory {
    type Target = AllBinaryAndroidImageRotationAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryPlatformImageRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactoryDyn for AllBinaryPlatformImageRotationAnimationFactory {
    fn as_AllBinaryAndroidImageRotationAnimationFactory(&self) -> &crate::org::allbinary::animation::image::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactory { &self.base }
    fn as_AllBinaryAndroidImageRotationAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactory { &mut self.base }
}
impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryPlatformImageRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base.base }
    fn getInstance(&mut self, p1: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::animation::image::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactory::getInstance(crate::org::allbinary::animation::image::AllBinaryAndroidImageRotationAnimationFactory::AllBinaryAndroidImageRotationAnimationFactoryDyn::as_AllBinaryAndroidImageRotationAnimationFactory_mut(self), p1) }
}


