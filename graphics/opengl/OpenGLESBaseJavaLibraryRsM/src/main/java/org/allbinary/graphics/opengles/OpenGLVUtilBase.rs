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
// use crate::javax::microedition::khronos::opengles::GL10::GL10;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLVUtilBase {
}


impl OpenGLVUtilBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn glTexEnvix(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glTexParameterix(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut pname: i32, mut param: i32) {
    }
    

    pub fn glGenerateMipmap(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut textureId: i32, mut target: i32, mut generateMipMap: bool) {
    }
}
pub trait OpenGLVUtilBaseDyn {
    fn as_OpenGLVUtilBase(&self) -> &OpenGLVUtilBase;
    fn as_OpenGLVUtilBase_mut(&mut self) -> &mut OpenGLVUtilBase;
    fn glTexEnvix(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut pname: i32, mut param: i32) { OpenGLVUtilBase::glTexEnvix(self.as_OpenGLVUtilBase_mut(), gl, target, pname, param) }
    fn glTexParameterix(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut target: i32, mut pname: i32, mut param: i32) { OpenGLVUtilBase::glTexParameterix(self.as_OpenGLVUtilBase_mut(), gl, target, pname, param) }
    fn glGenerateMipmap(&mut self, mut gl: Option<Rc<RefCell<dyn GL10>>>, mut textureId: i32, mut target: i32, mut generateMipMap: bool) { OpenGLVUtilBase::glGenerateMipmap(self.as_OpenGLVUtilBase_mut(), gl, textureId, target, generateMipMap) }
}
impl OpenGLVUtilBaseDyn for OpenGLVUtilBase {
    fn as_OpenGLVUtilBase(&self) -> &OpenGLVUtilBase { self }
    fn as_OpenGLVUtilBase_mut(&mut self) -> &mut OpenGLVUtilBase { self }
}


