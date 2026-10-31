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
// package org.allbinary.animation.image.sprite
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use super::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinarySpriteRotationAnimation {
    pub base: HackRotationSpriteIndexedAnimation,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for AllBinarySpriteRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { self.base.getFrame() }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinarySpriteRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for AllBinarySpriteRotationAnimation {
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, angle) }
    fn adjustFrame(&mut self, mut angle: i16) { Self::adjustFrame(self, angle) }
}

impl AllBinarySpriteRotationAnimation {
    

    pub fn new_struct(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: HackRotationSpriteIndexedAnimation::new_struct(sprite.clone(), image.clone(), AngleInfo::getInstance((((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE / jcall!(sprite).getRawFrameCount())) as i16)), animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        { let java_receiver = this.angleInfo.clone(); let java_arguments = (jcall!(this.sprite).getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
        this
    }

    pub fn new(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sprite, image, animationBehavior))))
    }
    

    pub fn nextRotation(&mut self) {
        jcall!(self.sprite).nextFrame();
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (jcall!(self.sprite).getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn previousRotation(&mut self) {
        jcall!(self.sprite).prevFrame();
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (jcall!(self.sprite).getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) {
        let mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>> = jcall!(self.directionUtil).getFrameAngle(direction.clone());
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        self.adjustFrameToAngle(angle.clone());
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.sprite).setFrame(index.clone());
        { let java_receiver = self.angleInfo.clone(); let java_arguments = (self.getFrame(),); jcall!(java_receiver).adjustAngle(java_arguments.0); };
    }
    

    pub fn adjustFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (jcall!(angle).getValue(),); self.adjustFrame(java_arguments.0) };
    }
    

    pub fn adjustFrame(&mut self, mut angle: i16) {
        { let java_arguments = ({ let java_receiver = self.frameUtil.clone(); let java_arguments = (angle.clone(), ((jcall!(jcall!(self.angleInfo).getAngleIncrementInfo()).getAngleIncrement()) as i32),); let java_result = jcall!(java_receiver).getFrameForAngle(java_arguments.0, java_arguments.1); java_result },); self.setFrame(java_arguments.0) };
    }
}
impl std::ops::Deref for AllBinarySpriteRotationAnimation {
    type Target = HackRotationSpriteIndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinarySpriteRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait AllBinarySpriteRotationAnimationDyn: crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn {
    fn as_AllBinarySpriteRotationAnimation(&self) -> &AllBinarySpriteRotationAnimation;
    fn as_AllBinarySpriteRotationAnimation_mut(&mut self) -> &mut AllBinarySpriteRotationAnimation;
}
impl AllBinarySpriteRotationAnimationDyn for AllBinarySpriteRotationAnimation {
    fn as_AllBinarySpriteRotationAnimation(&self) -> &AllBinarySpriteRotationAnimation { self }
    fn as_AllBinarySpriteRotationAnimation_mut(&mut self) -> &mut AllBinarySpriteRotationAnimation { self }
}

impl crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn for AllBinarySpriteRotationAnimation {
    fn as_HackRotationSpriteIndexedAnimation(&self) -> &crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation { &self.base }
    fn as_HackRotationSpriteIndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinarySpriteRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinarySpriteRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getFrame(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getAnimationSize(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getSize(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::setSequence(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getSequence(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinarySpriteRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, angle) }
    fn adjustFrame(&mut self, mut angle: i16) { Self::adjustFrame(self, angle) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinarySpriteRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


