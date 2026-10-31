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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory;
use crate::org::allbinary::image::AnimationFrameToImageUtil::AnimationFrameToImageUtil;
use super::ImageAnimation::ImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageAnimationSingletonInterfaceFactory {
    pub base: SingletonAnimationInterfaceFactory,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for ImageAnimationSingletonInterfaceFactory {
}

impl ImageAnimationSingletonInterfaceFactory {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            base: SingletonAnimationInterfaceFactory::new_struct(ImageAnimation::new(jcall!(AnimationFrameToImageUtil::getInstance()).getInstanceWH(width.clone(), height.clone(), animationInterface.clone()), AnimationBehavior::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::ImageAnimation::ImageAnimationDyn>>)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, width, height))))
    }
}
impl std::ops::Deref for ImageAnimationSingletonInterfaceFactory {
    type Target = SingletonAnimationInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageAnimationSingletonInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactoryDyn for ImageAnimationSingletonInterfaceFactory {
    fn as_SingletonAnimationInterfaceFactory(&self) -> &crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory { &self.base }
    fn as_SingletonAnimationInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory { &mut self.base }
}


