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
// package org.allbinary.animation.caption
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Graphics::Graphics;
// use crate::javax::microedition::media::Player::Player;
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::AnimationEventHandler::AnimationEventHandler;
use crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation;
use crate::org::allbinary::logic::math::PrimitiveIntUtil::PrimitiveIntUtil;
use crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObject;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CaptionIndexedAnimation {
    pub base: IndexedAnimation,
    pub animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub movieIndexedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>,
    pub captionDx: i32,
    pub captionDy: i32,
    pub dx: i32,
    pub dy: i32,
    pub time: i32,
    pub END_EVENT: Option<Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>>,
    pub timeDelayHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub player: Option<Rc<RefCell<Player>>>,
}

impl crate::org::allbinary::animation::AnimationInterface::AnimationInterface for CaptionIndexedAnimation {
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimationInterface::IndexedAnimationInterface for CaptionIndexedAnimation {
    fn reset(&mut self) { self.base.reset() }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
}
impl crate::org::allbinary::animation::SequentialAnimationInterface::SequentialAnimationInterface for CaptionIndexedAnimation {
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
}
impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CaptionIndexedAnimation {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CaptionIndexedAnimation {
    

    pub fn new_struct(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut movieIndexedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>, mut player: Option<Rc<RefCell<Player>>>, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32, mut time: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Self {
        let mut this = Self {
            base: IndexedAnimation::new_struct(animationBehavior.clone()),
            animationInterface: None,
            movieIndexedAnimationInterface: None,
            captionDx: 0,
            captionDy: 0,
            dx: 0,
            dy: 0,
            time: 0,
            END_EVENT: None,
            timeDelayHelper: None,
            player: None,
        };
        // super(...) is in the struct literal above
        this.animationInterface = animationInterface.clone();
        this.movieIndexedAnimationInterface = movieIndexedAnimationInterface.clone();
        this.captionDx = captionDx.clone();
        this.captionDy = captionDy.clone();
        this.dx = dx.clone();
        this.dy = dy.clone();
        this.time = time.clone();
        let mut timePerFrame: i32 = this.time / this.getSize();
        this.timeDelayHelper = TimeDelayHelper::new(timePerFrame.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.player = player.clone();
        jcall!(this.player).start();
        this
    }

    pub fn new(mut animationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut movieIndexedAnimationInterface: Option<Rc<RefCell<dyn crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn>>>, mut player: Option<Rc<RefCell<Player>>>, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32, mut time: i32, mut animationBehavior: Option<Rc<RefCell<dyn crate::org::allbinary::animation::AnimationBehavior::AnimationBehaviorDyn>>>) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(animationInterface, movieIndexedAnimationInterface, player, captionDx, captionDy, dx, dy, time, animationBehavior))));
            this.END_EVENT = AllBinaryEventObject::new((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone())).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::util::event::AllBinaryEventObject::AllBinaryEventObjectDyn>>);
        this
    }
    

    // throws Exception
    pub fn nextFrame(&mut self) {
        if { let java_receiver = self.timeDelayHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
            jcall!(self.movieIndexedAnimationInterface).nextFrame();
        }
        if self.isLastFrame() {
            jcall!(AnimationEventHandler::getInstance()).fireEvent(self.END_EVENT.clone());
        }
    }
    

    pub fn isLastFrame(&mut self) -> bool {
        if jcall!(self.movieIndexedAnimationInterface).getFrame() == self.getSize() - 1 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn previousFrame(&mut self) {
        jcall!(self.movieIndexedAnimationInterface).previousFrame();
    }
    

    pub fn setFrame(&mut self, mut index: i32) {
        jcall!(self.movieIndexedAnimationInterface).setFrame(index.clone());
    }
    

    pub fn getFrame(&mut self) -> i32 {
        return jcall!(self.movieIndexedAnimationInterface).getFrame();
    }
    

    pub fn getSize(&mut self) -> i32 {
        return jcall!(self.movieIndexedAnimationInterface).getSize();
    }
    

    pub fn setSequence(&mut self, mut sequence: Vec<i32>) {
    }
    

    pub fn getSequence(&mut self) -> Vec<i32> {
        return PrimitiveIntUtil::getArrayInstance();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.animationInterface).paintXY(graphics.clone(), x + self.captionDx, y + self.captionDy);
        jcall!(self.movieIndexedAnimationInterface).paintXY(graphics.clone(), x + self.captionDx + self.dx, y + self.captionDy + self.dy);
    }
    

    pub fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) {
        jcall!(self.animationInterface).paintThreedXYZ(graphics.clone(), x + self.captionDx, y + self.captionDy, z.clone());
        jcall!(self.movieIndexedAnimationInterface).paintThreedXYZ(graphics.clone(), x + self.captionDx + self.dx, y + self.captionDy + self.dy, z.clone());
    }
}
impl std::ops::Deref for CaptionIndexedAnimation {
    type Target = IndexedAnimation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CaptionIndexedAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::Animation::AnimationDyn for CaptionIndexedAnimation {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn nextFrame(&mut self) { Self::nextFrame(self) }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
    fn paintThreedXYZ(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32, mut z: i32) { Self::paintThreedXYZ(self, graphics, x, y, z) }
}
impl crate::org::allbinary::animation::IndexedAnimation::IndexedAnimationDyn for CaptionIndexedAnimation {
    fn as_IndexedAnimation(&self) -> &crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &self.base }
    fn as_IndexedAnimation_mut(&mut self) -> &mut crate::org::allbinary::animation::IndexedAnimation::IndexedAnimation { &mut self.base }
    fn setFrame(&mut self, mut index: i32) { Self::setFrame(self, index) }
    fn getFrame(&mut self) -> i32 { Self::getFrame(self) }
    fn getSize(&mut self) -> i32 { Self::getSize(self) }
    fn previousFrame(&mut self) { Self::previousFrame(self) }
    fn isLastFrame(&mut self) -> bool { Self::isLastFrame(self) }
    fn setSequence(&mut self, mut sequence: Vec<i32>) { Self::setSequence(self, sequence) }
    fn getSequence(&mut self) -> Vec<i32> { Self::getSequence(self) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CaptionIndexedAnimation {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


