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
// package org.allbinary.animation.caption
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CaptionAnimationHelperBase {
    pub base: Animation,
}

thread_local! { pub static CaptionAnimationHelperBase_INSTANCE: Option<Rc<RefCell<dyn CaptionAnimationHelperBaseDyn>>> = CaptionAnimationHelperBase::new().map(|reference| reference as Rc<RefCell<dyn CaptionAnimationHelperBaseDyn>>); }
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CaptionAnimationHelperBase {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CaptionAnimationHelperBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Animation::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isShowing(&mut self) -> bool {
        return false;
    }
    

    pub fn tick(&mut self) {
    }
    

    pub fn update(&mut self, mut message: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
    }
}
impl std::ops::Deref for CaptionAnimationHelperBase {
    type Target = Animation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CaptionAnimationHelperBase {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait CaptionAnimationHelperBaseDyn: crate::org::allbinary::animation::Animation::AnimationDyn {
    fn as_CaptionAnimationHelperBase(&self) -> &CaptionAnimationHelperBase;
    fn as_CaptionAnimationHelperBase_mut(&mut self) -> &mut CaptionAnimationHelperBase;
    fn isShowing(&mut self) -> bool { CaptionAnimationHelperBase::isShowing(self.as_CaptionAnimationHelperBase_mut()) }
    fn tick(&mut self) { CaptionAnimationHelperBase::tick(self.as_CaptionAnimationHelperBase_mut()) }
    fn update(&mut self, mut message: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { CaptionAnimationHelperBase::update(self.as_CaptionAnimationHelperBase_mut(), message, basicColor) }
}
impl CaptionAnimationHelperBaseDyn for CaptionAnimationHelperBase {
    fn as_CaptionAnimationHelperBase(&self) -> &CaptionAnimationHelperBase { self }
    fn as_CaptionAnimationHelperBase_mut(&mut self) -> &mut CaptionAnimationHelperBase { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CaptionAnimationHelperBase {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CaptionAnimationHelperBase {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


