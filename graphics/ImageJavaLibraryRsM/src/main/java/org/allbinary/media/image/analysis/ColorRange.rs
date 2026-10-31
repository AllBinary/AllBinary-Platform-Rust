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
// use crate::java::awt::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorRange {
    pub minRed: i32,
    pub maxRed: i32,
    pub minGreen: i32,
    pub maxGreen: i32,
    pub minBlue: i32,
    pub maxBlue: i32,
}

impl ColorRangeInterface for ColorRange {
    fn getMinRed(&mut self) -> i32 { Self::getMinRed(self) }
    fn setMinRed(&mut self, mut minRed: i32) { Self::setMinRed(self, minRed) }
    fn getMaxRed(&mut self) -> i32 { Self::getMaxRed(self) }
    fn setMaxRed(&mut self, mut maxRed: i32) { Self::setMaxRed(self, maxRed) }
    fn getMinGreen(&mut self) -> i32 { Self::getMinGreen(self) }
    fn setMinGreen(&mut self, mut minGreen: i32) { Self::setMinGreen(self, minGreen) }
    fn getMaxGreen(&mut self) -> i32 { Self::getMaxGreen(self) }
    fn setMaxGreen(&mut self, mut maxGreen: i32) { Self::setMaxGreen(self, maxGreen) }
    fn getMinBlue(&mut self) -> i32 { Self::getMinBlue(self) }
    fn setMinBlue(&mut self, mut minBlue: i32) { Self::setMinBlue(self, minBlue) }
    fn getMaxBlue(&mut self) -> i32 { Self::getMaxBlue(self) }
    fn setMaxBlue(&mut self, mut maxBlue: i32) { Self::setMaxBlue(self, maxBlue) }
    fn isInRange(&mut self, mut color: Option<Rc<RefCell<Color>>>) -> bool { Self::isInRange(self, color) }
}

impl ColorRange {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            minRed: 0,
            maxRed: 0,
            minGreen: 0,
            maxGreen: 0,
            minBlue: 0,
            maxBlue: 0,
        };
        this.minRed = 0;
        this.maxRed = 0;
        this.minGreen = 0;
        this.maxGreen = 0;
        this.minBlue = 0;
        this.maxBlue = 0;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getMinRed(&mut self) -> i32 {
        return self.minRed.clone();
    }
    

    pub fn setMinRed(&mut self, mut minRed: i32) {
        self.minRed = minRed.clone();
    }
    

    pub fn getMaxRed(&mut self) -> i32 {
        return self.maxRed.clone();
    }
    

    pub fn setMaxRed(&mut self, mut maxRed: i32) {
        self.maxRed = maxRed.clone();
    }
    

    pub fn getMinGreen(&mut self) -> i32 {
        return self.minGreen.clone();
    }
    

    pub fn setMinGreen(&mut self, mut minGreen: i32) {
        self.minGreen = minGreen.clone();
    }
    

    pub fn getMaxGreen(&mut self) -> i32 {
        return self.maxGreen.clone();
    }
    

    pub fn setMaxGreen(&mut self, mut maxGreen: i32) {
        self.maxGreen = maxGreen.clone();
    }
    

    pub fn getMinBlue(&mut self) -> i32 {
        return self.minBlue.clone();
    }
    

    pub fn setMinBlue(&mut self, mut minBlue: i32) {
        self.minBlue = minBlue.clone();
    }
    

    pub fn getMaxBlue(&mut self) -> i32 {
        return self.maxBlue.clone();
    }
    

    pub fn setMaxBlue(&mut self, mut maxBlue: i32) {
        self.maxBlue = maxBlue.clone();
    }
    

    pub fn isInRange(&mut self, mut color: Option<Rc<RefCell<Color>>>) -> bool {
        if jcall!(color).getRed() >= self.getMinRed() && jcall!(color).getRed() <= self.getMaxRed() && jcall!(color).getGreen() >= self.getMinGreen() && jcall!(color).getGreen() <= self.getMaxGreen() && jcall!(color).getBlue() >= self.getMinBlue() && jcall!(color).getBlue() <= self.getMaxBlue() {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("getMinRed(): "));
        jcall!(stringBuffer).appendint(self.getMinRed());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getMaxRed(): "));
        jcall!(stringBuffer).appendint(self.getMaxRed());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getMinGreen(): "));
        jcall!(stringBuffer).appendint(self.getMinGreen());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getMaxGreen(): "));
        jcall!(stringBuffer).appendint(self.getMaxGreen());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getMinBlue(): "));
        jcall!(stringBuffer).appendint(self.getMinGreen());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).NEW_LINE.clone());
        jcall!(stringBuffer).append(String::from("getMaxBlue(): "));
        jcall!(stringBuffer).appendint(self.getMaxBlue());
        return jcall!(stringBuffer).toString();
    }
}

