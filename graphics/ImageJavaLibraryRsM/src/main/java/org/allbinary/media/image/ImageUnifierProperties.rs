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
// package org.allbinary.media.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ImageUnifierCell::ImageUnifierCell;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImageUnifierProperties {
    pub rows: Option<i32>,
    pub columns: Option<i32>,
    pub imageUnifierCell: Option<Rc<RefCell<ImageUnifierCell>>>,
}


impl ImageUnifierProperties {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            rows: None,
            columns: None,
            imageUnifierCell: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getWidth(&mut self) -> i32 {
        return ((self.getColumns()).unwrap() as i32) * ((jcall!(self.getImageUnifierCell()).getWidth()).unwrap() as i32);
    }
    

    pub fn getHeight(&mut self) -> i32 {
        return ((self.getRows()).unwrap() as i32) * ((jcall!(self.getImageUnifierCell()).getHeight()).unwrap() as i32);
    }
    

    pub fn getRows(&mut self) -> Option<i32> {
        return self.rows.clone();
    }
    

    pub fn setRows(&mut self, mut rows: Option<i32>) {
        self.rows = rows.clone();
    }
    

    pub fn getColumns(&mut self) -> Option<i32> {
        return self.columns.clone();
    }
    

    pub fn setColumns(&mut self, mut columns: Option<i32>) {
        self.columns = columns.clone();
    }
    

    pub fn getImageUnifierCell(&mut self) -> Option<Rc<RefCell<ImageUnifierCell>>> {
        return self.imageUnifierCell.clone();
    }
    

    pub fn setImageUnifierCell(&mut self, mut imageUnifierCell: Option<Rc<RefCell<ImageUnifierCell>>>) {
        self.imageUnifierCell = imageUnifierCell.clone();
    }
}

