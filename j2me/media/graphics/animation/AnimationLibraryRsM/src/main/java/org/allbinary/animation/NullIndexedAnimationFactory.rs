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
use crate::org::allbinary::media::ScaleProperties::ScaleProperties;
use super::AnimationInterfaceFactoryInterface::AnimationInterfaceFactoryInterface;
use super::ProceduralAnimationInterfaceFactoryInterface::ProceduralAnimationInterfaceFactoryInterface;
use super::IndexedAnimation::IndexedAnimation;
use super::NullIndexedAnimation::NullIndexedAnimation;
use super::IndexedAnimationBehavior::IndexedAnimationBehavior;
use super::NullIndexedAnimationInner::NullIndexedAnimationInner;
use super::Animation::Animation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullIndexedAnimationFactory {
    pub NULL_ANIMATION: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

thread_local! { pub static NullIndexedAnimationFactory_NULL_INDEXED_ANIMATION_ARRAY: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>> = vec![None; (0) as usize]; }
pub static mut NullIndexedAnimationFactory_NULL_INDEXED_ANIMATION_FACTORY: std::cell::LazyCell<Option<Rc<RefCell<NullIndexedAnimationFactory>>>> = std::cell::LazyCell::new(|| NullIndexedAnimationFactory::new());
impl AnimationInterfaceFactoryInterface for NullIndexedAnimationFactory {
    fn getInstance(&mut self, mut instanceId: i32) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstance(self, instanceId) }
    fn setInitialScale(&mut self, mut scaleProperties: Option<Rc<RefCell<ScaleProperties>>>) { Self::setInitialScale(self, scaleProperties) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullIndexedAnimationFactory {
}
impl ProceduralAnimationInterfaceFactoryInterface for NullIndexedAnimationFactory {
    fn getInstanceAnimation(&mut self, mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>> { Self::getInstanceAnimation(self, animationInterface) }
}
impl crate::org::allbinary::animation::BasicAnimationInterfaceFactoryInterface::BasicAnimationInterfaceFactoryInterface for NullIndexedAnimationFactory {
}

impl NullIndexedAnimationFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_ANIMATION: None,
        };
        this.NULL_ANIMATION = NullIndexedAnimationInner::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFactoryInstance() -> Option<Rc<RefCell<NullIndexedAnimationFactory>>> {
        return unsafe { NullIndexedAnimationFactory_NULL_INDEXED_ANIMATION_FACTORY.clone() }.clone();
    }
    

    // throws Exception
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

pub struct NullIndexedAnimationInner {
    pub base: NullIndexedAnimation,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for NullIndexedAnimationInner {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for NullIndexedAnimationInner {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl NullIndexedAnimationInner {
        

        pub fn new_struct() -> Self {
            let mut this = Self {
            base: NullIndexedAnimation::new_struct(IndexedAnimationBehavior::new(1, 250)),
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
impl std::ops::Deref for NullIndexedAnimationInner {
    type Target = NullIndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullIndexedAnimationInner {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for NullIndexedAnimationInner {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for NullIndexedAnimationInner {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation::getAnimationSize(crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimationDyn::as_NullIndexedAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimationDyn for NullIndexedAnimationInner {
    fn as_NullIndexedAnimation(&self) -> &crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation { &self }
    fn as_NullIndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation { &mut self }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for NullIndexedAnimationInner {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


