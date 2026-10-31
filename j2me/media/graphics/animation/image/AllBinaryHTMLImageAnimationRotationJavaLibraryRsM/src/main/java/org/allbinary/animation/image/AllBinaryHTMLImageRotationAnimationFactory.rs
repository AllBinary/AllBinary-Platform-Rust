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
use crate::org::allbinary::game::configuration::GameConfigurationCentral::GameConfigurationCentral;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::AllBinaryHTMLImageRotationAnimation::AllBinaryHTMLImageRotationAnimation;
use super::AllBinaryAdjustedHTMLImageRotationAnimation::AllBinaryAdjustedHTMLImageRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryHTMLImageRotationAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub angleIncrement: i16,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryHTMLImageRotationAnimationFactory {
}

impl AllBinaryHTMLImageRotationAnimationFactory {
    

    // throws Exception
    pub fn createFactoryADXY(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<AllBinaryHTMLImageRotationAnimationFactory>>> {
        let mut htmlImageRotationAnimationFactory: Option<Rc<RefCell<AllBinaryHTMLImageRotationAnimationFactory>>> = AllBinaryHTMLImageRotationAnimationFactory::new(image.clone(), width.clone(), height.clone(), angleIncrement.clone(), animationBehaviorFactory.clone());
        jcall!(htmlImageRotationAnimationFactory).init(dx.clone(), dy.clone());
        return htmlImageRotationAnimationFactory.clone();
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone()),
            angleIncrement: 0,
        };
        // super(...) is in the struct literal above
        this.angleIncrement = angleIncrement.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, width, height, angleIncrement, animationBehaviorFactory))))
    }
    

    pub fn init(&mut self, mut dx: i32, mut dy: i32) {
        jcall!(self.animationFactoryInitializationVisitor).dx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).dy = dy.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDy = dy.clone();
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut scaledImage: Option<Rc<RefCell<Image>>> = { let java_receiver = self.animationFactoryImageScaleUtil.clone(); let java_arguments = (self.getImage(), jcall!(self.animationFactoryInitializationVisitor).width.clone(), jcall!(self.animationFactoryInitializationVisitor).height.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        let mut copyOfScaledImage: Option<Rc<RefCell<Image>>> = jcall!(ImageCopyUtil::getInstance()).createImage(scaledImage.clone());
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            jcall!(self.animationFactoryImageScaleUtil).processAdjust(None /* Java this: a &mut self method has no Rc of itself */);
            return AllBinaryAdjustedHTMLImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.getAngleIncrement()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return AllBinaryHTMLImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.angleIncrement.clone()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
    

    pub fn getAngleIncrement(&mut self) -> i16 {
        return self.angleIncrement.clone();
    }
}
impl std::ops::Deref for AllBinaryHTMLImageRotationAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryHTMLImageRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryHTMLImageRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


