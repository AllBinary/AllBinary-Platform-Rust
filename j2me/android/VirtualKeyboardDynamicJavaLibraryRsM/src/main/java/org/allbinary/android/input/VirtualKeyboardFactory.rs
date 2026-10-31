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
// package org.allbinary.android.input
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::AndroidInfoFactory::AndroidInfoFactory;
use crate::org::allbinary::android::input::api1::VirtualKeyboardAPI1::VirtualKeyboardAPI1;
use crate::org::allbinary::android::input::api3::VirtualKeyboardAPI3::VirtualKeyboardAPI3;
use crate::android::app::Activity::Activity;
use super::VirtualKeyboard::VirtualKeyboard;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VirtualKeyboardFactory {
}

pub static mut VirtualKeyboardFactory_virtualKeyboard: std::cell::LazyCell<Option<Rc<RefCell<dyn crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboardDyn>>>> = std::cell::LazyCell::new(|| VirtualKeyboard_NULL_VIRTUAL_KEYBOARD.with(Clone::clone).clone());

impl VirtualKeyboardFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(mut activity: Option<Rc<RefCell<Activity>>>) {
        let mut SDK_VERSION: i32 = jcall!(AndroidInfoFactory::getInstance()).getVersion();
        if SDK_VERSION <= 2 {
            unsafe { *VirtualKeyboardFactory_virtualKeyboard = VirtualKeyboardAPI1::new(activity.clone()) };
        } else {
            unsafe { *VirtualKeyboardFactory_virtualKeyboard = VirtualKeyboardAPI3::new(activity.clone()) };
        }
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::android::input::VirtualKeyboard::VirtualKeyboardDyn>>> {
        return unsafe { VirtualKeyboardFactory_virtualKeyboard.clone() }.clone();
    }
}

