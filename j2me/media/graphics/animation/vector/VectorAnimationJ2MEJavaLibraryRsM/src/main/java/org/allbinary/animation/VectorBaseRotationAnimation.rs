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
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::math::AngleInfo::AngleInfo;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::RotationAnimation::RotationAnimation;
use super::VectorAnimationInterface::VectorAnimationInterface;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorBaseRotationAnimation {
    pub base: RotationAnimation,
    pub currentPoints: Vec<Vec<Vec<i32>>>,
}

// impl VectorAnimationInterface for VectorBaseRotationAnimation with the methods of the trait
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for VectorBaseRotationAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for VectorBaseRotationAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl VectorBaseRotationAnimation {
    

    pub fn new_struct(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: RotationAnimation::new_struct(angleInfo.clone(), CircularIndexUtil::createInstance(360 / jcall!(jcall!(angleInfo).getAngleIncrementInfo()).getAngleIncrement()), animationBehavior.clone()),
            currentPoints: Vec::new(),
        };
        this.currentPoints = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY_ARRAY.clone();
        // super(...) is in the struct literal above
        this.setPoints(currentPoints.clone());
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut angleInfo: Option<Rc<RefCell<AngleInfo>>>, mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(angleInfo, currentPoints, basicColor, animationBehavior))))
    }
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.circularIndexUtil).getIndex();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.circularIndexUtil).setIndex(index.clone());
    }
    

    pub fn nextFrame(&mut self) {
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.circularIndexUtil).previous();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return (self.currentPoints.len() as i32).clone();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.basicColor.clone());
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut nextPointX: i32 = 0;
            let mut nextPointY: i32 = 0;
            let mut nextPoint: Vec<i32> = Vec::new();
            let mut point: Vec<i32> = Vec::new();
            let mut currentPointsFrame: Vec<Vec<i32>> = self.currentPoints[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
            let mut size: i32 = (currentPointsFrame.len() as i32).clone();
            let mut index: i32 = size - 2;
            while { index -= 1; index } >= 0 {
                nextPoint = currentPointsFrame[(index) as usize].clone();
                point = currentPointsFrame[(index + 1) as usize].clone();
                nextPointX = nextPoint[(0) as usize].clone();
                nextPointY = nextPoint[(1) as usize].clone();
                if nextPointX != 1000 {
                    jcall!(graphics).drawLine(point[(0) as usize] + x, point[(1) as usize] + y, nextPointX + x, nextPointY + y);
                } else {
                    index -= 1;
                }
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("paintVectors"), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn getPoints(&mut self, mut frame: i32) -> Vec<Vec<i32>> {
        return self.currentPoints[(frame) as usize].clone();
    }
    

    pub fn setPoints(&mut self, mut currentPoints: Vec<Vec<Vec<i32>>>) {
        self.currentPoints = currentPoints.clone();
        self.circularIndexUtil = CircularIndexUtil::createInstance((self.currentPoints.len() as i32).clone());
    }
}
impl std::ops::Deref for VectorBaseRotationAnimation {
    type Target = RotationAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VectorBaseRotationAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait VectorBaseRotationAnimationDyn: crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn {
    fn as_VectorBaseRotationAnimation(&self) -> &VectorBaseRotationAnimation;
    fn as_VectorBaseRotationAnimation_mut(&mut self) -> &mut VectorBaseRotationAnimation;
    fn getPoints(&mut self, mut frame: i32) -> Vec<Vec<i32>> { VectorBaseRotationAnimation::getPoints(self.as_VectorBaseRotationAnimation_mut(), frame) }
    fn setPoints(&mut self, mut currentPoints: Vec<Vec<Vec<i32>>>) { VectorBaseRotationAnimation::setPoints(self.as_VectorBaseRotationAnimation_mut(), currentPoints) }
}
impl VectorBaseRotationAnimationDyn for VectorBaseRotationAnimation {
    fn as_VectorBaseRotationAnimation(&self) -> &VectorBaseRotationAnimation { self }
    fn as_VectorBaseRotationAnimation_mut(&mut self) -> &mut VectorBaseRotationAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for VectorBaseRotationAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for VectorBaseRotationAnimation {
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
impl crate::org::allbinary::animation::RotationAnimation::RotationAnimationDyn for VectorBaseRotationAnimation {
    fn as_RotationAnimation(&self) -> &crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &self.base }
    fn as_RotationAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::RotationAnimation::RotationAnimation { &mut self.base }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for VectorBaseRotationAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


