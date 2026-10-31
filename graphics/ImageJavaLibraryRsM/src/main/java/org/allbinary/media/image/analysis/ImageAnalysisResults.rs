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
use super::ImageColorRangeResults::ImageColorRangeResults;
use super::ImageColorResults::ImageColorResults;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageAnalysisResults {
    pub imageColorRangeResults: Option<Rc<RefCell<ImageColorRangeResults>>>,
    pub imageColorResults: Option<Rc<RefCell<ImageColorResults>>>,
    pub averageAmbientLevel: f32,
}


impl ImageAnalysisResults {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageColorRangeResults: None,
            imageColorResults: None,
            averageAmbientLevel: 0.0_f32,
        };
        this.imageColorRangeResults = ImageColorRangeResults::new();
        this.imageColorResults = ImageColorResults::new();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getImageColorRangeResults(&mut self) -> Option<Rc<RefCell<ImageColorRangeResults>>> {
        return self.imageColorRangeResults.clone();
    }
    

    pub fn setImageColorRangeResults(&mut self, mut imageColorRangeResults: Option<Rc<RefCell<ImageColorRangeResults>>>) {
        self.imageColorRangeResults = imageColorRangeResults.clone();
    }
    

    pub fn getAverageAmbientLevel(&mut self) -> f32 {
        return self.averageAmbientLevel.clone();
    }
    

    pub fn setAverageAmbientLevel(&mut self, mut averageAmbientLevel: f32) {
        self.averageAmbientLevel = averageAmbientLevel.clone();
    }
    

    pub fn getImageColorResults(&mut self) -> Option<Rc<RefCell<ImageColorResults>>> {
        return self.imageColorResults.clone();
    }
    

    pub fn setImageColorResults(&mut self, mut imageColorResults: Option<Rc<RefCell<ImageColorResults>>>) {
        self.imageColorResults = imageColorResults.clone();
    }
}

