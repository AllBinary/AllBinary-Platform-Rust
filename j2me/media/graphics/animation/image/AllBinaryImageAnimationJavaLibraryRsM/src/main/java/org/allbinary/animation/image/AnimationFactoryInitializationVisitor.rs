// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::math::PositionStrings::PositionStrings;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnimationFactoryInitializationVisitor {
    pub width: i32,
    pub height: i32,
    pub dx: i32,
    pub dy: i32,
    pub originalDx: i32,
    pub originalDy: i32,
}


impl AnimationFactoryInitializationVisitor {

    pub fn new_struct() -> Self {
        let mut this = Self {
            width: 0,
            height: 0,
            dx: 0,
            dy: 0,
            originalDx: 0,
            originalDy: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn visit(&mut self) {
    }
    

    pub fn toString(&mut self) -> String {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut commonLabels: Option<Rc<RefCell<CommonLabels>>> = CommonLabels::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(PositionStrings::getInstance()).DX_LABEL.clone()).appendfloat(((self.dx) as f32));
        jcall!(stringBuffer).append(jcall!(PositionStrings::getInstance()).DY_LABEL.clone()).appendfloat(((self.dy) as f32));
        jcall!(stringBuffer).append(jcall!(commonLabels).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.width.clone());
        jcall!(stringBuffer).append(jcall!(commonSeps).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(commonLabels).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.height.clone());
        return jcall!(stringBuffer).toString();
    }
}

