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
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct HackRotationSpriteIndexedAnimation {
    pub base: RotationAnimation,
    pub sprite: Option<Rc<RefCell<Sprite>>>,
    pub image: Option<Rc<RefCell<Image>>>,
}

impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for HackRotationSpriteIndexedAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for HackRotationSpriteIndexedAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for HackRotationSpriteIndexedAnimation {
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
}

impl HackRotationSpriteIndexedAnimation {
    

    pub fn new_struct(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(angleInfo.clone(), CircularIndexUtil::createInstance(360 / jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement()), animationBehavior.clone()),
            sprite: None,
            image: None,
        };
        // super(...) is in the struct literal above
        this.sprite = sprite.clone();
        this.image = image.clone();
        this
    }

    pub fn new(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sprite, image, angleInfo, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn paintFrame(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) {
        self.setFrame(frame.clone());
        self.paintXY(g.clone(), x.clone(), y.clone());
    }
    

    pub fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.sprite).setPosition(x.clone(), y.clone());
        self.paint(g.clone());
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.sprite).paint(g.clone());
    }
    

    pub fn nextRotation(&mut self) {
        jcall!(self.sprite).nextFrame();
    }
    

    pub fn previousRotation(&mut self) {
        jcall!(self.sprite).prevFrame();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.sprite).getRawFrameCount();
    }
    

    pub fn setFrame(&mut self, mut frame: i32) {
        jcall!(self.sprite).setFrame(frame.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.sprite).getFrame();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
}
impl std::ops::Deref for HackRotationSpriteIndexedAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for HackRotationSpriteIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait HackRotationSpriteIndexedAnimationDyn: crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn {
    fn as_HackRotationSpriteIndexedAnimation(&self) -> &HackRotationSpriteIndexedAnimation;
    fn as_HackRotationSpriteIndexedAnimation_mut(&mut self) -> &mut HackRotationSpriteIndexedAnimation;
    fn paintFrame(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) { HackRotationSpriteIndexedAnimation::paintFrame(self.as_HackRotationSpriteIndexedAnimation_mut(), g, frame, x, y) }
    fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>) { HackRotationSpriteIndexedAnimation::paint(self.as_HackRotationSpriteIndexedAnimation_mut(), g) }
}
impl HackRotationSpriteIndexedAnimationDyn for HackRotationSpriteIndexedAnimation {
    fn as_HackRotationSpriteIndexedAnimation(&self) -> &HackRotationSpriteIndexedAnimation { self }
    fn as_HackRotationSpriteIndexedAnimation_mut(&mut self) -> &mut HackRotationSpriteIndexedAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for HackRotationSpriteIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for HackRotationSpriteIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for HackRotationSpriteIndexedAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for HackRotationSpriteIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


