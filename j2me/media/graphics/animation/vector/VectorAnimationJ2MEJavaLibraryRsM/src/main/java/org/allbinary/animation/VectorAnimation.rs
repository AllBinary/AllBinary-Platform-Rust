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
// use crate::javax::microedition::lcdui::NullImage::NullImage;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::IndexedAnimation::IndexedAnimation;
use super::VectorAnimationInterface::VectorAnimationInterface;
use super::AnimationBehavior::AnimationBehavior;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorAnimation {
    pub base: IndexedAnimation,
    pub currentPoints: Vec<Vec<Vec<i32>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
}

// impl VectorAnimationInterface for VectorAnimation with the methods of the trait
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for VectorAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for VectorAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl VectorAnimation {
    

    pub fn createVectorAnimation(mut currentPoints2: Vec<Vec<i32>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<dyn VectorAnimationDyn>>> {
        let mut currentPoints: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; (2) as usize]; ((currentPoints2.len() as i32)) as usize]; (1) as usize];
        let mut size: i32 = (currentPoints2.len() as i32).clone();
        for index in (0..size) {
            currentPoints[(0) as usize][(index) as usize][(0) as usize] = currentPoints2[(index) as usize][(0) as usize].clone();
            currentPoints[(0) as usize][(index) as usize][(1) as usize] = currentPoints2[(index) as usize][(1) as usize].clone();
        }
        return VectorAnimation::new(currentPoints.clone(), basicColor.clone(), animationBehavior.clone()).map(|reference| reference as Rc<RefCell<dyn VectorAnimationDyn>>);
    }
    

    pub fn new_struct(mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            currentPoints: Vec::new(),
            circularIndexUtil: None,
        };
        this.currentPoints = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY_ARRAY.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance(0);
        // super(...) is in the struct literal above
        this.setPoints(currentPoints.clone());
        this.setBasicColorP(basicColor.clone());
        this
    }

    pub fn new(mut currentPoints: Vec<Vec<Vec<i32>>>, mut basicColor: Option<Rc<RefCell<BasicColor>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(currentPoints, basicColor, animationBehavior))))
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
    

    // throws Exception
    pub fn getAnimationSize(&mut self) -> i32 {
        return self.getSize();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return (self.currentPoints.len() as i32).clone();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn nextFrame(&mut self) {
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.circularIndexUtil).previous();
    }
    

    pub fn paintVectors(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
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
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.basicColor.clone());
        self.paintVectors(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.circularIndexUtil).getIndex();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.circularIndexUtil).setIndex(index.clone());
    }
    

    pub fn getPoints(&mut self, mut frame: i32) -> Vec<Vec<i32>> {
        return self.currentPoints[(frame) as usize].clone();
    }
    

    pub fn setPoints(&mut self, mut currentPoints: Vec<Vec<Vec<i32>>>) {
        self.currentPoints = currentPoints.clone();
        self.circularIndexUtil = CircularIndexUtil::createInstance((currentPoints.len() as i32).clone());
    }
}
impl std::ops::Deref for VectorAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VectorAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait VectorAnimationDyn: crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn {
    fn as_VectorAnimation(&self) -> &VectorAnimation;
    fn as_VectorAnimation_mut(&mut self) -> &mut VectorAnimation;
    fn updateModifiers(&mut self) { VectorAnimation::updateModifiers(self.as_VectorAnimation_mut()) }
    fn paintVectors(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { VectorAnimation::paintVectors(self.as_VectorAnimation_mut(), graphics, x, y) }
    fn getPoints(&mut self, mut frame: i32) -> Vec<Vec<i32>> { VectorAnimation::getPoints(self.as_VectorAnimation_mut(), frame) }
    fn setPoints(&mut self, mut currentPoints: Vec<Vec<Vec<i32>>>) { VectorAnimation::setPoints(self.as_VectorAnimation_mut(), currentPoints) }
}
impl VectorAnimationDyn for VectorAnimation {
    fn as_VectorAnimation(&self) -> &VectorAnimation { self }
    fn as_VectorAnimation_mut(&mut self) -> &mut VectorAnimation { self }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for VectorAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn setAlpha(&mut self, mut alpha: i32) { Self::setAlpha(self, alpha) }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn setBasicColorP(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::setBasicColorP(self, basicColor) }
    fn changeBasicColor(&mut self, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::changeBasicColor(self, basicColor) }
    fn setScale(&mut self, mut scaleX: f32, mut scaleY: f32) { Self::setScale(self, scaleX, scaleY) }
    fn setMaxScale(&mut self, mut maxScaleX: f32, mut maxScaleY: f32) { Self::setMaxScale(self, maxScaleX, maxScaleY) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for VectorAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getAnimationSize(&mut self) -> i32 { Self::getAnimationSize(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for VectorAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


