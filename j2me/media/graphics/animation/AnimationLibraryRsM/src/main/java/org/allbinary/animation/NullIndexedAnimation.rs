// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::IndexedAnimation::IndexedAnimation;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullIndexedAnimation {
    pub base: IndexedAnimation,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for NullIndexedAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl NullIndexedAnimation {
    

    pub fn new_struct(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return 0;
    }
}
impl std::ops::Deref for NullIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait NullIndexedAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_NullIndexedAnimation(&self) -> &NullIndexedAnimation;
    fn as_NullIndexedAnimation_mut(&mut self) -> &mut NullIndexedAnimation;
}
impl NullIndexedAnimationDyn for NullIndexedAnimation {
    fn as_NullIndexedAnimation(&self) -> &NullIndexedAnimation { self }
    fn as_NullIndexedAnimation_mut(&mut self) -> &mut NullIndexedAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for NullIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for NullIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for NullIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


