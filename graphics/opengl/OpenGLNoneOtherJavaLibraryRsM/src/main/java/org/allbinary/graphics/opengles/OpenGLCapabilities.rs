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
// package org.allbinary.graphics.opengles
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLCapabilities {
    pub VERSION_1_0: String,
    pub VERSION_1_1: String,
    pub VERSION_2_0: String,
    pub VERSION_3_0: String,
    pub VERSION_3_1: String,
    pub VERSION_3_2: String,
    pub VERSION_HIGHER_THAN_EXISTS: String,
    pub glVersion: String,
    pub shaderVersion: i32,
    pub glInstanceVersion: String,
    pub maxTextureSize: i32,
}

thread_local! { pub static OpenGLCapabilities_instance: Option<Rc<RefCell<OpenGLCapabilities>>> = OpenGLCapabilities::new(); }

impl OpenGLCapabilities {
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLCapabilities>>> {
        return OpenGLCapabilities_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            VERSION_1_0: String::new(),
            VERSION_1_1: String::new(),
            VERSION_2_0: String::new(),
            VERSION_3_0: String::new(),
            VERSION_3_1: String::new(),
            VERSION_3_2: String::new(),
            VERSION_HIGHER_THAN_EXISTS: String::new(),
            glVersion: String::new(),
            shaderVersion: 0,
            glInstanceVersion: String::new(),
            maxTextureSize: 0,
        };
        this.VERSION_1_0 = String::from("1.0");
        this.VERSION_1_1 = String::from("1.1");
        this.VERSION_2_0 = String::from("2.0");
        this.VERSION_3_0 = String::from("3.0");
        this.VERSION_3_1 = String::from("3.1");
        this.VERSION_3_2 = String::from("3.2");
        this.VERSION_HIGHER_THAN_EXISTS = String::from("999.999");
        this.glVersion = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.shaderVersion = 0;
        this.glInstanceVersion = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.maxTextureSize = 64;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn initCapabilities(&mut self, mut gl: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
    }
    

    pub fn isGlExtensionDrawTexture(&mut self) -> bool {
        return false;
    }
    

    pub fn getGlVersion(&mut self) -> String {
        return self.glVersion.clone();
    }
    

    pub fn isVertexBufferObjectSupport(&mut self) -> bool {
        return false;
    }
    

    pub fn isTextureSizeValid(&mut self, mut widthAndHeight: i32) -> bool {
        return true;
    }
}

