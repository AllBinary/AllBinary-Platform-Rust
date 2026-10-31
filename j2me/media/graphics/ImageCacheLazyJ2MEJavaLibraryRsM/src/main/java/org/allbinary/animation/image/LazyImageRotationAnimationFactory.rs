// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
use crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactory;
use crate::org::allbinary::animation::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use crate::org::allbinary::image::ImageCacheFactory::ImageCacheFactory;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::LazyImageRotationAnimation::LazyImageRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LazyImageRotationAnimationFactory {
    pub layoutIndex: i32,
    pub animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>,
    pub scaleProperties: Option<Rc<RefCell<ScaleProperties>>>,
}

impl AnimationInterfaceFactoryInterface for LazyImageRotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for LazyImageRotationAnimationFactory {
}

impl LazyImageRotationAnimationFactory {
    

    pub fn new_struct(mut layoutIndex: i32, mut associatedLazyAnimationId: i32, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>) -> Self {
        let mut this = Self {
            layoutIndex: 0,
            animationInterfaceFactoryInterface: None,
            scaleProperties: None,
        };
        this.scaleProperties = crate::org::allbinary::media::ScaleProperties::ScaleProperties_instance.with(Clone::clone).clone();
        this.layoutIndex = layoutIndex.clone();
        this.animationInterfaceFactoryInterface = animationInterfaceFactoryInterface.clone();
        jcall!(ImageCacheFactory::getInstance()).hasAnyLazyAnimationFactories = true;
        if match (&jcall!(this.animationInterfaceFactoryInterface).as_BaseImageAnimationFactory_mut().animationBehaviorFactory, &AnimationBehaviorFactory::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            ForcedLogUtil::log(String::from("Using default AnimationBehaviorFactory with IndexedAnimationFactory"), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        }
        this
    }

    pub fn new(mut layoutIndex: i32, mut associatedLazyAnimationId: i32, mut animationInterfaceFactoryInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(layoutIndex, associatedLazyAnimationId, animationInterfaceFactoryInterface))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        if jcall!(jcall!(self.animationInterfaceFactoryInterface).getImage()).isReady() {
            jcall!(self.animationInterfaceFactoryInterface).setInitialScale(self.scaleProperties.clone());
            return jcall!(self.animationInterfaceFactoryInterface).getInstance(instanceId.clone());
        } else {
            return LazyImageRotationAnimation::new(self.layoutIndex.clone(), instanceId.clone(), self.scaleProperties.clone(), self.animationInterfaceFactoryInterface.clone(), jcall!(jcall!(self.animationInterfaceFactoryInterface).as_BaseImageAnimationFactory_mut().animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
        self.scaleProperties = scaleProperties.clone();
    }
}

