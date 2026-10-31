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
use crate::org::allbinary::animation::VectorAnimation::VectorAnimation;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::image::AnimationFrameToImageUtil::AnimationFrameToImageUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::media::image::ImageToRotationImageArrayUtil::ImageToRotationImageArrayUtil;
use crate::org::allbinary::vector::VectorInfo::VectorInfo;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorToImageArrayRotationAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub angleIncrement: i32,
}

impl crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface for VectorToImageArrayRotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for VectorToImageArrayRotationAnimationFactory {
}

impl VectorToImageArrayRotationAnimationFactory {
    

    pub fn new_struct(mut vectorInfo: Option<Rc<RefCell<VectorInfo>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(jcall!(AnimationFrameToImageUtil::getInstance()).getInstanceTranslate(jcall!(vectorInfo).getWidth(), jcall!(vectorInfo).getHeight(), VectorAnimation::createVectorAnimation(jcall!(vectorInfo).getPoints(), basicColor.clone(), jcall!(animationBehaviorFactory).getOrCreateInstance())), PrimitiveIntUtil::getArrayInstance(), 0, 0, 0, 0, animationBehaviorFactory.clone()),
            imageArray: Vec::new(),
            angleIncrement: 0,
        };
        this.imageArray = crate::javax::microedition::lcdui::NullImage::NullImage_NULL_IMAGE_ARRAY.with(Clone::clone).clone();
        // super(...) is in the struct literal above
        this.init();
        this
    }

    pub fn new(mut vectorInfo: Option<Rc<RefCell<VectorInfo>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(vectorInfo, basicColor, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn init(&mut self) {
        let mut angleFactory: Option<Rc<RefCell<AngleFactory>>> = AngleFactory::getInstance();
        self.angleIncrement = ((jcall!(angleFactory).TOTAL_ANGLE) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity();
        self.imageArray = jcall!(ImageToRotationImageArrayUtil::getInstance()).generate(self.getImage(), self.getAngleIncrement(), ((jcall!(angleFactory).TOTAL_ANGLE) as i32));
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            return AdjustedImageArrayRotationAnimation::new(self.imageArray.clone(), AngleInfo::getInstance(((self.getAngleIncrement()) as i16)), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return AdjustedImageArrayRotationAnimation::createAnimation(self.imageArray.clone(), AngleInfo::getInstance(((self.angleIncrement) as i16)), ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32), jcall!(self.animationBehaviorFactory).getOrCreateInstance());
        }
    }
    

    pub fn getAngleIncrement(&mut self) -> i32 {
        return self.angleIncrement.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}
impl std::ops::Deref for VectorToImageArrayRotationAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VectorToImageArrayRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for VectorToImageArrayRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}


