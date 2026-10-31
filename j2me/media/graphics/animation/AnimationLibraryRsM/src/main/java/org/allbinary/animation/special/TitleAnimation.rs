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
// package org.allbinary.animation.special
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::animation::IndexedAnimationBehavior::IndexedAnimationBehavior;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::graphics::color::BasicColorFactory::BasicColorFactory;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;
use crate::org::allbinary::logic::ABSystemWrapper::ABSystemWrapper;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use super::SpecialAnimation::SpecialAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TitleAnimation {
    pub base: SpecialAnimation,
    pub systemWrapper: Option<Rc<RefCell<ABSystemWrapper>>>,
    pub deltaX: f32,
    pub deltaY: f32,
    pub deltaZ: f32,
    pub basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>,
    pub dxArray: Vec<i32>,
    pub dyArray: Vec<i32>,
    pub widthP: i32,
    pub animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>,
    pub sizeP: i32,
    pub y: i32,
    pub lastFrameStartTime: i64,
    pub displayInfoSingleton: Option<Rc<RefCell<DisplayInfoSingleton>>>,
    pub CLEAR_COLOR: Option<Rc<RefCell<BasicColor>>>,
}

impl crate::org::allbinary::animation::special::SpecialAnimationInterface::SpecialAnimationInterface for TitleAnimation {
    fn setLastFrame(&mut self) { Self::setLastFrame(self) }
    fn isComplete(&mut self) -> bool { Self::isComplete(self) }
}
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for TitleAnimation {
    fn reset(&mut self) { Self::reset(self) }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for TitleAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for TitleAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut ax: i32, mut ay: i32) { Self::paintXY(self, graphics, ax, ay) }
}

impl TitleAnimation {
    

    pub fn createAnimation(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut dxArray: Vec<i32>, mut dyArray: Vec<i32>, mut y: i32, mut width: i32) -> Option<Rc<RefCell<TitleAnimation>>> {
        return TitleAnimation::new(animationInterfaceArray.clone(), basicColorArray.clone(), dxArray.clone(), dyArray.clone(), y.clone(), width.clone(), IndexedAnimationBehavior::new(1, 250));
    }
    

