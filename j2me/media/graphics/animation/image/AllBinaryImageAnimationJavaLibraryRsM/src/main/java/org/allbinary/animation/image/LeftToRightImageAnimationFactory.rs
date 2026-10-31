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
use super::BaseImageAnimationFactory::BaseImageAnimationFactory;
use super::LeftToRightImageAnimation::LeftToRightImageAnimation;
use super::AdjustedLeftToRightImageAnimation::AdjustedLeftToRightImageAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LeftToRightImageAnimationFactory {
    pub base: BaseImageAnimationFactory,
}

impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for LeftToRightImageAnimationFactory {
}

impl LeftToRightImageAnimationFactory {
    

    pub fn new_struct(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Self {
        let mut this = Self {
            base: BaseImageAnimationFactory::new_struct(image.clone(), sequenceArray.clone(), jcall!(image).getWidth(), jcall!(image).getHeight(), dx.clone(), dy.clone(), animationBehaviorFactory.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut image: Option<Rc<RefCell<Image>>>, mut sequenceArray: Vec<i32>, mut dx: i32, mut dy: i32, mut animationBehaviorFactory: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehaviorFactory::AnimationBehaviorFactoryDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(image, sequenceArray, dx, dy, animationBehaviorFactory))))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        let mut scaledImage: Option<Rc<RefCell<Image>>> = { let java_receiver = self.animationFactoryImageScaleUtil.clone(); let java_arguments = (self.getImage(), jcall!(self.animationFactoryInitializationVisitor).width.clone(), jcall!(self.animationFactoryInitializationVisitor).height.clone(), jcall!(self.scaleProperties).scaleWidth.clone(), jcall!(self.scaleProperties).scaleHeight.clone(),); let java_result = jcall!(java_receiver).createImage(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3, java_arguments.4); java_result };
        if jcall!(self.animationFactoryInitializationVisitor).dx != 0 || jcall!(self.animationFactoryInitializationVisitor).dy != 0 {
            jcall!(self.animationFactoryImageScaleUtil).processAdjust(None /* Java this: a &mut self method has no Rc of itself */);
            return AdjustedLeftToRightImageAnimation::new(scaledImage.clone(), self.getSequenceArray(), jcall!(self.animationFactoryInitializationVisitor).dx.clone(), jcall!(self.animationFactoryInitializationVisitor).dy.clone(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        } else {
            return LeftToRightImageAnimation::new(scaledImage.clone(), self.getSequenceArray(), jcall!(self.animationBehaviorFactory).getOrCreateInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        }
    }
}
impl std::ops::Deref for LeftToRightImageAnimationFactory {
    type Target = BaseImageAnimationFactory;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LeftToRightImageAnimationFactory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactoryDyn for LeftToRightImageAnimationFactory {
    fn as_BaseImageAnimationFactory(&self) -> &crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &self.base }
    fn as_BaseImageAnimationFactory_mut(&mut self) -> &mut crate::org::allbinary::animation::image::BaseImageAnimationFactory::BaseImageAnimationFactory { &mut self.base }
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
}


