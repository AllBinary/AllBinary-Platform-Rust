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
// package org.allbinary.logic.math.vector
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::math::BasicDecimal::BasicDecimal;
use super::MathVectorInterface::MathVectorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MathVector {
    pub magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
    pub direction: i32,
    pub resultBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>,
}

impl MathVectorInterface for MathVector {
    fn getDirection(&mut self) -> i32 { Self::getDirection(self) }
    fn getMagnitude(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { Self::getMagnitude(self) }
    fn getResult(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { Self::getResult(self) }
}

impl MathVector {
    

    pub fn new_struct(mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: i32) -> Self {
        let mut this = Self {
            magnitudeBasicDecimal: None,
            direction: 0,
            resultBasicDecimal: None,
        };
        this.magnitudeBasicDecimal = magnitudeBasicDecimal.clone();
        this.direction = direction.clone();
        this
    }

    pub fn new(mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(magnitudeBasicDecimal, direction))))
    }
    

    pub fn getMagnitude(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.magnitudeBasicDecimal.clone();
    }
    

    pub fn getDirection(&mut self) -> i32 {
        return self.direction.clone();
    }
    

    pub fn getResult(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.resultBasicDecimal.clone();
    }
}
pub trait MathVectorDyn {
    fn as_MathVector(&self) -> &MathVector;
    fn as_MathVector_mut(&mut self) -> &mut MathVector;
    fn getMagnitude(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { MathVector::getMagnitude(self.as_MathVector_mut()) }
    fn getDirection(&mut self) -> i32 { MathVector::getDirection(self.as_MathVector_mut()) }
    fn getResult(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> { MathVector::getResult(self.as_MathVector_mut()) }
}
impl MathVectorDyn for MathVector {
    fn as_MathVector(&self) -> &MathVector { self }
    fn as_MathVector_mut(&mut self) -> &mut MathVector { self }
}