    pub fn new_struct(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut dxArray: Vec<i32>, mut dyArray: Vec<i32>, mut y: i32, mut width: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: SpecialAnimation::new_struct(animationBehavior.clone()),
            systemWrapper: None,
            deltaX: 0.0_f32,
            deltaY: 0.0_f32,
            deltaZ: 0.0_f32,
            basicColorArray: Vec::new(),
            dxArray: Vec::new(),
            dyArray: Vec::new(),
            widthP: 0,
            animationInterfaceArray: Vec::new(),
            sizeP: 0,
            y: 0,
            lastFrameStartTime: 0_i64,
            displayInfoSingleton: None,
            CLEAR_COLOR: None,
        };
        this.systemWrapper = ABSystemWrapper::getInstance();
        this.displayInfoSingleton = DisplayInfoSingleton::getInstance();
        this.CLEAR_COLOR = jcall!(BasicColorFactory::getInstance()).CLEAR_COLOR.clone();
        // super(...) is in the struct literal above
        this.lastFrameStartTime = jcall!(this.systemWrapper).currentTimeMillis();
        this.animationInterfaceArray = animationInterfaceArray.clone();
        this.sizeP = (this.animationInterfaceArray.len() as i32).clone();
        this.basicColorArray = basicColorArray.clone();
        this.dxArray = dxArray.clone();
        this.dyArray = dyArray.clone();
        this.y = y.clone();
        this.widthP = width.clone();
        this.reset();
        this
    }

    pub fn new(mut animationInterfaceArray: Vec<Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>>, mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>, mut dxArray: Vec<i32>, mut dyArray: Vec<i32>, mut y: i32, mut width: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(animationInterfaceArray, basicColorArray, dxArray, dyArray, y, width, animationBehavior))))
    }
    

    pub fn nextFrame(&mut self) {
        let mut currentTime: i64 = jcall!(self.systemWrapper).currentTimeMillis();
        let mut totalTimeElapsed: i64 = currentTime - self.lastFrameStartTime;
        let mut indexedAnimationBehavior: Option<Rc<RefCell<IndexedAnimationBehavior>>> = (self.getAnimationBehavior()).clone() /* as IndexedAnimationBehavior */;
        if totalTimeElapsed > jcall!(indexedAnimationBehavior).frameDelayTime {
            self.previousFrame();
            self.lastFrameStartTime = currentTime.clone();
        }
        if jcall!(self.animationInterfaceArray[(0) as usize]).getFrame() == 0 {
            jcall!(indexedAnimationBehavior).loopIndex += 1;
        }
    }
    

    pub fn isComplete(&mut self) -> bool {
        let mut indexedAnimationBehavior: Option<Rc<RefCell<IndexedAnimationBehavior>>> = (self.getAnimationBehavior()).clone() /* as IndexedAnimationBehavior */;
        if jcall!(indexedAnimationBehavior).loopTotal == -1 || jcall!(indexedAnimationBehavior).loopIndex < jcall!(indexedAnimationBehavior).loopTotal || self.getFrame() != 0 {
            return false;
        } else {
            return true;
        }
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getSize();
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.animationInterfaceArray[(0) as usize]).getFrame();
    }
    

    pub fn setFrame(&mut self, mut frame: i32) {
        for index in (0..self.sizeP) {
            jcall!(self.animationInterfaceArray[(index) as usize]).setFrame(frame.clone());
        }
    }
    

    pub fn setLastFrame(&mut self) {
        { let java_arguments = (self.getSize() - 1,); self.setFrame(java_arguments.0) };
    }
    

    pub fn reset(&mut self) {
        self.setLastFrame();
        let mut indexedAnimationBehavior: Option<Rc<RefCell<IndexedAnimationBehavior>>> = ((self.getAnimationBehavior()).clone() /* as IndexedAnimationBehavior */);
        jcall!(indexedAnimationBehavior).reset();
    }
    

    pub fn previousFrame(&mut self) {
        for index in (0..self.sizeP) {
            jcall!(self.animationInterfaceArray[(index) as usize]).previousFrame();
        }
    }
    

    pub fn paintFrameXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut frame: i32, mut x: i32, mut y: i32) {
        self.setFrame(frame.clone());
        self.paintXY(graphics.clone(), x.clone(), y.clone());
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut ax: i32, mut ay: i32) {
        let mut x: i32 = 0;
        if self.widthP != i32::MIN {
            x = ((jcall!(self.displayInfoSingleton).getLastWidth() - self.widthP) / 2);
        }
        let mut deltaX: i32 = 0;
        let mut deltaY: i32 = 0;
        for index in (0..self.sizeP) {
            deltaX = self.dxArray[(index) as usize] + x;
            deltaY = self.dyArray[(index) as usize] + self.y;
            if !match (&self.basicColorArray[(index) as usize], &self.CLEAR_COLOR) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.basicSetColorUtil).setBasicColorP(graphics.clone(), self.basicColorArray[(index) as usize].clone());
            }
            jcall!(self.animationInterfaceArray[(index) as usize]).paintXY(graphics.clone(), deltaX.clone(), deltaY.clone());
        }
    }
}
impl std::ops::Deref for TitleAnimation {
    type Target = SpecialAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TitleAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimationDyn for TitleAnimation {
    fn as_SpecialAnimation(&self) -> &crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimation { &self.base }
    fn as_SpecialAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::special::SpecialAnimation::SpecialAnimation { &mut self.base }
    fn setLastFrame(&mut self) { Self::setLastFrame(self) }
    fn isComplete(&mut self) -> bool { Self::isComplete(self) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for TitleAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut ax: i32, mut ay: i32) { Self::paintXY(self, graphics, ax, ay) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for TitleAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base.base }
    fn reset(&mut self) { Self::reset(self) }
    fn setFrame(&mut self, mut frame: i32) { Self::setFrame(self, frame) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for TitleAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


