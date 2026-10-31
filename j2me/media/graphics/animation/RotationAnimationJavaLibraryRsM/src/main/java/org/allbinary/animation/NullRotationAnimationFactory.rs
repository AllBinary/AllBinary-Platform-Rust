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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::RotationAnimation::RotationAnimation;
use super::NullRotationAnimation::NullRotationAnimation;
use super::IndexedAnimationBehavior::IndexedAnimationBehavior;
use super::NullRotationAnimation2::NullRotationAnimation2;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRotationAnimationFactory {
    pub NULL_ROTATION_ANIMATION_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>,
    pub NULL_ANIMATION: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

pub static mut NullRotationAnimationFactory_NULL_ROTATION_ANIMATION_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<NullRotationAnimationFactory>>>> = std::cell::LazyCell::new(|| NullRotationAnimationFactory::new());
impl AnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullRotationAnimationFactory {
}

impl NullRotationAnimationFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_ROTATION_ANIMATION_ARRAY: Vec::new(),
            NULL_ANIMATION: None,
        };
        this.NULL_ROTATION_ANIMATION_ARRAY = vec![None; (0) as usize];
        this.NULL_ANIMATION = NullRotationAnimation2::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFactoryInstance() -> Option<Rc<RefCell<NullRotationAnimationFactory>>> {
        return unsafe { NullRotationAnimationFactory_NULL_ROTATION_ANIMATION_FACTORY.clone() }.clone();
    }
    

    pub fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    // throws Exception
    pub fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> {
        return self.NULL_ANIMATION.clone();
    }
    

    pub fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) {
    }
}

pub struct NullRotationAnimation2 {
    pub base: NullRotationAnimation,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for NullRotationAnimation2 {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for NullRotationAnimation2 {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl NullRotationAnimation2 {
        

        pub fn new_struct() -> Self {
            let mut this = Self {
            base: NullRotationAnimation::new_struct(AngleInfo::getInstance(jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone()), CircularIndexUtil::createInstance(4), IndexedAnimationBehavior::new(1, 250)),
            };
            // super(...) is in the struct literal above
            this
        }

        pub fn new() -> Option<Rc<RefCell<Self>>> {
            Some(Rc::new(RefCell::new(Self::new_struct())))
        }
        

        pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        }
}
impl std::ops::Deref for NullRotationAnimation2 {
    type Target = NullRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullRotationAnimation2 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::NullRotationAnimation::NullRotationAnimationDyn for NullRotationAnimation2 {
    fn as_NullRotationAnimation(&self) -> &crate::org::allbinary::animation::NullRotationAnimation::NullRotationAnimation { &self }
    fn as_NullRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::NullRotationAnimation::NullRotationAnimation { &mut self }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for NullRotationAnimation2 {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for NullRotationAnimation2 {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for NullRotationAnimation2 {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::NullRotationAnimation::NullRotationAnimation::getAnimationSize(crate::org::allbinary::animation::NullRotationAnimation::NullRotationAnimationDyn::as_NullRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for NullRotationAnimation2 {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


