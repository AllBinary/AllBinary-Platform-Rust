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
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::color::ColorCompositeInterface::ColorCompositeInterface;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpriteIndexedAnimation {
    pub base: IndexedAnimation,
    pub sprite: Option<Rc<RefCell<Sprite>>>,
    pub image: Option<Rc<RefCell<Image>>>,
    pub basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>,
}

impl ColorCompositeInterface for SpriteIndexedAnimation {
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getBasicColorP(self) }
}
impl crate::org::allbinary::graphics::color::BasicColorCompositeSetterInterface::BasicColorCompositeSetterInterface for SpriteIndexedAnimation {
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for SpriteIndexedAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for SpriteIndexedAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for SpriteIndexedAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl SpriteIndexedAnimation {
    

    pub fn new_struct(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            sprite: None,
            image: None,
            basicColorArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.sprite = sprite.clone();
        this.image = image.clone();
        this.basicColorArray = basicColorArray.clone();
        if (this.basicColorArray.len() as i32) != 0 && this.getSize() != (this.basicColorArray.len() as i32) {
            panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(jcall!(CommonLabels::getInstance()).TOTAL_LABEL.clone()).appendint(this.getSize()).append(String::from("!=")).appendint((this.basicColorArray.len() as i32).clone()).toString())));
        }
        this
    }

    pub fn new(mut sprite: Option<Rc<RefCell<Sprite>>>, mut image: Option<Rc<RefCell<Image>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sprite, image, basicColorArray, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        for index in (0..(self.basicColorArray.len() as i32)) {
            if match (&basicColor, &self.basicColorArray[(index) as usize]) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                self.setFrame(index.clone());
                break;
            }
        }
    }
    

    pub fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        if (self.basicColorArray.len() as i32) < self.getFrame() {
            return self.basicColorArray[(self.getFrame()) as usize].clone();
        } else {
            return jcall!(BasicColorFactory::getInstance()).WHITE.clone();
        }
    }
    

    pub fn paintFrame(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) {
        self.setFrame(frame.clone());
        jcall!(self.sprite).setPosition(x.clone(), y.clone());
        jcall!(self.sprite).paint(graphics.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.sprite).setPosition(x.clone(), y.clone());
        jcall!(self.sprite).paint(graphics.clone());
    }
    

    pub fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) {
        jcall!(self.sprite).paint(graphics.clone());
    }
    

    pub fn nextFrame(&mut self) {
        jcall!(self.sprite).nextFrame();
    }
    

    pub fn previousFrame(&mut self) {
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
    

    pub fn isLastFrame(&mut self) -> bool {
        if jcall!(self.sprite).getFrame() == self.getSize() - 1 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn getLastFrame(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    // throws Exception
    pub fn close(&mut self) {
    }
    

    // throws Throwable
    pub fn finalize(&mut self) {
    }
}
impl std::ops::Deref for SpriteIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SpriteIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait SpriteIndexedAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_SpriteIndexedAnimation(&self) -> &SpriteIndexedAnimation;
    fn as_SpriteIndexedAnimation_mut(&mut self) -> &mut SpriteIndexedAnimation;
    fn paintFrame(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) { SpriteIndexedAnimation::paintFrame(self.as_SpriteIndexedAnimation_mut(), graphics, frame, x, y) }
    fn paint(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>) { SpriteIndexedAnimation::paint(self.as_SpriteIndexedAnimation_mut(), graphics) }
    fn getLastFrame(&mut self) -> i32 { SpriteIndexedAnimation::getLastFrame(self.as_SpriteIndexedAnimation_mut()) }
    fn close(&mut self) { SpriteIndexedAnimation::close(self.as_SpriteIndexedAnimation_mut()) }
    fn finalize(&mut self) { SpriteIndexedAnimation::finalize(self.as_SpriteIndexedAnimation_mut()) }
}
impl SpriteIndexedAnimationDyn for SpriteIndexedAnimation {
    fn as_SpriteIndexedAnimation(&self) -> &SpriteIndexedAnimation { self }
    fn as_SpriteIndexedAnimation_mut(&mut self) -> &mut SpriteIndexedAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for SpriteIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn getBasicColorP(&mut self) -> Option<Rc<RefCell<BasicColor>>> { Self::getBasicColorP(self) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for SpriteIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn isLastFrame(&mut self) -> bool { Self::isLastFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for SpriteIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


