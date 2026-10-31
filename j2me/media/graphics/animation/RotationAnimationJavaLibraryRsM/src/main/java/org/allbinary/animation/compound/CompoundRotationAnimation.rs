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
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::RotationAnimation::RotationAnimation;
use crate::org::allbinary::direction::Direction::Direction;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::Angle::Angle;
use crate::org::allbinary::math::AngleFactory::AngleFactory;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::CompoundAnimationInterface::CompoundAnimationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundRotationAnimation {
    pub base: RotationAnimation,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>,
}

// impl CompoundAnimationInterface for CompoundRotationAnimation with the methods of the trait
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for CompoundRotationAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for CompoundRotationAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::animation::RotationAnimationInterface::RotationAnimationInterface for CompoundRotationAnimation {
    fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> { Self::getAngleInfoP(self) }
    fn nextRotation(&mut self) { Self::nextRotation(self) }
    fn previousRotation(&mut self) { Self::previousRotation(self) }
}
impl crate::org::allbinary::direction::DirectionalInterface::DirectionalInterface for CompoundRotationAnimation {
    fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) { Self::setFrameByDirection(self, direction) }
    fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::setFrameToAngle(self, angle) }
    fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) { Self::adjustFrameToAngle(self, newAngle) }
    fn adjustFrame(&mut self, mut angle: i16) { Self::adjustFrame(self, angle) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CompoundRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CompoundRotationAnimation {
    

    pub fn new_struct(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(AngleInfo::getInstance(jcall!(AngleFactory::getInstance()).QUARTER_TOTAL_ANGLE.clone()), CircularIndexUtil::createInstance(4), animationBehavior.clone()),
            animationInterfaceArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.animationInterfaceArray = animationInterfaceArray.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance((this.animationInterfaceArray.len() as i32).clone());
        this
    }

    pub fn new(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterfaceArray, animationBehavior))))
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).setFrame(index.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).getFrame();
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).getAnimationSize();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).getSize();
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).previousFrame();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).nextFrame();
    }
    

    pub fn previousRotation(&mut self) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).previousRotation();
    }
    

    pub fn nextRotation(&mut self) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).nextRotation();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).paintThreedXYZ(graphics.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn getCurrentAnimation(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> {
        return self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
    }
    

    pub fn nextAnimation(&mut self) {
        let mut frame: i32 = self.getFrame();
        jcall!(self.circularIndexUtil).next();
        self.setFrame(frame.clone());
    }
    

    pub fn previousAnimation(&mut self) {
        let mut frame: i32 = self.getFrame();
        jcall!(self.circularIndexUtil).previous();
        self.setFrame(frame.clone());
    }
    

    pub fn setAnimation(&mut self, mut index: i32) {
        let mut frame: i32 = self.getFrame();
        jcall!(self.circularIndexUtil).setIndex(index.clone());
        self.setFrame(frame.clone());
    }
    

    pub fn setFrameByDirection(&mut self, mut direction: Option<Rc<RefCell<Direction>>>) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).setFrameByDirection(direction.clone());
    }
    

    pub fn setFrameToAngle(&mut self, mut angle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).setFrameToAngle(angle.clone());
    }
    

    pub fn adjustFrameToAngle(&mut self, mut newAngle: Option<Rc<RefCell<dyn crate::org::allbinary::math::Angle::AngleDyn>>>) {
        { let java_arguments = (jcall!(newAngle).getValue(),); self.adjustFrame(java_arguments.0) };
    }
    

    pub fn adjustFrame(&mut self, mut angle: i16) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).adjustFrame(angle.clone());
    }
    

    pub fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> {
        return jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).getAngleInfoP();
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
impl std::ops::Deref for CompoundRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompoundRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CompoundRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for CompoundRotationAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for CompoundRotationAnimation {
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
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CompoundRotationAnimation {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


