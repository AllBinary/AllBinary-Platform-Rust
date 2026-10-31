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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Rectangle {
    pub point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub width: i32,
    pub height: i32,
}


impl Rectangle {
    

    pub fn new_struct(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut width: i32, mut height: i32) -> Self {
        let mut this = Self {
            point: None,
            width: 0,
            height: 0,
        };
        this.point = point.clone();
        this.width = width.clone();
        this.height = height.clone();
        this
    }

    pub fn new(mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut width: i32, mut height: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(point, width, height))))
    }
    

    pub fn getPoint(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.point.clone();
    }
    

    pub fn setPoint(&mut self, mut point: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.point = point.clone();
    }
    

    pub fn getMaxX(&mut self) -> i32 {
        return jcall!(self.point).getX() + self.width;
    }
    

    pub fn getMaxY(&mut self) -> i32 {
        return jcall!(self.point).getY() + self.height;
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
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.getPoint()).toString());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).WIDTH_LABEL.clone());
        jcall!(stringBuffer).appendint(self.getWidth());
        jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
        jcall!(stringBuffer).append(jcall!(CommonLabels::getInstance()).HEIGHT_LABEL.clone());
        jcall!(stringBuffer).appendint(self.getHeight());
        jcall!(stringBuffer).append(String::from(" MaxX: "));
        jcall!(stringBuffer).appendint(self.getMaxX());
        jcall!(stringBuffer).append(String::from(" MaxY: "));
        jcall!(stringBuffer).appendint(self.getMaxY());
        return jcall!(stringBuffer).toString();
    }
}

