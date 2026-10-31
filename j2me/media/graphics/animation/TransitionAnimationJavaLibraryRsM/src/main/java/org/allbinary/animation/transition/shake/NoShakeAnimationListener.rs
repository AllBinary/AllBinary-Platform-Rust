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
use super::ShakeAnimationListener::ShakeAnimationListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NoShakeAnimationListener {
    pub base: ShakeAnimationListener,
}

thread_local! { pub static NoShakeAnimationListener_NO_SHAKE_ANIMATION_LISTENER: Option<Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>> = NoShakeAnimationListener::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn>>); }

impl NoShakeAnimationListener {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ShakeAnimationListener::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for NoShakeAnimationListener {
    type Target = ShakeAnimationListener;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for NoShakeAnimationListener {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListenerDyn for NoShakeAnimationListener {
    fn as_ShakeAnimationListener(&self) -> &crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListener { &self.base }
    fn as_ShakeAnimationListener_mut(&mut self) -> &mut crate::org::allbinary::animation::transition::shake::ShakeAnimationListener::ShakeAnimationListener { &mut self.base }
}


