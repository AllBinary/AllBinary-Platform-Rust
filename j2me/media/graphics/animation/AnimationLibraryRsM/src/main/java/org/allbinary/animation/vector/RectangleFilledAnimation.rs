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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RectangleFilledAnimation {
    pub base: Animation,
    pub widthP: i32,
    pub heightP: i32,
}

// impl ColorCompositeInterface for RectangleFilledAnimation with the methods of the trait
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for RectangleFilledAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl RectangleFilledAnimation {
    

    pub fn new_struct(mut width: i32, mut height: i32, mut basicColor: Option<Rc<RefCell<BasicColor>>>) -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
            widthP: 0,
            heightP: 0,
        };
        this.widthP = width.clone();
        this.heightP = height.clone();
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
        jcall!(graphics).fillRect(x.clone(), y.clone(), self.widthP.clone(), self.heightP.clone());
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        self.widthP = width.clone();
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.heightP = height.clone();
    }
}
impl std::ops::Deref for RectangleFilledAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for RectangleFilledAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait RectangleFilledAnimationDyn: crate::org::allbinary::animation::Animation::AnimationDyn {
    fn as_RectangleFilledAnimation(&self) -> &RectangleFilledAnimation;
    fn as_RectangleFilledAnimation_mut(&mut self) -> &mut RectangleFilledAnimation;
    fn setWidth(&mut self, mut width: i32) { RectangleFilledAnimation::setWidth(self.as_RectangleFilledAnimation_mut(), width) }
    fn setHeight(&mut self, mut height: i32) { RectangleFilledAnimation::setHeight(self.as_RectangleFilledAnimation_mut(), height) }
}
impl RectangleFilledAnimationDyn for RectangleFilledAnimation {
    fn as_RectangleFilledAnimation(&self) -> &RectangleFilledAnimation { self }
    fn as_RectangleFilledAnimation_mut(&mut self) -> &mut RectangleFilledAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for RectangleFilledAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for RectangleFilledAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


