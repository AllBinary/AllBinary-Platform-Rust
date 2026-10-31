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
// package org.allbinary.business.advertisement.image
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::control::contraints::size::TwoDimensionalConstraintInterface::TwoDimensionalConstraintInterface;
use super::AdvertisementImageInterface::AdvertisementImageInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementImage {
    pub imageFileName: String,
    pub imageUrl: String,
    pub constraint: Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>,
}

impl AdvertisementImageInterface for AdvertisementImage {
    fn getFileName(&mut self) -> String { Self::getFileName(self) }
    fn getUrl(&mut self) -> String { Self::getUrl(self) }
    fn getTwoDimensionalConstraint(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> { Self::getTwoDimensionalConstraint(self) }
}

impl AdvertisementImage {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            imageFileName: String::new(),
            imageUrl: String::new(),
            constraint: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getFileName(&mut self) -> String {
        return self.imageFileName.clone();
    }
    

    pub fn getUrl(&mut self) -> String {
        return self.imageUrl.clone();
    }
    

    pub fn getTwoDimensionalConstraint(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> {
        return self.constraint.clone();
    }
}

