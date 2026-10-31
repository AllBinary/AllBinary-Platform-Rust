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
// package org.allbinary.animation.transition
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use super::AnimationListenerInterface::AnimationListenerInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct RunAtEndAnimationListener {
    pub runnable: Option<Rc<RefCell<dyn Runnable>>>,
}

impl AnimationListenerInterface for RunAtEndAnimationListener {
    fn onPlatformAnimationStart(&mut self) { Self::onPlatformAnimationStart(self) }
    fn onPlatformAnimationEnd(&mut self) { Self::onPlatformAnimationEnd(self) }
    fn onPlatformAnimationRepeat(&mut self) { Self::onPlatformAnimationRepeat(self) }
}

impl RunAtEndAnimationListener {
    

    pub fn new_struct(mut runnable: Option<Rc<RefCell<dyn Runnable>>>) -> Self {
        let mut this = Self {
            runnable: None,
        };
        this.runnable = runnable.clone();
        this
    }

    pub fn new(mut runnable: Option<Rc<RefCell<dyn Runnable>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(runnable))))
    }
    

    pub fn onPlatformAnimationStart(&mut self) {
    }
    

    pub fn onPlatformAnimationEnd(&mut self) {
    }
    

    pub fn onPlatformAnimationRepeat(&mut self) {
    }
}
pub trait RunAtEndAnimationListenerDyn: crate::org::allbinary::animation::transition::AnimationListenerInterface::AnimationListenerInterface {
    fn as_RunAtEndAnimationListener(&self) -> &RunAtEndAnimationListener;
    fn as_RunAtEndAnimationListener_mut(&mut self) -> &mut RunAtEndAnimationListener;
}
impl RunAtEndAnimationListenerDyn for RunAtEndAnimationListener {
    fn as_RunAtEndAnimationListener(&self) -> &RunAtEndAnimationListener { self }
    fn as_RunAtEndAnimationListener_mut(&mut self) -> &mut RunAtEndAnimationListener { self }
}


