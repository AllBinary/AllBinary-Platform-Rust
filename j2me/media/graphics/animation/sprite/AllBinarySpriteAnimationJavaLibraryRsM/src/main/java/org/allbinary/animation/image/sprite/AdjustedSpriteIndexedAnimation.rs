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
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorUtil::BasicColorUtil;
use super::SpriteIndexedAnimation::SpriteIndexedAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdjustedSpriteIndexedAnimation {
    pub base: SpriteIndexedAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AdjustedSpriteIndexedAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustedSpriteIndexedAnimation {
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
}

impl AdjustedSpriteIndexedAnimation {
    

    pub fn new_struct(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: SpriteIndexedAnimation::new_struct(sprite.clone(), image.clone(), basicColorArray.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.dx = dx.clone();
        this.dy = dy.clone();
        this
    }

    pub fn new(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sprite, image, basicColorArray, dx, dy, animationBehavior))))
    }
    

    pub fn paintFrame(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) {
        self.setFrame(frame.clone());
        jcall!(self.sprite).setPosition(x + self.dx, y + self.dy);
        self.base.paint(graphics.clone());
    }
    

    pub fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.sprite).setPosition(x + self.dx, y + self.dy);
        self.base.paint(g.clone());
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.sprite).setPosition(self.dx.clone(), self.dy.clone());
        self.base.paint(g.clone());
    }
}
impl std::ops::Deref for AdjustedSpriteIndexedAnimation {
    type Target = SpriteIndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustedSpriteIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn for AdjustedSpriteIndexedAnimation {
    fn as_SpriteIndexedAnimation(&self) -> &crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation { &self.base }
    fn as_SpriteIndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation { &mut self.base }
    fn paintFrame(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) { Self::paintFrame(self, graphics, frame, x, y) }
    fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>) { Self::paint(self, g) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustedSpriteIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::nextFrame(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn paintXY(&mut self, mut g: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, g, x, y) }
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>> { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::getBasicColorP(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn setBasicColorP(&mut self, p1: Option<Rc<RefCell<crate::org::allbinary::graphics::color::BasicColor::BasicColor>>>) { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::setBasicColorP(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self), p1) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AdjustedSpriteIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::setFrame(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::getFrame(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::getAnimationSize(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::getSize(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn previousFrame(&mut self) { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::previousFrame(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn isLastFrame(&mut self) -> bool { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::isLastFrame(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
    fn setSequence(&mut self, p1: Vec<i32>) { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::setSequence(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self), p1) }
    fn getSequence(&mut self) -> Vec<i32> { crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimation::getSequence(crate::org::allbinary::animation::image::sprite::SpriteIndexedAnimation::SpriteIndexedAnimationDyn::as_SpriteIndexedAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustedSpriteIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


