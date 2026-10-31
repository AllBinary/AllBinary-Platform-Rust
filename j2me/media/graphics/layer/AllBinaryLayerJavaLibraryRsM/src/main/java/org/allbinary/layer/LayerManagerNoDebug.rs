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
// package org.allbinary.layer
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::LayerManagerLoggingBase::LayerManagerLoggingBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LayerManagerNoDebug {
    pub base: LayerManagerLoggingBase,
}

thread_local! { pub static LayerManagerNoDebug_instance: Option<Rc<RefCell<LayerManagerNoDebug>>> = LayerManagerNoDebug::new(); }

impl LayerManagerNoDebug {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: LayerManagerLoggingBase::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<LayerManagerNoDebug>>> {
        return LayerManagerNoDebug_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for LayerManagerNoDebug {
    type Target = LayerManagerLoggingBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for LayerManagerNoDebug {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBaseDyn for LayerManagerNoDebug {
    fn as_LayerManagerLoggingBase(&self) -> &crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBase { &self.base }
    fn as_LayerManagerLoggingBase_mut(&mut self) -> &mut crate::org::allbinary::layer::LayerManagerLoggingBase::LayerManagerLoggingBase { &mut self.base }
}


