// Generated Code Do Not Modify
// package org.allbinary.animation.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use super::RectangleFilledAnimation::RectangleFilledAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ARectangleFilledAnimation {
    pub base: RectangleFilledAnimation,
    pub x: i32,
    pub y: i32,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for ARectangleFilledAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut unusedX: i32, mut unusedY: i32) { Self::paintXY(self, graphics, unusedX, unusedY) }
}

impl ARectangleFilledAnimation {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: RectangleFilledAnimation::new_struct(0, 0, jcall!(BasicColorFactory::getInstance()).BLACK.clone()),
            x: 0,
            y: 0,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn nextFrame(&mut self) {
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut unusedX: i32, mut unusedY: i32) {
        self.base.paintXY(graphics.clone(), self.x.clone(), self.y.clone());
    }
}
impl std::ops::Deref for ARectangleFilledAnimation {
    type Target = RectangleFilledAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ARectangleFilledAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimationDyn for ARectangleFilledAnimation {
    fn as_RectangleFilledAnimation(&self) -> &crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimation { &self.base }
    fn as_RectangleFilledAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::vector::RectangleFilledAnimation::RectangleFilledAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for ARectangleFilledAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut unusedX: i32, mut unusedY: i32) { Self::paintXY(self, graphics, unusedX, unusedY) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for ARectangleFilledAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


