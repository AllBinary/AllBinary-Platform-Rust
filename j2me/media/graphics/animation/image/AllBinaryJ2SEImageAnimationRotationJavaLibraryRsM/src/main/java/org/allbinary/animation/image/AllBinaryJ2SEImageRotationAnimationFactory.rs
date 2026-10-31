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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::graphics::opengles::OpenGLUtil::OpenGLUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::AllBinaryJ2SEImageRotationAnimation::AllBinaryJ2SEImageRotationAnimation;
use super::AllBinaryAdjustedJ2SEImageRotationAnimation::AllBinaryAdjustedJ2SEImageRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryJ2SEImageRotationAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub angleIncrementP: i16,
    pub resizeCanvasForRotation: bool,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryJ2SEImageRotationAnimationFactory {
}

impl AllBinaryJ2SEImageRotationAnimationFactory {
    

    // throws Exception
    pub fn createDXY(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Option<Rc<RefCell<dyn AllBinaryJ2SEImageRotationAnimationFactoryDyn>>> {
        let mut imageRotationAnimationFactory: Option<Rc<RefCell<dyn AllBinaryJ2SEImageRotationAnimationFactoryDyn>>> = AllBinaryJ2SEImageRotationAnimationFactory::new(image.clone(), width.clone(), height.clone(), angleIncrement.clone(), animationBehaviorFactory.clone(), resizeCanvasForRotation.clone()).map(|reference| reference as Rc<RefCell<dyn AllBinaryJ2SEImageRotationAnimationFactoryDyn>>);
        jcall!(jcall!(imageRotationAnimationFactory).as_AllBinaryJ2SEImageRotationAnimationFactory_mut().animationFactoryInitializationVisitor).dx = dx.clone();
        jcall!(jcall!(imageRotationAnimationFactory).as_AllBinaryJ2SEImageRotationAnimationFactory_mut().animationFactoryInitializationVisitor).dy = dy.clone();
        jcall!(jcall!(imageRotationAnimationFactory).as_AllBinaryJ2SEImageRotationAnimationFactory_mut().animationFactoryInitializationVisitor).originalDx = dx.clone();
        jcall!(jcall!(imageRotationAnimationFactory).as_AllBinaryJ2SEImageRotationAnimationFactory_mut().animationFactoryInitializationVisitor).originalDy = dy.clone();
        return imageRotationAnimationFactory.clone();
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone()),
            angleIncrementP: 0,
            resizeCanvasForRotation: false,
        };
        // super(...) is in the struct literal above
        this.angleIncrementP = angleIncrement.clone();
        this.resizeCanvasForRotation = resizeCanvasForRotation.clone();
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, width, height, angleIncrement, animationBehaviorFactory, resizeCanvasForRotation))))
    }
    

    // throws Exception
    pub fn getCanvasImage(&mut self) -> Option<Rc<RefCell<Image>>> {
        let mut features: Option<Rc<RefCell<Features>>> = Features::getInstance();
        if self.resizeCanvasForRotation && !jcall!(features).isDefault(jcall!(OpenGLFeatureFactory::getInstance()).OPENGL.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::game::configuration::feature::Feature::FeatureDyn>>)) {
            return jcall!(ImageCopyUtil::getInstance()).createImageScale(self.getImage(), 1.44_f32, false);
        } else {
            return self.getImage();
        }
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut imageCopyUtil: Option<Rc<RefCell<ImageCopyUtil>>> = ImageCopyUtil::getInstance();
        let mut canvasImage: Option<Rc<RefCell<Image>>> = self.getCanvasImage();
        let mut scaledImage: Option<Rc<RefCell<Image>>> = jcall!(self.animationFactoryImageScaleUtil).createImage(canvasImage.clone(), jcall!(self.animationFactoryInitializationVisitor).width.clone(), jcall!(self.animationFactoryInitializationVisitor).height.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone());
        let mut openGLUtil: Option<Rc<RefCell<OpenGLUtil>>> = OpenGLUtil::getInstance();
        scaledImage = jcall!(openGLUtil).addImage(scaledImage.clone());
        let mut copyOfScaledImage: Option<Rc<RefCell<Image>>> = jcall!(imageCopyUtil).createImageForRotation(scaledImage.clone());
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            jcall!(self.animationFactoryImageScaleUtil).processAdjust(None /* Java this: a &mut self method has no Rc of itself */);
            return AllBinaryAdjustedJ2SEImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.angleIncrementP.clone()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return AllBinaryJ2SEImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.angleIncrementP.clone()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
    

    pub fn getAngleIncrement(&mut self) -> i16 {
        return self.angleIncrementP.clone();
    }
}
impl std::ops::Deref for AllBinaryJ2SEImageRotationAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryJ2SEImageRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryJ2SEImageRotationAnimationFactoryDyn: crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn {
    fn as_AllBinaryJ2SEImageRotationAnimationFactory(&self) -> &AllBinaryJ2SEImageRotationAnimationFactory;
    fn as_AllBinaryJ2SEImageRotationAnimationFactory_mut(&mut self) -> &mut AllBinaryJ2SEImageRotationAnimationFactory;
    fn getCanvasImage(&mut self) -> Option<Rc<RefCell<Image>>> { AllBinaryJ2SEImageRotationAnimationFactory::getCanvasImage(self.as_AllBinaryJ2SEImageRotationAnimationFactory_mut()) }
    fn getAngleIncrement(&mut self) -> i16 { AllBinaryJ2SEImageRotationAnimationFactory::getAngleIncrement(self.as_AllBinaryJ2SEImageRotationAnimationFactory_mut()) }
}
impl AllBinaryJ2SEImageRotationAnimationFactoryDyn for AllBinaryJ2SEImageRotationAnimationFactory {
    fn as_AllBinaryJ2SEImageRotationAnimationFactory(&self) -> &AllBinaryJ2SEImageRotationAnimationFactory { self }
    fn as_AllBinaryJ2SEImageRotationAnimationFactory_mut(&mut self) -> &mut AllBinaryJ2SEImageRotationAnimationFactory { self }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryJ2SEImageRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


