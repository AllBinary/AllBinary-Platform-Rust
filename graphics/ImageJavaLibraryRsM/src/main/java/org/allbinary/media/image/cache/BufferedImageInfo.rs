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
// package org.allbinary.media.image.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedImageInfo {
    pub width: i32,
    pub height: i32,
    pub r#type: i32,
}


impl BufferedImageInfo {
    

    pub fn new_struct(mut width: i32, mut height: i32, mut r#type: i32) -> Self {
        let mut this = Self {
            width: 0,
            height: 0,
            r#type: 0,
        };
        this.width = width.clone();
        this.height = height.clone();
        this.r#type = r#type.clone();
        this
    }

    pub fn new(mut width: i32, mut height: i32, mut r#type: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height, r#type))))
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return self.width.clone();
    }
    

    pub fn setWidth(&mut self, mut width: i32) {
        self.width = width.clone();
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return self.height.clone();
    }
    

    pub fn setHeight(&mut self, mut height: i32) {
        self.height = height.clone();
    }
    

    pub fn getType(&mut self) -> i32 {
        return self.r#type.clone();
    }
    

    pub fn setType(&mut self, mut r#type: i32) {
        self.r#type = r#type.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("BufferedImageInfo -"));
        jcall!(stringBuffer).append(jcall!(commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.getWidth());
        jcall!(stringBuffer).append(jcall!(commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.getHeight());
        jcall!(stringBuffer).append(String::from(" Type: "));
        jcall!(stringBuffer).appendint(self.getType());
        return jcall!(stringBuffer).toString();
    }
}

