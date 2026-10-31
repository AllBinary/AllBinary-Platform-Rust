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
use super::MathVector::MathVector;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct YAxisMathVector {
    pub base: MathVector,
}


impl YAxisMathVector {
    

    pub fn new_struct(mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: i32) -> Self {
        let mut this = Self {
            base: MathVector::new_struct2(magnitudeBasicDecimal.clone(), direction.clone()),
        };
        // super(...) is in the struct literal above
        this.calculate();
        this
    }

    pub fn new(mut magnitudeBasicDecimal: Option<Rc<RefCell<BasicDecimal>>>, mut direction: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(magnitudeBasicDecimal, direction))))
    }
    

    pub fn getResult(&mut self) -> Option<Rc<RefCell<BasicDecimal>>> {
        return self.base.resultBasicDecimal.clone();
    }
    

    pub fn calculate(&mut self) {
        let mut result: i64 = YAxisMathVectorUtil::YAxisMathVectorUtil::calculate(jcall!(self.getMagnitude()).getUnscaled(), self.getDirection());
        self.resultBasicDecimal = BasicDecimal::new(result.clone());
    }
}
impl std::ops::Deref for YAxisMathVector {
    type Target = MathVector;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for YAxisMathVector {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



