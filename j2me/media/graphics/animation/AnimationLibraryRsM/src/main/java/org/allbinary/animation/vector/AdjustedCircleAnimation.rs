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
// package org.allbinary.animation.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::AndroidUtil::AndroidUtil;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::CircleAnimation::CircleAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdjustedCircleAnimation {
    pub base: CircleAnimation,
    pub xAdjustment: i32,
    pub yAdjustment: i32,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for AdjustedCircleAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl AdjustedCircleAnimation {
    

    pub fn createAnimation(mut width: i32, mut height: i32, mut xAdjustment: i32, mut yAdjustment: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<AdjustedCircleAnimation>>> {
        let mut adjustedCircleAnimation: Option<Rc<RefCell<AdjustedCircleAnimation>>> = AdjustedCircleAnimation::new(width.clone(), height.clone(), basicColor.clone());
        jcall!(adjustedCircleAnimation).initXY(xAdjustment.clone(), yAdjustment.clone());
        return adjustedCircleAnimation.clone();
    }
    

    pub fn createW(mut width: i32, mut height: i32, mut innerWidth: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<AdjustedCircleAnimation>>> {
        let mut adjustedCircleAnimation: Option<Rc<RefCell<AdjustedCircleAnimation>>> = AdjustedCircleAnimation::new(width.clone(), height.clone(), basicColor.clone());
        jcall!(adjustedCircleAnimation).init(innerWidth.clone());
        return adjustedCircleAnimation.clone();
    }
    

    pub fn new_struct(mut width: i32, mut height: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: CircleAnimation::new_struct(width.clone(), height.clone(), basicColor.clone()),
            xAdjustment: 0,
            yAdjustment: 0,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut width: i32, mut height: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height, basicColor))))
    }
    

    pub fn initXY(&mut self, mut xAdjustment: i32, mut yAdjustment: i32) {
        self.xAdjustment = xAdjustment.clone();
        self.yAdjustment = yAdjustment.clone();
    }
    

    pub fn init(&mut self, mut innerWidth: i32) {
        if AndroidUtil::isAndroid() {
            let mut widthPortion: i32 = (((innerWidth / 3) * 10) / 7);
            self.xAdjustment = -((self.width >> 1) - widthPortion);
            self.yAdjustment = self.xAdjustment.clone();
        } else {
            self.xAdjustment = -((self.width >> 1) - innerWidth);
            self.yAdjustment = self.xAdjustment.clone();
        }
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(graphics).drawArc(x + self.xAdjustment, y + self.yAdjustment, self.width.clone(), self.height.clone(), 0, self.TOTAL_ANGLE.clone());
    }
}
impl std::ops::Deref for AdjustedCircleAnimation {
    type Target = CircleAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AdjustedCircleAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::vector::CircleAnimation::CircleAnimationDyn for AdjustedCircleAnimation {
    fn as_CircleAnimation(&self) -> &crate::org::allbinary::animation::vector::CircleAnimation::CircleAnimation { &self.base }
    fn as_CircleAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::vector::CircleAnimation::CircleAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for AdjustedCircleAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { crate::org::allbinary::animation::vector::CircleAnimation::CircleAnimation::nextFrame(crate::org::allbinary::animation::vector::CircleAnimation::CircleAnimationDyn::as_CircleAnimation_mut(self)) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for AdjustedCircleAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


