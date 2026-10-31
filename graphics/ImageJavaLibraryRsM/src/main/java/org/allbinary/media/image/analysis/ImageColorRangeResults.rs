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
use super::ColorRangeInterface::ColorRangeInterface;
use super::ColorRange::ColorRange;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageColorRangeResults {
    pub colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>,
    pub matchingPixelsChecked: i64,
    pub totalPixelsChecked: i64,
}


impl ImageColorRangeResults {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            colorRangeInterface: None,
            matchingPixelsChecked: 0_i64,
            totalPixelsChecked: 0_i64,
        };
        this.setColorRangeInterface((ColorRange::new()).clone() /* as ColorRangeInterface */);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getColorRangeInterface(&mut self) -> Option<Rc<RefCell<dyn ColorRangeInterface>>> {
        return self.colorRangeInterface.clone();
    }
    

    pub fn setColorRangeInterface(&mut self, mut colorRangeInterface: Option<Rc<RefCell<dyn ColorRangeInterface>>>) {
        self.colorRangeInterface = colorRangeInterface.clone();
    }
    

    pub fn getMatchingPixelsChecked(&mut self) -> i64 {
        return self.matchingPixelsChecked.clone();
    }
    

    pub fn setMatchingPixelsChecked(&mut self, mut matchingPixelsChecked: i64) {
        self.matchingPixelsChecked = matchingPixelsChecked.clone();
    }
    

    pub fn addMatchingPixelsChecked(&mut self) {
        self.matchingPixelsChecked += 1;
    }
    

    pub fn getTotalPixelsChecked(&mut self) -> i64 {
        return self.totalPixelsChecked.clone();
    }
    

    pub fn addTotalPixelsChecked(&mut self) {
        self.totalPixelsChecked += 1;
    }
    

    pub fn setTotalPixelsChecked(&mut self, mut totalPixelsChecked: i64) {
        self.totalPixelsChecked = totalPixelsChecked.clone();
    }
    

    pub fn getPercent(&mut self) -> f32 {
        return (((self.getMatchingPixelsChecked()) as f32) / ((self.getTotalPixelsChecked()) as f32));
    }
}

