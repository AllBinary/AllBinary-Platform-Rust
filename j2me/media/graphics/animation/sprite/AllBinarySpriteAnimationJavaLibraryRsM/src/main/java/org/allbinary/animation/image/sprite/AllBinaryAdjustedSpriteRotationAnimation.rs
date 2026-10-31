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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::lcdui::Image::Image;
// use crate::javax::microedition::lcdui::game::Sprite::Sprite;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use super::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AllBinaryAdjustedSpriteRotationAnimation {
    pub base: AllBinarySpriteRotationAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AllBinaryAdjustedSpriteRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AllBinaryAdjustedSpriteRotationAnimation {
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
}

impl AllBinaryAdjustedSpriteRotationAnimation {
    

    pub fn new_struct(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: AllBinarySpriteRotationAnimation::new_struct(sprite.clone(), image.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.dx = dx.clone();
        this.dy = dy.clone();
        this
    }

    pub fn new(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sprite, image, dx, dy, animationBehavior))))
    }
    

    pub fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.sprite).setPosition(x + self.dx, y + self.dy);
        self.paint(g.clone());
    }
}
impl std::ops::Deref for AllBinaryAdjustedSpriteRotationAnimation {
    type Target = AllBinarySpriteRotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AllBinaryAdjustedSpriteRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn for AllBinaryAdjustedSpriteRotationAnimation {
    fn as_AllBinarySpriteRotationAnimation(&self) -> &crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation { &self.base }
    fn as_AllBinarySpriteRotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn for AllBinaryAdjustedSpriteRotationAnimation {
    fn as_HackRotationSpriteIndexedAnimation(&self) -> &crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation { &self.base.base }
    fn as_HackRotationSpriteIndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation { &mut self.base.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AllBinaryAdjustedSpriteRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base.base }
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AllBinaryAdjustedSpriteRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::setFrame(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getFrame(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getAnimationSize(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getSize(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::setSequence(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimation::getSequence(crate::org::allbinary::animation::image::sprite::HackRotationSpriteIndexedAnimation::HackRotationSpriteIndexedAnimationDyn::as_HackRotationSpriteIndexedAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for AllBinaryAdjustedSpriteRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base.base.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base.base.base }
    fn nextRotation(&mut self) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::nextRotation(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self)) }
    fn previousRotation(&mut self) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::previousRotation(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self)) }
    fn setFrameByDirection(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::direction::Direction::Direction>>>) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::setFrameByDirection(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self), p1) }
    fn setFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::setFrameToAngle(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self), p1) }
    fn adjustFrameToAngle(&mut self, p1: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::adjustFrameToAngle(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self), p1) }
    fn adjustFrame(&mut self, p1: i16) { crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimation::adjustFrame(crate::org::allbinary::animation::image::sprite::AllBinarySpriteRotationAnimation::AllBinarySpriteRotationAnimationDyn::as_AllBinarySpriteRotationAnimation_mut(self), p1) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AllBinaryAdjustedSpriteRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


