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
// package org.allbinary.animation.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Image::Image;
use crate::org::allbinary::math::AngleInfo::AngleInfo;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageArrayRotationAnimationInfo {
    pub imageArray: Vec<Option<Rc<RefCell<Image>>>>,
    pub angleIncrement: i32,
    pub totalAngle: i32,
    pub dx: i32,
    pub dy: i32,
}


impl ImageArrayRotationAnimationInfo {
    

    pub fn new_struct(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleIncrement: i32, mut totalAngle: i32, mut dx: i32, mut dy: i32) -> Self {
        let mut this = Self {
            imageArray: Vec::new(),
            angleIncrement: 0,
            totalAngle: 0,
            dx: 0,
            dy: 0,
        };
        this.imageArray = imageArray.clone();
        this.angleIncrement = angleIncrement.clone();
        this.totalAngle = totalAngle.clone();
        this.dx = dx.clone();
        this.dy = dy.clone();
        this
    }

    pub fn new(mut imageArray: Vec<Option<Rc<RefCell<Image>>>>, mut angleIncrement: i32, mut totalAngle: i32, mut dx: i32, mut dy: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(imageArray, angleIncrement, totalAngle, dx, dy))))
    }
    

    pub fn setImageArray(&mut self, mut imageArray: Vec<Option<Rc<RefCell<Image>>>>) {
        self.imageArray = imageArray.clone();
    }
    

    pub fn getImageArray(&mut self) -> Vec<Option<Rc<RefCell<Image>>>> {
        return self.imageArray.clone();
    }
    

    pub fn getAngleInfoP(&mut self) -> Option<Rc<RefCell<AngleInfo>>> {
        return AngleInfo::getInstance(((self.angleIncrement) as i16));
    }
    

    pub fn setTotalAngle(&mut self, mut totalAngle: i32) {
        self.totalAngle = totalAngle.clone();
    }
    

    pub fn getTotalAngle(&mut self) -> i32 {
        return self.totalAngle.clone();
    }
    

    pub fn setDx(&mut self, mut dx: i32) {
        self.dx = dx.clone();
    }
    

    pub fn getDx(&mut self) -> i32 {
        return self.dx.clone();
    }
    

    pub fn setDy(&mut self, mut dy: i32) {
        self.dy = dy.clone();
    }
    

    pub fn getDy(&mut self) -> i32 {
        return self.dy.clone();
    }
}

