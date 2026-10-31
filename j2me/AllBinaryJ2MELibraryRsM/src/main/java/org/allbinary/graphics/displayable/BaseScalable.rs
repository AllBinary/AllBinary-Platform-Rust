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
// package org.allbinary.graphics.displayable
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ScalableListener::ScalableListener;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseScalable {
}

impl ScalableListener for BaseScalable {
    fn getRatio(&mut self, mut ratio: f32) -> f32 { Self::getRatio(self, ratio) }
    fn getLeft(&mut self, mut xOffset: i32) -> i32 { Self::getLeft(self, xOffset) }
    fn getTop(&mut self, mut yOffset: i32) -> i32 { Self::getTop(self, yOffset) }
    fn scale(&mut self, mut ratio: f32) { Self::scale(self, ratio) }
}

impl BaseScalable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getRatio(&mut self, mut ratio: f32) -> f32 {
        return 1.0_f32;
    }
    

    pub fn getLeft(&mut self, mut xOffset: i32) -> i32 {
        return -1;
    }
    

    pub fn getTop(&mut self, mut yOffset: i32) -> i32 {
        return -1;
    }
    

    pub fn scale(&mut self, mut ratio: f32) {
    }
}
pub trait BaseScalableDyn: crate::org::allbinary::graphics::displayable::ScalableListener::ScalableListener {
    fn as_BaseScalable(&self) -> &BaseScalable;
    fn as_BaseScalable_mut(&mut self) -> &mut BaseScalable;
}
impl BaseScalableDyn for BaseScalable {
    fn as_BaseScalable(&self) -> &BaseScalable { self }
    fn as_BaseScalable_mut(&mut self) -> &mut BaseScalable { self }
}


