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
// package org.allbinary.animation.compound
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::khronos::opengles::GL::GL;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimultaneousCompoundRotationAnimation {
    pub base: RotationAnimation,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for SimultaneousCompoundRotationAnimation {
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for SimultaneousCompoundRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut frameIndex: i32) { Self::setFrame(self, frameIndex) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for SimultaneousCompoundRotationAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::animation::RotationAnimationInterface::RotationAnimationInterface for SimultaneousCompoundRotationAnimation {
    fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> { Self::getAngleInfoP(self) }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for SimultaneousCompoundRotationAnimation {
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, newAngle) }
    fn adjustFrame(&mut self, mut angle: i16) { Self::adjustFrame(self, angle) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for SimultaneousCompoundRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl SimultaneousCompoundRotationAnimation {
    

    pub fn new_struct(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(AngleInfo::getInstance(jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone()), CircularIndexUtil::createInstance(4), animationBehavior.clone()),
            animationInterfaceArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.animationInterfaceArray = animationInterfaceArray.clone();
        this
    }

    pub fn new(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterfaceArray, animationBehavior))))
    }
    

    pub fn setFrame(&mut self, mut frameIndex: i32) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).setFrame(frameIndex.clone());
            }
        }
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getFrame();
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getAnimationSize();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getSize();
    }
    

    pub fn previousFrame(&mut self) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).previousFrame();
            }
        }
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).nextFrame();
            }
        }
    }
    

    pub fn previousRotation(&mut self) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).previousRotation();
            }
        }
    }
    

    pub fn nextRotation(&mut self) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).nextRotation();
            }
        }
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
            }
        }
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).paintThreedXYZ(graphics.clone(), x.clone(), y.clone(), z.clone());
            }
        }
    }
    

    pub fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).setFrameByDirection(direction.clone());
            }
        }
    }
    

    pub fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).setFrameToAngle(angle.clone());
            }
        }
    }
    

    pub fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (jcall!(newAngle).getValue(),); self.adjustFrame(java_arguments.0) };
    }
    

    pub fn adjustFrame(&mut self, mut angle: i16) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).adjustFrame(angle.clone());
            }
        }
    }
    

    pub fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getAngleInfoP();
    }
    

    pub fn getAnimationInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>> {
        return self.animationInterfaceArray.clone();
    }
    

    pub fn setAnimationInterfaceArray(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>) {
        self.animationInterfaceArray = animationInterfaceArray.clone();
    }
    

    // throws Exception
    pub fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) {
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            jcall!(self.animationInterfaceArray[(index) as usize]).set(gl.clone());
            }
        }
    }
}
impl std::ops::Deref for SimultaneousCompoundRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimultaneousCompoundRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for SimultaneousCompoundRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for SimultaneousCompoundRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut frameIndex: i32) { Self::setFrame(self, frameIndex) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for SimultaneousCompoundRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, newAngle) }
    fn adjustFrame(&mut self, mut angle: i16) { Self::adjustFrame(self, angle) }
    fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> { Self::getAngleInfoP(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for SimultaneousCompoundRotationAnimation {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


