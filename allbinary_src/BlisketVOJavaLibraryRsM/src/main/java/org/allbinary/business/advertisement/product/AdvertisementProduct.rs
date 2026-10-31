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
// package org.allbinary.business.advertisement.product
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::control::contraints::size::TwoDimensionalConstraintInterface::TwoDimensionalConstraintInterface;
use super::AdvertisementProductInterface::AdvertisementProductInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AdvertisementProduct {
    pub r#type: String,
    pub link: String,
    pub number: i32,
    pub constraint: Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>>,
}

impl AdvertisementProductInterface for AdvertisementProduct {
    fn getType(&mut self) -> String { Self::getType(self) }
    fn getLink(&mut self) -> String { Self::getLink(self) }
    fn getNumber(&mut self) -> i32 { Self::getNumber(self) }
    fn getTwoDimensionalConstraint(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> { Self::getTwoDimensionalConstraint(self) }
}

impl AdvertisementProduct {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            r#type: String::new(),
            link: String::new(),
            number: 0,
            constraint: None,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn getLink(&mut self) -> String {
        return self.link.clone();
    }
    

    pub fn getNumber(&mut self) -> i32 {
        return self.number.clone();
    }
    

    pub fn getTwoDimensionalConstraint(&mut self) -> Option<Rc<RefCell<dyn TwoDimensionalConstraintInterface>>> {
        return self.constraint.clone();
    }
}

