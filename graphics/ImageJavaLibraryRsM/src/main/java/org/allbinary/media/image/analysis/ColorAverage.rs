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
// package org.allbinary.media.image.analysis
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorAverage {
    pub avgRed: f32,
    pub avgGreen: f32,
    pub avgBlue: f32,
}


impl ColorAverage {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            avgRed: 0.0_f32,
            avgGreen: 0.0_f32,
            avgBlue: 0.0_f32,
        };
        this.setAvgRed(0);
        this.setAvgGreen(0);
        this.setAvgBlue(0);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("getAvgRed(): "));
        jcall!(stringBuffer).appendfloat(self.getAvgRed());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getAvgRed(): "));
        jcall!(stringBuffer).appendfloat(self.getAvgRed());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getAvgGreen(): "));
        jcall!(stringBuffer).appendfloat(self.getAvgGreen());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn getAvgRed(&mut self) -> f32 {
        return self.avgRed.clone();
    }
    

    pub fn setAvgRed(&mut self, mut avgRed: f32) {
        self.avgRed = avgRed.clone();
    }
    

    pub fn getAvgGreen(&mut self) -> f32 {
        return self.avgGreen.clone();
    }
    

    pub fn setAvgGreen(&mut self, mut avgGreen: f32) {
        self.avgGreen = avgGreen.clone();
    }
    

    pub fn getAvgBlue(&mut self) -> f32 {
        return self.avgBlue.clone();
    }
    

    pub fn setAvgBlue(&mut self, mut avgBlue: f32) {
        self.avgBlue = avgBlue.clone();
    }
}

