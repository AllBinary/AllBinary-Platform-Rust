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
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use super::VectorAnimation::VectorAnimation;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorLessVectorAnimation {
    pub base: VectorAnimation,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ColorLessVectorAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ColorLessVectorAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ColorLessVectorAnimation {
    

    pub fn new_struct(mut currentPoints: Vec<Vec<Vec<i32>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: VectorAnimation::new_struct(currentPoints.clone(), jcall!(BasicColorFactory::getInstance()).WHITE.clone(), animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut currentPoints: Vec<Vec<Vec<i32>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(currentPoints, animationBehavior))))
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.paintVectors(graphics.clone(), x.clone(), y.clone());
    }
}
impl std::ops::Deref for ColorLessVectorAnimation {
    type Target = VectorAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ColorLessVectorAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for ColorLessVectorAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn setAlpha(&mut self, p1: i32) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setAlpha(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1) }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::nextFrame(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setBasicColorP(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1) }
    fn changeBasicColor(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::changeBasicColor(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1) }
    fn setScale(&mut self, p1: f32, p2: f32) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setScale(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1, p2) }
    fn setMaxScale(&mut self, p1: f32, p2: f32) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setMaxScale(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1, p2) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ColorLessVectorAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setFrame(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::getFrame(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::getAnimationSize(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::getSize(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::previousFrame(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::setSequence(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::VectorAnimation::VectorAnimation::getSequence(crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn::as_VectorAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::VectorAnimation::VectorAnimationDyn for ColorLessVectorAnimation {
    fn as_VectorAnimation(&self) -> &crate::org::allbinary::animation::VectorAnimation::VectorAnimation { &self.base }
    fn as_VectorAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::VectorAnimation::VectorAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ColorLessVectorAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


