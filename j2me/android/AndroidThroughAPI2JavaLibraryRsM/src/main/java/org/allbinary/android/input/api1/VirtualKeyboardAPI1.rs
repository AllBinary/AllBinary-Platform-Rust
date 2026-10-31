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
// package org.allbinary.android.input.api1
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard;
use crate::org::allbinary::input::event::VirtualKeyboardEventListenerInterface::VirtualKeyboardEventListenerInterface;
use crate::android::app::Activity::Activity;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VirtualKeyboardAPI1 {
    pub base: VirtualKeyboard,
}

pub static mut VirtualKeyboardAPI1_MIN_API: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 1);
pub static mut VirtualKeyboardAPI1_MAX_API: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 2);
// impl VirtualKeyboardEventListenerInterface for VirtualKeyboardAPI1 with the methods of the trait

impl VirtualKeyboardAPI1 {
    

    pub fn new_struct(mut activity: Option<Rc<RefCell<Activity>>>) -> Self {
        let mut this = Self {
            base: VirtualKeyboard::new_struct(),
        };
        this
    }

    pub fn new(mut activity: Option<Rc<RefCell<Activity>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(activity))))
    }
}
impl std::ops::Deref for VirtualKeyboardAPI1 {
    type Target = VirtualKeyboard;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for VirtualKeyboardAPI1 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboardDyn for VirtualKeyboardAPI1 {
    fn as_VirtualKeyboard(&self) -> &crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard { &self.base }
    fn as_VirtualKeyboard_mut(&mut self) -> &mut crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboard { &mut self.base }
}


