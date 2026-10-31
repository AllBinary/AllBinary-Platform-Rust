// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.animation.text
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use super::CustomTextAnimation::CustomTextAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdjustCustomTextAnimation {
    pub base: CustomTextAnimation,
    pub dx: i32,
    pub dy: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for AdjustCustomTextAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustCustomTextAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AdjustCustomTextAnimation {
    

    pub fn new_struct(mut text: String, mut fontSize: i32, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: CustomTextAnimation::new_struct(text.clone(), fontSize.clone(), animationBehavior.clone()),
            dx: 0,
            dy: 0,
        };
        // super(...) is in the struct literal above
        this.dx = dx.clone();
        this.dy = dy.clone();
        this
    }

    pub fn new(mut text: String, mut fontSize: i32, mut dx: i32, mut dy: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(text, fontSize, dx, dy, animationBehavior))))
    }
    

    pub fn getDx(&mut self) -> i32 {
        return self.dx.clone();
    }
    

    pub fn setDx(&mut self, mut dx: i32) {
        self.dx = dx.clone();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return self.dy.clone();
    }
    

    pub fn setDy(&mut self, mut dy: i32) {
        self.dy = dy.clone();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        self.base.paintXY(graphics.clone(), x + self.dx, y + self.dy);
    }
}
impl std::ops::Deref for AdjustCustomTextAnimation {
    type Target = CustomTextAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustCustomTextAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn for AdjustCustomTextAnimation {
    fn as_CustomTextAnimation(&self) -> &crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation { &self.base }
    fn as_CustomTextAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn for AdjustCustomTextAnimation {
    fn as_TextAnimation(&self) -> &crate::org::allbinary::animation::text::TextAnimation::TextAnimation { &self.base.base }
    fn as_TextAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::text::TextAnimation::TextAnimation { &mut self.base.base }
    fn setText(&mut self, p1: String) { crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation::setText(crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn::as_CustomTextAnimation_mut(self), p1) }
    fn getFontHeight(&mut self) -> i32 { crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation::getFontHeight(crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn::as_CustomTextAnimation_mut(self)) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustCustomTextAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::text::TextAnimation::TextAnimation::nextFrame(crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn::as_TextAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setScale(&mut self, p1: f32, p2: f32) { crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation::setScale(crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn::as_CustomTextAnimation_mut(self), p1, p2) }
    fn setDx(&mut self, mut dx: i32) { Self::setDx(self, dx) }
    fn getDx(&mut self) -> i32 { Self::getDx(self) }
    fn setDy(&mut self, mut dy: i32) { Self::setDy(self, dy) }
    fn getDy(&mut self) -> i32 { Self::getDy(self) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for AdjustCustomTextAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base.base }
    fn getWidth(&mut self) -> i32 { crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimation::getWidth(crate::org::allbinary::animation::text::CustomTextAnimation::CustomTextAnimationDyn::as_CustomTextAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustCustomTextAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


