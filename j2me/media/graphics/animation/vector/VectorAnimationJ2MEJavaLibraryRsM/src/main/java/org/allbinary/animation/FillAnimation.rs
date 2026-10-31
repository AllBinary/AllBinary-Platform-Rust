// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use super::IndexedAnimation::IndexedAnimation;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct FillAnimation {
    pub base: IndexedAnimation,
    pub pixelBuffer: Vec<i32>,
    pub width: i32,
    pub height: i32,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for FillAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl FillAnimation {
    

    pub fn new_struct(mut pixelBuffer: Vec<i32>, mut width: i32, mut height: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            pixelBuffer: Vec::new(),
            width: 0,
            height: 0,
        };
        // super(...) is in the struct literal above
        this.width = 100;
        this.height = 100;
        this.pixelBuffer = pixelBuffer.clone();
        this
    }

    pub fn new(mut pixelBuffer: Vec<i32>, mut width: i32, mut height: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(pixelBuffer, width, height, animationBehavior))))
    }
    

    pub fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getBasicColorP().is_none() || ((self.getBasicColorP()).unwrap() as i32) != jcall!(basicColor).intValue() {
            changed = true;
        }
        self.base.setBasicColorP(basicColor.clone());
        if changed {
            self.updateModifiers();
        }
    }
    

    pub fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        let mut changed: bool = false;
        if self.getChangeBasicColor().is_none() || ((self.getChangeBasicColor()).unwrap() as i32) != jcall!(basicColor).intValue() {
            changed = true;
        }
        self.base.changeBasicColor(basicColor.clone());
        if changed {
            self.updateModifiers();
        }
    }
    

    pub fn setAlpha(&mut self, mut alpha: i32) {
        let mut changed: bool = false;
        if self.alphaP != alpha {
            changed = true;
        }
        self.base.setAlpha(alpha.clone());
        if changed {
            self.updateModifiers();
        }
    }
    

    pub fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) {
    }
    

    pub fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) {
    }
    

    pub fn updateModifiers(&mut self) {
    }
    

    pub fn paint(&mut self, mut g: Option<Rc<RefCell<Graphics>>>) {
        jcall!(g).drawRGB(self.pixelBuffer.clone(), 0, self.width.clone(), 0, 0, self.width.clone(), self.height.clone(), true);
    }
}
impl std::ops::Deref for FillAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for FillAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for FillAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Self::setScale(self, scaleX, scaleY) }
    fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) { Self::setMaxScale(self, maxScaleX, maxScaleY) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for FillAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for FillAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


