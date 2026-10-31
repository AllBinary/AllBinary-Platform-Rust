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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use super::CompoundAnimationInterface::CompoundAnimationInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CompoundIndexedAnimation {
    pub base: IndexedAnimation,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>,
}

// impl CompoundAnimationInterface for CompoundIndexedAnimation with the methods of the trait
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for CompoundIndexedAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for CompoundIndexedAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CompoundIndexedAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CompoundIndexedAnimation {
    

    pub fn new_struct(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            circularIndexUtil: None,
            animationInterfaceArray: Vec::new(),
        };
        // super(...) is in the struct literal above
        this.animationInterfaceArray = animationInterfaceArray.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance((this.animationInterfaceArray.len() as i32).clone());
        this
    }

    pub fn new(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterfaceArray, animationBehavior))))
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as IndexedAnimation */;
        jcall!(indexedAnimation).setFrame(index.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as IndexedAnimation */;
        return jcall!(indexedAnimation).getFrame();
    }
    

    pub fn getSize(&mut self) -> i32 {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as IndexedAnimation */;
        return jcall!(indexedAnimation).getSize();
    }
    

    pub fn previousFrame(&mut self) {
        let mut indexedAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> = (self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as IndexedAnimation */;
        jcall!(indexedAnimation).previousFrame();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).nextFrame();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).paintXY(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).paintThreedXYZ(graphics.clone(), x.clone(), y.clone(), z.clone());
    }
    

    pub fn getCurrentAnimation(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>> {
        return (self.animationInterfaceArray[(jcall!(self.circularIndexUtil).getIndex()) as usize]).clone() /* as IndexedAnimation */;
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
    

    pub fn getAnimationInterfaceArray(&mut self) -> Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>> {
        return self.animationInterfaceArray.clone();
    }
    

    pub fn setAnimationInterfaceArray(&mut self, mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>>) {
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
impl std::ops::Deref for CompoundIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CompoundIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CompoundIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for CompoundIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CompoundIndexedAnimation {
    fn set(&mut self, mut gl: Option<Rc<RefCell<dyn GL>>>) { Self::set(self, gl) }
}


