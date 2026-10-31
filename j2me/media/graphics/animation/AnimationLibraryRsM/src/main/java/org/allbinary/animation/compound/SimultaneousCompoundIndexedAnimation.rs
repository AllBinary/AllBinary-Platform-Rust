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
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SimultaneousCompoundIndexedAnimation {
    pub base: IndexedAnimation,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for SimultaneousCompoundIndexedAnimation {
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for SimultaneousCompoundIndexedAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut frameIndex: i32) { Self::setFrame(self, frameIndex) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for SimultaneousCompoundIndexedAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for SimultaneousCompoundIndexedAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl SimultaneousCompoundIndexedAnimation {
    

    pub fn new_struct(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            animationInterfaceArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.animationInterfaceArray = animationInterfaceArray.clone();
        this
    }

    pub fn new(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterfaceArray, animationBehavior))))
    }
    

    pub fn setFrame(&mut self, mut frameIndex: i32) {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = None;
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            indexedAnimation = (self.animationInterfaceArray[(index) as usize]).clone() /* as IndexedAnimation */;
            jcall!(indexedAnimation).setFrame(frameIndex.clone());
            }
        }
    }
    

    pub fn getFrame(&mut self) -> i32 {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(0) as usize]).clone() /* as IndexedAnimation */;
        return jcall!(indexedAnimation).getFrame();
    }
    

    pub fn getSize(&mut self) -> i32 {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(0) as usize]).clone() /* as IndexedAnimation */;
        return jcall!(indexedAnimation).getSize();
    }
    

    pub fn previousFrame(&mut self) {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = None;
        {
            let mut index: i32 = (self.animationInterfaceArray.len() as i32).clone();
            while { index -= 1; index } >= 0 {
            indexedAnimation = (self.animationInterfaceArray[(index) as usize]).clone() /* as IndexedAnimation */;
            jcall!(indexedAnimation).previousFrame();
            }
        }
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
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
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        let mut size: i32 = (self.animationInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(self.animationInterfaceArray[(index) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
        }
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        let mut size: i32 = (self.animationInterfaceArray.len() as i32).clone();
        for index in (0..size) {
            jcall!(self.animationInterfaceArray[(index) as usize]).paintThreedXYZ(graphics.clone(), x.clone(), y.clone(), z.clone());
        }
    }
    

    pub fn getAnimationInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> {
        return self.animationInterfaceArray.clone();
    }
    

    pub fn setAnimationInterfaceArray(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) {
        self.animationInterfaceArray = animationInterfaceArray.clone();
    }
}
impl std::ops::Deref for SimultaneousCompoundIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SimultaneousCompoundIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for SimultaneousCompoundIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for SimultaneousCompoundIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut frameIndex: i32) { Self::setFrame(self, frameIndex) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for SimultaneousCompoundIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


