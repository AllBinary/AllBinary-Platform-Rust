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
use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory;
use crate::org::allbinary::image::IndexedAnimationToImageArrayUtil::IndexedAnimationToImageArrayUtil;
use crate::java::lang::Exception::*;
use super::ImageArrayAnimation::ImageArrayAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArraySingletonAnimationInterfaceFactory {
    pub base: SingletonAnimationInterfaceFactory,
}

impl crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface for ImageArraySingletonAnimationInterfaceFactory {
    fn getInstance(&mut self, p1: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory::getInstance(crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactoryDyn::as_SingletonAnimationInterfaceFactory_mut(self), p1) }
    fn setInitialScale(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::media::ScaleProperties::ScaleProperties>>>) { crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory::setInitialScale(crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactoryDyn::as_SingletonAnimationInterfaceFactory_mut(self), p1) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for ImageArraySingletonAnimationInterfaceFactory {
}
impl crate::org::allbinary::animation::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface for ImageArraySingletonAnimationInterfaceFactory {
    fn getInstanceAnimation(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory::getInstanceAnimation(crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactoryDyn::as_SingletonAnimationInterfaceFactory_mut(self), p1) }
}

impl ImageArraySingletonAnimationInterfaceFactory {
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>) -> Self {
        let mut this = Self {
            base: SingletonAnimationInterfaceFactory::new_struct(ImageArrayAnimation::new(imageArray.clone(), AnimationBehavior::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Image::ImageDyn>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray))))
    }
}
impl std::ops::Deref for ImageArraySingletonAnimationInterfaceFactory {
    type Target = SingletonAnimationInterfaceFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArraySingletonAnimationInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactoryDyn for ImageArraySingletonAnimationInterfaceFactory {
    fn as_SingletonAnimationInterfaceFactory(&self) -> &crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory { &self.base }
    fn as_SingletonAnimationInterfaceFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::SingletonAnimationInterfaceFactory::SingletonAnimationInterfaceFactory { &mut self.base }
}


