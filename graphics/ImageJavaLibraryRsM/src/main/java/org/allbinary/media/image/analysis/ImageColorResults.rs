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
use super::ColorRange::ColorRange;
use super::ColorAverage::ColorAverage;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageColorResults {
    pub colorRange: Option<Rc<RefCell<ColorRange>>>,
    pub colorAverage: Option<Rc<RefCell<ColorAverage>>>,
}


impl ImageColorResults {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            colorRange: None,
            colorAverage: None,
        };
        this.setColorRange(ColorRange::new());
        jcall!(this.getColorRange()).setMaxRed(-1);
        jcall!(this.getColorRange()).setMaxGreen(-1);
        jcall!(this.getColorRange()).setMaxBlue(-1);
        jcall!(this.getColorRange()).setMinRed(256);
        jcall!(this.getColorRange()).setMinGreen(256);
        jcall!(this.getColorRange()).setMinBlue(256);
        this.setColorAverage(ColorAverage::new());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getColorRange(&mut self) -> Option<Rc<RefCell<ColorRange>>> {
        return self.colorRange.clone();
    }
    

    pub fn setColorRange(&mut self, mut colorRange: Option<Rc<RefCell<ColorRange>>>) {
        self.colorRange = colorRange.clone();
    }
    

    pub fn getColorAverage(&mut self) -> Option<Rc<RefCell<ColorAverage>>> {
        return self.colorAverage.clone();
    }
    

    pub fn setColorAverage(&mut self, mut colorAverage: Option<Rc<RefCell<ColorAverage>>>) {
        self.colorAverage = colorAverage.clone();
    }
}

