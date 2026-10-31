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
use crate::org::allbinary::game::configuration::feature::Features::Features;
use crate::org::allbinary::graphics::opengles::OpenGLFeatureFactory::OpenGLFeatureFactory;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::media::image::ImageCopyUtil::ImageCopyUtil;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::AllBinaryFlickerAndroidImageRotationAnimation::AllBinaryFlickerAndroidImageRotationAnimation;
use super::AllBinaryAdjustedAndroidImageRotationAnimation::AllBinaryAdjustedAndroidImageRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryAndroidImageRotationAnimationFactory {
    pub base: BaseImageAnimationFactory,
    pub angleIncrement: i16,
    pub resizeCanvasForRotation: bool,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for AllBinaryAndroidImageRotationAnimationFactory {
}

impl AllBinaryAndroidImageRotationAnimationFactory {
    

    // throws Exception
    pub fn createFactory(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> {
        return AllBinaryAndroidImageRotationAnimationFactory::new(image.clone(), width.clone(), height.clone(), (((((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity())) as i16), animationBehaviorFactory.clone(), false).map(|reference| reference as Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>);
    }
    

    // throws Exception
    pub fn createFactoryA(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> {
        return AllBinaryAndroidImageRotationAnimationFactory::new(image.clone(), width.clone(), height.clone(), angleIncrement.clone(), animationBehaviorFactory.clone(), false).map(|reference| reference as Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>);
    }
    

    // throws Exception
    pub fn createFactoryU(mut image: Option<Rc<RefCell<Image>>>, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> {
        let mut androidImageRotationAnimationFactory: Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> = AllBinaryAndroidImageRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), (((((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity())) as i16), animationBehaviorFactory.clone(), false).map(|reference| reference as Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>);
        jcall!(androidImageRotationAnimationFactory).init(-(jcall!(image).getWidth() >> 2), -(jcall!(image).getHeight() >> 2));
        return androidImageRotationAnimationFactory.clone();
    }
    

    // throws Exception
    pub fn createFactoryDXYU(mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> {
        let mut androidImageRotationAnimationFactory: Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> = AllBinaryAndroidImageRotationAnimationFactory::new(image.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), (((((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32) / jcall!(GameConfigurationCentral::getInstance()).getGameControlFidelity())) as i16), animationBehaviorFactory.clone(), false).map(|reference| reference as Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>);
        jcall!(androidImageRotationAnimationFactory).init(dx.clone(), dy.clone());
        return androidImageRotationAnimationFactory.clone();
    }
    

    // throws Exception
    pub fn createFactoryADXY(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut dx: i32, mut dy: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> {
        let mut androidImageRotationAnimationFactory: Option<Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>> = AllBinaryAndroidImageRotationAnimationFactory::new(image.clone(), width.clone(), height.clone(), angleIncrement.clone(), animationBehaviorFactory.clone(), resizeCanvasForRotation.clone()).map(|reference| reference as Rc<RefCell<dyn AllBinaryAndroidImageRotationAnimationFactoryDyn>>);
        jcall!(androidImageRotationAnimationFactory).init(dx.clone(), dy.clone());
        return androidImageRotationAnimationFactory.clone();
    }
    

    pub fn init(&mut self, mut dx: i32, mut dy: i32) {
        jcall!(self.animationFactoryInitializationVisitor).dx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).dy = dy.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDx = dx.clone();
        jcall!(self.animationFactoryInitializationVisitor).originalDy = dy.clone();
    }
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut width: i32, mut height: i32, mut angleIncrement: i16, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>, mut resizeCanvasForRotation: bool) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), PrimitiveIntUtil::getArrayInstance(), width.clone(), height.clone(), 0, 0, animationBehaviorFactory.clone()),
            angleIncrement: 0,
            resizeCanvasForRotation: false,
        };
        // super(...) is in the struct literal above
        this.angleIncrement = angleIncrement.clone();
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
        let mut canvasImage: Option<Rc<RefCell<Image>>> = self.getCanvasImage();
        let mut scaledImage: Option<Rc<RefCell<Image>>> = jcall!(self.animationFactoryImageScaleUtil).createImage(canvasImage.clone(), jcall!(self.animationFactoryInitializationVisitor).width.clone(), jcall!(self.animationFactoryInitializationVisitor).height.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone());
        let mut copyOfScaledImage: Option<Rc<RefCell<Image>>> = jcall!(ImageCopyUtil::getInstance()).createImage(scaledImage.clone());
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            jcall!(self.animationFactoryImageScaleUtil).processAdjust(None /* Java this: a &mut self method has no Rc of itself */);
            return AllBinaryAdjustedAndroidImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.angleIncrement.clone()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.base.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return AllBinaryFlickerAndroidImageRotationAnimation::new(scaledImage.clone(), copyOfScaledImage.clone(), AngleInfo::getInstance(self.angleIncrement.clone()), jcall!(AngleFactory::getInstance()).TOTAL_ANGLE.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
}
impl std::ops::Deref for AllBinaryAndroidImageRotationAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryAndroidImageRotationAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinaryAndroidImageRotationAnimationFactoryDyn: crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn {
    fn as_AllBinaryAndroidImageRotationAnimationFactory(&self) -> &AllBinaryAndroidImageRotationAnimationFactory;
    fn as_AllBinaryAndroidImageRotationAnimationFactory_mut(&mut self) -> &mut AllBinaryAndroidImageRotationAnimationFactory;
    fn init(&mut self, mut dx: i32, mut dy: i32) { AllBinaryAndroidImageRotationAnimationFactory::init(self.as_AllBinaryAndroidImageRotationAnimationFactory_mut(), dx, dy) }
    fn getCanvasImage(&mut self) -> Option<Rc<RefCell<Image>>> { AllBinaryAndroidImageRotationAnimationFactory::getCanvasImage(self.as_AllBinaryAndroidImageRotationAnimationFactory_mut()) }
}
impl AllBinaryAndroidImageRotationAnimationFactoryDyn for AllBinaryAndroidImageRotationAnimationFactory {
    fn as_AllBinaryAndroidImageRotationAnimationFactory(&self) -> &AllBinaryAndroidImageRotationAnimationFactory { self }
    fn as_AllBinaryAndroidImageRotationAnimationFactory_mut(&mut self) -> &mut AllBinaryAndroidImageRotationAnimationFactory { self }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for AllBinaryAndroidImageRotationAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


