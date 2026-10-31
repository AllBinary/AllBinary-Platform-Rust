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
// package org.allbinary.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct VectorInfo {
    pub width: i32,
    pub height: i32,
    pub points: Vec<Vec<i32>>,
    pub totalFrames: i32,
}


impl VectorInfo {
    

    pub fn new_struct(mut width: i32, mut height: i32, mut points: Vec<Vec<i32>>, mut totalFrames: i32) -> Self {
        let mut this = Self {
            width: 0,
            height: 0,
            points: Vec::new(),
            totalFrames: 0,
        };
        this.width = width.clone();
        this.height = height.clone();
        this.points = points.clone();
        this.totalFrames = totalFrames.clone();
        this
    }

    pub fn new(mut width: i32, mut height: i32, mut points: Vec<Vec<i32>>, mut totalFrames: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(width, height, points, totalFrames))))
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
    

    pub fn getPoints(&mut self) -> Vec<Vec<i32>> {
        return self.points.clone();
    }
    

    pub fn setPoints(&mut self, mut points: Vec<Vec<i32>>) {
        self.points = points.clone();
    }
    

    pub fn getTotalFrames(&mut self) -> i32 {
        return self.totalFrames.clone();
    }
    

    pub fn setTotalFrames(&mut self, mut totalFrames: i32) {
        self.totalFrames = totalFrames.clone();
    }
}

