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
use crate::org::allbinary::direction::DirectionUtil::DirectionUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::math::FrameUtil::FrameUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::IndexedAnimation::IndexedAnimation;
use super::RotationAnimationInterface::RotationAnimationInterface;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RotationAnimation {
    pub base: IndexedAnimation,
    pub frameUtil: Option<Rc<RefCell<FrameUtil>>>,
    pub directionUtil: Option<Rc<RefCell<DirectionUtil>>>,
    pub angleInfo: Option<Rc<RefCell<AngleInfo>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
}

// impl RotationAnimationInterface for RotationAnimation with the methods of the trait
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for RotationAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl RotationAnimation {
    

    pub fn new_struct(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            frameUtil: None,
            directionUtil: None,
            angleInfo: None,
            circularIndexUtil: None,
        };
        this.frameUtil = FrameUtil::getInstance();
        this.directionUtil = DirectionUtil::getInstance();
        // super(...) is in the struct literal above
        this.angleInfo = angleInfo.clone();
        this.circularIndexUtil = circularIndexUtil.clone();
        this
    }

    pub fn new(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angleInfo, circularIndexUtil, animationBehavior))))
    }
    

    pub fn nextRotationX(&mut self) {
    }
    

    pub fn previousRotationX(&mut self) {
    }
    

    pub fn nextRotation(&mut self) {
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (jcall!(self.circularIndexUtil).next(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn previousRotation(&mut self) {
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (jcall!(self.circularIndexUtil).previous(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn nextRotationZ(&mut self) {
    }
    

    pub fn previousRotationZ(&mut self) {
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.circularIndexUtil).setIndex(index.clone());
        let mut newFrame: i32 = jcall!(self.circularIndexUtil).getIndex();
        jcall!(self.angleInfo).adjustAngle(newFrame.clone());
    }
    

    pub fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getFrameAngle(direction.clone());
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn adjustFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (jcall!(angle).getValue(),); self.adjustFrame(java_arguments.0) };
    }
    

    pub fn adjustFrame(&mut self, mut angle: i16) {
        { let java_arguments = ({ let java_receiver = self.frameUtil.clone(); let java_arguments = (angle.clone(), ((jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getAngleIncrement()) as i32),); let java_result = jcall!(java_receiver).getFrameForAngle(java_arguments.0, java_arguments.1); java_result },); self.setFrame(java_arguments.0) };
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.circularIndexUtil).getIndex();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.circularIndexUtil).getSize();
    }
    

    pub fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> {
        return self.angleInfo.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(StringMaker::new()).append(self.base.toString()).append(String::from("circularIndexUtil: ")).append(jcall!(self.circularIndexUtil).toString()).append(String::from("angleInfo: ")).append(jcall!(self.angleInfo).toString()).toString();
    }
}
impl std::ops::Deref for RotationAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RotationAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_RotationAnimation(&self) -> &RotationAnimation;
    fn as_RotationAnimation_mut(&mut self) -> &mut RotationAnimation;
    fn nextRotationX(&mut self) { RotationAnimation::nextRotationX(self.as_RotationAnimation_mut()) }
    fn previousRotationX(&mut self) { RotationAnimation::previousRotationX(self.as_RotationAnimation_mut()) }
    fn nextRotation(&mut self) { RotationAnimation::nextRotation(self.as_RotationAnimation_mut()) }
    fn previousRotation(&mut self) { RotationAnimation::previousRotation(self.as_RotationAnimation_mut()) }
    fn nextRotationZ(&mut self) { RotationAnimation::nextRotationZ(self.as_RotationAnimation_mut()) }
    fn previousRotationZ(&mut self) { RotationAnimation::previousRotationZ(self.as_RotationAnimation_mut()) }
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { RotationAnimation::setFrameByDirection(self.as_RotationAnimation_mut(), direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { RotationAnimation::setFrameToAngle(self.as_RotationAnimation_mut(), angle) }
    fn adjustFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { RotationAnimation::adjustFrameToAngle(self.as_RotationAnimation_mut(), angle) }
    fn adjustFrame(&mut self, mut angle: i16) { RotationAnimation::adjustFrame(self.as_RotationAnimation_mut(), angle) }
    fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> { RotationAnimation::getAngleInfoP(self.as_RotationAnimation_mut()) }
}
impl RotationAnimationDyn for RotationAnimation {
    fn as_RotationAnimation(&self) -> &RotationAnimation { self }
    fn as_RotationAnimation_mut(&mut self) -> &mut RotationAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for RotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for RotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


