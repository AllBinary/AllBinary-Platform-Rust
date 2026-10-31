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
// package org.allbinary.animation.transition.shake
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ShakeEvent::ShakeEvent;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ShakeAnimationListener {
}


impl ShakeAnimationListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn onSmallShakeEvent(&mut self) {
    }
    

    pub fn onMediumShakeEvent(&mut self) {
    }
    

    pub fn onLargeShakeEvent(&mut self) {
    }
    

    pub fn onShakeEvent(&mut self, mut shakeEvent: Option<Rc<RefCell<ShakeEvent>>>) {
    }
    

    pub fn getType(&mut self) -> i32 {
        return 0;
    }
}
pub trait ShakeAnimationListenerDyn {
    fn as_ShakeAnimationListener(&self) -> &ShakeAnimationListener;
    fn as_ShakeAnimationListener_mut(&mut self) -> &mut ShakeAnimationListener;
    fn onSmallShakeEvent(&mut self) { ShakeAnimationListener::onSmallShakeEvent(self.as_ShakeAnimationListener_mut()) }
    fn onMediumShakeEvent(&mut self) { ShakeAnimationListener::onMediumShakeEvent(self.as_ShakeAnimationListener_mut()) }
    fn onLargeShakeEvent(&mut self) { ShakeAnimationListener::onLargeShakeEvent(self.as_ShakeAnimationListener_mut()) }
    fn onShakeEvent(&mut self, mut shakeEvent: Option<Rc<RefCell<ShakeEvent>>>) { ShakeAnimationListener::onShakeEvent(self.as_ShakeAnimationListener_mut(), shakeEvent) }
    fn getType(&mut self) -> i32 { ShakeAnimationListener::getType(self.as_ShakeAnimationListener_mut()) }
}
impl ShakeAnimationListenerDyn for ShakeAnimationListener {
    fn as_ShakeAnimationListener(&self) -> &ShakeAnimationListener { self }
    fn as_ShakeAnimationListener_mut(&mut self) -> &mut ShakeAnimationListener { self }
}


