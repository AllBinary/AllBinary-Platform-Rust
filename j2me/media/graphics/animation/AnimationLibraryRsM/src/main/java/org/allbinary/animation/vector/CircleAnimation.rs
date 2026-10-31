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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::ColorCompositeInterface::ColorCompositeInterface;
use crate::org::allbinary::math::AngleFactory::AngleFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CircleAnimation {
    pub base: Animation,
    pub TOTAL_ANGLE: i32,
    pub width: i32,
    pub height: i32,
}

// impl ColorCompositeInterface for CircleAnimation with the methods of the trait
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CircleAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CircleAnimation {
    

    pub fn new_struct(mut width: i32, mut height: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            TOTAL_ANGLE: 0,
            width: 0,
            height: 0,
        };
        this.TOTAL_ANGLE = ((jcall!(AngleFactory::getInstance()).TOTAL_ANGLE) as i32);
        this.width = width.clone();
        this.height = height.clone();
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut width: i32, mut height: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height, basicColor))))
    }
    

    pub fn nextFrame(&mut self) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        { let java_receiver = self.basicSetColorUtil.clone(); let java_arguments = (graphics.clone(), self.getBasicColorP(), self.getColor(),); jcall!(java_receiver).setBasicColorP3(java_arguments.0, java_arguments.1, java_arguments.2); };
        jcall!(graphics).drawArc(x.clone(), y.clone(), self.width.clone(), self.height.clone(), 0, self.TOTAL_ANGLE.clone());
    }
}
impl std::ops::Deref for CircleAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CircleAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CircleAnimationDyn: crate::org::allbinary::animation::Animation::AnimationDyn {
    fn as_CircleAnimation(&self) -> &CircleAnimation;
    fn as_CircleAnimation_mut(&mut self) -> &mut CircleAnimation;
}
impl CircleAnimationDyn for CircleAnimation {
    fn as_CircleAnimation(&self) -> &CircleAnimation { self }
    fn as_CircleAnimation_mut(&mut self) -> &mut CircleAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CircleAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CircleAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


