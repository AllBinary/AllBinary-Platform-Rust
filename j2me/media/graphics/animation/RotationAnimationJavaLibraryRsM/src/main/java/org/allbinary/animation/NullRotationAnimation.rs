// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::RotationAnimation::RotationAnimation;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullRotationAnimation {
    pub base: RotationAnimation,
}

impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for NullRotationAnimation {
    fn getSize(&mut self) -> i32 { self.base.getSize() }
    fn previousFrame(&mut self) { self.base.previousFrame() }
}

impl NullRotationAnimation {
    

    pub fn createTotalAngle(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut totalAngle: i16, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn NullRotationAnimationDyn>>> {
        return NullRotationAnimation::new(angleInfo.clone(), CircularIndexUtil::createInstance(totalAngle / jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement()), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn NullRotationAnimationDyn>>);
    }
    

    pub fn createQuarter(mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn NullRotationAnimationDyn>>> {
        return NullRotationAnimation::new(AngleInfo::getInstance(jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone()), CircularIndexUtil::createInstance(4), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn NullRotationAnimationDyn>>);
    }
    

    pub fn new_struct(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(angleInfo.clone(), circularIndexUtil.clone(), animationBehavior.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angleInfo, circularIndexUtil, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return 0;
    }
}
impl std::ops::Deref for NullRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NullRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait NullRotationAnimationDyn: crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn {
    fn as_NullRotationAnimation(&self) -> &NullRotationAnimation;
    fn as_NullRotationAnimation_mut(&mut self) -> &mut NullRotationAnimation;
}
impl NullRotationAnimationDyn for NullRotationAnimation {
    fn as_NullRotationAnimation(&self) -> &NullRotationAnimation { self }
    fn as_NullRotationAnimation_mut(&mut self) -> &mut NullRotationAnimation { self }
}

impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for NullRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for NullRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for NullRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, p1: i32) { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::setFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self), p1) }
    fn getFrame(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getFrame(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { crate::org::allbinary::animation::RotationAnimation::RotationAnimation::getSize(crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn::as_RotationAnimation_mut(self)) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for NullRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


