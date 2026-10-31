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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::special::SpecialAnimationInterface::SpecialAnimationInterface;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::ImageArrayAnimation::ImageArrayAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArraySpecialAnimation {
    pub base: ImageArrayAnimation,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub loopCount: i32,
    pub loopCountTotal: i32,
}

// impl SpecialAnimationInterface for ImageArraySpecialAnimation with the methods of the trait
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for ImageArraySpecialAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ImageArraySpecialAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl ImageArraySpecialAnimation {
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: ImageArrayAnimation::new_struct(imageArray.clone(), animationBehavior.clone()),
            timeDelayHelper: None,
            loopCount: 0,
            loopCountTotal: 0,
        };
        this.timeDelayHelper = TimeDelayHelper::new(270).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.loopCount = 0;
        this.loopCountTotal = 1;
        // super(...) is in the struct literal above
        this.reset();
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray, animationBehavior))))
    }
    

    pub fn getLoopCount(&mut self) -> i32 {
        return self.loopCount.clone();
    }
    

    pub fn setLastFrame(&mut self) {
        { let java_receiver = self.circularIndexUtil.clone(); let java_arguments = (jcall!(self.circularIndexUtil).getSize() - 1,); jcall!(java_receiver).setIndex(java_arguments.0); };
    }
    

    pub fn reset(&mut self) {
        { let java_arguments = (self.getSize() - 1,); self.setFrame(java_arguments.0) };
        self.loopCount = 0;
    }
    

    pub fn nextFrame(&mut self) {
        if self.getFrame() > 0 {
            if { let java_receiver = self.timeDelayHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
                self.previousFrame();
                if self.getFrame() == 0 {
                    self.loopCount += 1;
                }
            }
        }
    }
    

    pub fn isComplete(&mut self) -> bool {
        if self.loopCount < self.loopCountTotal || self.getFrame() != 0 {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        x = jcall!(DisplayInfoSingleton::getInstance()).getLastHalfWidth() - (jcall!({ let java_arguments = (jcall!(self.circularIndexUtil).getIndex(),); self.getImage(java_arguments.0) }).getWidth() >> 1);
        y = 5;
        self.base.paintXY(graphics.clone(), x.clone(), y.clone());
    }
}
impl std::ops::Deref for ImageArraySpecialAnimation {
    type Target = ImageArrayAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageArraySpecialAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn for ImageArraySpecialAnimation {
    fn as_ImageArrayAnimation(&self) -> &crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation { &self.base }
    fn as_ImageArrayAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for ImageArraySpecialAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for ImageArraySpecialAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn reset(&mut self) { Self::reset(self) }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::setFrame(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::getFrame(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::getAnimationSize(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::getSize(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::previousFrame(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::setSequence(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimation::getSequence(crate::org::allbinary::animation::image::ImageArrayAnimation::ImageArrayAnimationDyn::as_ImageArrayAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ImageArraySpecialAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


