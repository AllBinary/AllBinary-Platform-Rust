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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::media::image::ImageToRotationImageArrayUtil::ImageToRotationImageArrayUtil;
use super::ImageArrayRotationAnimationInfo::ImageArrayRotationAnimationInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//TWB - Adjustments should be done in the resource creation and not at the animation level
pub struct PooledImageArrayRotationAnimationFactory {
    pub allBinaryImageRotationAnimationInfo: Option<Rc<RefCell<Box<dyn std::any::Any>>>>,
    pub animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>,
}

impl AnimationInterfaceFactoryInterface for PooledImageArrayRotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for PooledImageArrayRotationAnimationFactory {
}

impl PooledImageArrayRotationAnimationFactory {
    

    // throws Exception
    pub fn createFactory(mut image: Option<Rc<RefCell<Image>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<PooledImageArrayRotationAnimationFactory>>> {
        let mut totalAngle: i32 = ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32);
        let mut angleIncrement: i32 = (totalAngle / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity());
        return PooledImageArrayRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), -(jcall!(image).getWidth() >> 2), -(jcall!(image).getHeight() >> 2), angleIncrement.clone(), animationBehaviorFactory.clone());
    }
    

    // throws Exception
    pub fn createDXY(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<PooledImageArrayRotationAnimationFactory>>> {
        let mut totalAngle: i32 = ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32);
        let mut angleIncrement: i32 = (totalAngle / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity());
        return PooledImageArrayRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), dx.clone(), dy.clone(), angleIncrement.clone(), animationBehaviorFactory.clone());
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut angleIncrement: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            allBinaryImageRotationAnimationInfo: None,
            animationBehaviorFactory: None,
        };
        this.allBinaryImageRotationAnimationInfo = jcall!(NullUtil::getInstance()).NULL_OBJECT.clone();
        this.animationBehaviorFactory = animationBehaviorFactory.clone();
        let mut totalAngle: i32 = ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32);
        let mut imageArray: Vec<Option<Rc<RefCell<Image>>>> = jcall!(ImageToRotationImageArrayUtil::getInstance()).generate(image.clone(), angleIncrement.clone(), totalAngle.clone());
        this.allBinaryImageRotationAnimationInfo = ImageArrayRotationAnimationInfo::new(imageArray.clone(), angleIncrement.clone(), totalAngle.clone(), dx.clone(), dy.clone());
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut angleIncrement: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, width, height, dx, dy, angleIncrement, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return AdjustedImageArrayRotationAnimation::AdjustedImageArrayRotationAnimation::createAdjustedImageArrayRotationAnimationInfo(self.allBinaryImageRotationAnimationInfo.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance());
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}

