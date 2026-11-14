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
use crate::javax::microedition::lcdui::Graphics::Graphics;
use super::NullIndexedAnimation::NullIndexedAnimation;
use super::IndexedAnimationBehavior::IndexedAnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullIndexedAnimationInner {
    pub base: NullIndexedAnimation,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for NullIndexedAnimationInner {
    fn nextFrame(&mut self) { crate::org::allbinary::animation::Animation::Animation::nextFrame(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self)) }
    fn paintThreedXYZ(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, p2: i32, p3: i32, p4: i32) { crate::org::allbinary::animation::Animation::Animation::paintThreedXYZ(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1, p2, p3, p4) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for NullIndexedAnimationInner {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl NullIndexedAnimationInner {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: NullIndexedAnimation::new_struct(IndexedAnimationBehavior::new(1, 250).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>)),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) {
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
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<dyn crate::javax::microedition::lcdui::Graphics::GraphicsDyn>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for NullIndexedAnimationInner {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation::getAnimationSize(crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimationDyn::as_NullIndexedAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimationDyn for NullIndexedAnimationInner {
    fn as_NullIndexedAnimation(&self) -> &crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation { &self.base }
    fn as_NullIndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::NullIndexedAnimation::NullIndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for NullIndexedAnimationInner {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


