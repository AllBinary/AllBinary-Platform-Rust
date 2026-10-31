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
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::VectorBaseRotationAnimation::VectorBaseRotationAnimation;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorRotationAnimation {
    pub base: VectorBaseRotationAnimation,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for VectorRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for VectorRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for VectorRotationAnimation {
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, newAngle) }
    fn adjustFrame(&mut self, mut newAngle: i16) { Self::adjustFrame(self, newAngle) }
}

impl VectorRotationAnimation {
    

    pub fn new_struct(mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: VectorBaseRotationAnimation::new_struct(AngleInfo::getInstance((((((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32) / (currentPoints.len() as i32))) as i16)), currentPoints.clone(), basicColor.clone(), animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        { let java_receiver = this.angleInfo.clone(); let java_arguments = (this.getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
        this
    }

    pub fn new(mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(currentPoints, basicColor, animationBehavior))))
    }
    

    pub fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getFrameAngle(direction.clone());
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        self.base.setFrame(index.clone());
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (self.getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn adjustFrame(&mut self, mut newAngle: i16) {
        { let java_arguments = ({ let java_receiver = self.frameUtil.clone(); let java_arguments = (newAngle.clone(), ((jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getAngleIncrement()) as i32),); let java_result = jcall!(java_receiver).getFrameForAngle(java_arguments.0, java_arguments.1); java_result },); self.setFrame(java_arguments.0) };
    }
    

    pub fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (jcall!(newAngle).getValue(),); self.adjustFrame(java_arguments.0) };
    }
}
impl std::ops::Deref for VectorRotationAnimation {
    type Target = VectorBaseRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VectorRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for VectorRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::nextFrame(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for VectorRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::getFrame(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::getAnimationSize(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::getSize(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::previousFrame(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::setSequence(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation::getSequence(crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn::as_VectorBaseRotationAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for VectorRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, newAngle) }
    fn adjustFrame(&mut self, mut newAngle: i16) { Self::adjustFrame(self, newAngle) }
}
impl crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimationDyn for VectorRotationAnimation {
    fn as_VectorBaseRotationAnimation(&self) -> &crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation { &self.base }
    fn as_VectorBaseRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::VectorBaseRotationAnimation::VectorBaseRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for VectorRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


