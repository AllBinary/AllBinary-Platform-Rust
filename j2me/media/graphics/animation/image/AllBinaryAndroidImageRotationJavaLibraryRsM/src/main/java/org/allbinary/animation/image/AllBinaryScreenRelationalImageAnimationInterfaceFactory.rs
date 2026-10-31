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
use crate::org::allbinary::image::GameFeatureImageCacheFactory::GameFeatureImageCacheFactory;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::graphics::displayable::ScreenRelationalUtil::ScreenRelationalUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::media::image::ImageScaleUtil::ImageScaleUtil;
use crate::org::microemu::android::device::AndroidImageInterface::AndroidImageInterface;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::ImageAnimation::ImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryScreenRelationalImageAnimationInterfaceFactory {
    pub base: BaseImageAnimationFactory,
    pub lastImage: Option<Rc<RefCell<Image>>>,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryScreenRelationalImageAnimationInterfaceFactory {
}

impl AllBinaryScreenRelationalImageAnimationInterfaceFactory {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), 0, 0, 0, 0, animationBehaviorFactory.clone()),
            lastImage: None,
        };
        this.lastImage = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        if !match (&this.lastImage, &crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE.with(Clone::clone)) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            let mut androidImageInterface: Option<Rc<RefCell<dyn AndroidImageInterface>>> = (this.lastImage).clone() /* as AndroidImageInterface */;
            jcall!(jcall!(androidImageInterface).getBitmap()).recycle();
        }
        let mut scale: f32 = jcall!(ScreenRelationalUtil::getInstance()).getScaleImage(image.clone());
        this.lastImage = jcall!(ImageScaleUtil::getInstance()).createImage(GameFeatureImageCacheFactory::getInstance(), this.getImage(), scale.clone(), scale.clone(), false);
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return ImageAnimation::new(self.lastImage.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
    }
}
impl std::ops::Deref for AllBinaryScreenRelationalImageAnimationInterfaceFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryScreenRelationalImageAnimationInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryScreenRelationalImageAnimationInterfaceFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


