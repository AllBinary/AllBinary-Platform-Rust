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
// package org.allbinary.android.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::init::Init::Init;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchButtonAndroidResources {
    pub base: Init,
}


impl TouchButtonAndroidResources {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: Init::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self) {
    }
}
impl std::ops::Deref for TouchButtonAndroidResources {
    type Target = Init;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TouchButtonAndroidResources {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::init::Init::InitDyn for TouchButtonAndroidResources {
    fn as_Init(&self) -> &crate::org::allbinary::init::Init::Init { &self.base }
    fn as_Init_mut(&mut self) -> &mut crate::org::allbinary::init::Init::Init { &mut self.base }
}
impl crate::org::allbinary::init::InitInterface::InitInterface for TouchButtonAndroidResources {
    fn init(&mut self) { Self::init(self) }
}


