// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLVUtilBase::OpenGLVUtilBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLVUtil {
    pub base: OpenGLVUtilBase,
}

thread_local! { pub static OpenGLVUtil_instance: Option<Rc<RefCell<OpenGLVUtil>>> = OpenGLVUtil::new(); }

impl OpenGLVUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: OpenGLVUtilBase::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLVUtil>>> {
        return OpenGLVUtil_instance.with(Clone::clone).clone();
    }
}
impl std::ops::Deref for OpenGLVUtil {
    type Target = OpenGLVUtilBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OpenGLVUtil {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::graphics::opengles::OpenGLVUtilBase::OpenGLVUtilBaseDyn for OpenGLVUtil {
    fn as_OpenGLVUtilBase(&self) -> &crate::org::allbinary::graphics::opengles::OpenGLVUtilBase::OpenGLVUtilBase { &self.base }
    fn as_OpenGLVUtilBase_mut(&mut self) -> &mut crate::org::allbinary::graphics::opengles::OpenGLVUtilBase::OpenGLVUtilBase { &mut self.base }
}


