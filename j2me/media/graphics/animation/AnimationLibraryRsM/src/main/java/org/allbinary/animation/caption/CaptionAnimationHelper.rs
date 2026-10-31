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
use crate::org::allbinary::animation::Animation::Animation;
use crate::org::allbinary::animation::AnimationBehavior::AnimationBehavior;
use crate::org::allbinary::animation::NullAnimationFactory::NullAnimationFactory;
use crate::org::allbinary::animation::text::TextAnimation::TextAnimation;
use crate::org::allbinary::graphics::color::BasicColor::BasicColor;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::time::GameTickTimeDelayHelperFactory::GameTickTimeDelayHelperFactory;
use crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelper;
use super::CaptionAnimationHelperBase::CaptionAnimationHelperBase;
use super::CaptionAnimation::CaptionAnimation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CaptionAnimationHelper {
    pub base: CaptionAnimationHelperBase,
    pub timeHelper: Option<Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>>,
    pub textAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn>>>,
    pub captionAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
    pub animation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>,
}

impl crate::org::allbinary::graphics::paint::LocationPaintableInterface::LocationPaintableInterface for CaptionAnimationHelper {
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}

impl CaptionAnimationHelper {
    

    pub fn new_struct(mut captionAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32) -> Self {
        let mut this = Self {
            base: CaptionAnimationHelperBase::new_struct(),
            timeHelper: None,
            textAnimation: None,
            captionAnimation: None,
            animation: None,
        };
        this.timeHelper = TimeDelayHelper::new(620).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::time::TimeDelayHelper::TimeDelayHelperDyn>>);
        this.textAnimation = TextAnimation::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(), AnimationBehavior::getInstance()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::text::TextAnimation::TextAnimationDyn>>);
        this.animation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        this.captionAnimation = CaptionAnimation::new(captionAnimation.clone(), this.textAnimation.clone(), captionDx.clone(), captionDy.clone(), dx.clone(), dy.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>);
        this
    }

    pub fn new(mut captionAnimation: Option<Rc<RefCell<dyn crate::org::allbinary::animation::Animation::AnimationDyn>>>, mut captionDx: i32, mut captionDy: i32, mut dx: i32, mut dy: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(captionAnimation, captionDx, captionDy, dx, dy))))
    }
    

    pub fn isShowing(&mut self) -> bool {
        if match (&self.animation, &self.captionAnimation) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn tick(&mut self) {
        if { let java_receiver = self.timeHelper.clone(); let java_arguments = (jcall!(GameTickTimeDelayHelperFactory::getInstance()).startTime.clone(),); let java_result = jcall!(java_receiver).isTime(java_arguments.0); java_result } {
            self.animation = jcall!(NullAnimationFactory::getFactoryInstance()).getInstance(0);
        }
    }
    

    pub fn update(&mut self, mut message: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>) {
        jcall!(self.textAnimation).setBasicColorP(basicColor.clone());
        jcall!(self.textAnimation).setText(message.clone());
        self.animation = self.captionAnimation.clone();
        jcall!(self.timeHelper).setStartTimeTNT();
    }
    

    pub fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) {
        jcall!(self.animation).paintXY(graphics.clone(), x.clone(), y.clone());
    }
}
impl std::ops::Deref for CaptionAnimationHelper {
    type Target = CaptionAnimationHelperBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for CaptionAnimationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBaseDyn for CaptionAnimationHelper {
    fn as_CaptionAnimationHelperBase(&self) -> &crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase { &self.base }
    fn as_CaptionAnimationHelperBase_mut(&mut self) -> &mut crate::org::allbinary::animation::caption::CaptionAnimationHelperBase::CaptionAnimationHelperBase { &mut self.base }
    fn isShowing(&mut self) -> bool { Self::isShowing(self) }
    fn tick(&mut self) { Self::tick(self) }
    fn update(&mut self, mut message: String, mut basicColor: Option<Rc<RefCell<BasicColor>>>) { Self::update(self, message, basicColor) }
}
impl crate::org::allbinary::animation::Animation::AnimationDyn for CaptionAnimationHelper {
    fn as_Animation(&self) -> &crate::org::allbinary::animation::Animation::Animation { &self.base.base }
    fn as_Animation_mut(&mut self) -> &mut crate::org::allbinary::animation::Animation::Animation { &mut self.base.base }
    fn paintXY(&mut self, mut graphics: Option<Rc<RefCell<Graphics>>>, mut x: i32, mut y: i32) { Self::paintXY(self, graphics, x, y) }
}
impl crate::org::allbinary::image::opengles::OpenGLSurfaceChangedInterface::OpenGLSurfaceChangedInterface for CaptionAnimationHelper {
    fn set(&mut self, p1: Option<Rc<RefCell<dyn crate::javax::microedition::khronos::opengles::GL::GL>>>) { crate::org::allbinary::animation::Animation::Animation::set(crate::org::allbinary::animation::Animation::AnimationDyn::as_Animation_mut(self), p1) }
}


