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
// package org.allbinary.logic.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicDecimal {
    pub units: i64,
    pub scaled: i32,
    pub factor: i32,
    pub factorValue: i32,
}

thread_local! { pub static BasicDecimal_ZERO_BIGDECIMAL: Option<Rc<RefCell<BasicDecimal>>> = BasicDecimal::new(0); }

impl BasicDecimal {
    

    pub fn createBigDecimal(mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) -> Option<Rc<RefCell<BasicDecimal>>> {
        return BasicDecimal::new(jcall!(bigDecimal).getUnscaled());
    }
    

    pub fn new_struct(mut units: i64) -> Self {
        let mut this = Self {
            units: 0_i64,
            scaled: 0,
            factor: 0,
            factorValue: 0,
        };
        this.factor = jcall!(ScaleFactorFactory::ScaleFactorFactory::getInstance()).DEFAULT_SCALE_FACTOR.clone();
        this.factorValue = jcall!(ScaleFactorFactory::ScaleFactorFactory::getInstance()).DEFAULT_SCALE_VALUE.clone();
        this.units = units.clone();
        this.updateScaled();
        this
    }

    pub fn new(mut units: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(units))))
    }
    

    pub fn setint(&mut self, mut value: i32) {
        self.units = ((value) as i64);
        self.updateScaled();
    }
    

    pub fn setlong(&mut self, mut value: i64) {
        self.units = value.clone();
        self.updateScaled();
    }
    

    pub fn set(&mut self, mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) {
        self.units = jcall!(bigDecimal).getUnscaled();
        self.updateScaled();
    }
    

    pub fn addint(&mut self, mut value: i32) {
        self.units = self.units + ((value) as i64);
        self.updateScaled();
    }
    

    pub fn addlong(&mut self, mut value: i64) {
        self.units = self.units + value;
        self.updateScaled();
    }
    

    pub fn add(&mut self, mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) {
        self.units = self.units + jcall!(bigDecimal).getUnscaled();
        self.updateScaled();
    }
    

    pub fn subtractint(&mut self, mut value: i32) {
        self.units = self.units - ((value) as i64);
        self.updateScaled();
    }
    

    pub fn subtractlong(&mut self, mut value: i64) {
        self.units = self.units - value;
        self.updateScaled();
    }
    

    pub fn subtract(&mut self, mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) {
        self.units = self.units - jcall!(bigDecimal).getUnscaled();
        self.updateScaled();
    }
    

    pub fn multiplyint(&mut self, mut value: i32) {
        self.units = self.units * ((value) as i64);
        self.updateScaled();
    }
    

    pub fn multiplylong(&mut self, mut value: i64) {
        self.units = self.units * value;
        self.updateScaled();
    }
    

    pub fn multiply(&mut self, mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) {
        self.units = self.units * jcall!(bigDecimal).getUnscaled();
        self.updateScaled();
    }
    

    pub fn divideint(&mut self, mut value: i32) {
        self.units = self.units / ((value) as i64);
        self.updateScaled();
    }
    

    pub fn dividelong(&mut self, mut value: i64) {
        self.units = self.units / value;
        self.updateScaled();
    }
    

    pub fn divide(&mut self, mut bigDecimal: Option<Rc<RefCell<BasicDecimal>>>) {
        self.units = self.units / jcall!(bigDecimal).getUnscaled();
        self.updateScaled();
    }
    

    pub fn getUnscaled(&mut self) -> i64 {
        return self.units.clone();
    }
    

    pub fn updateScaled(&mut self) {
        self.scaled = (((self.units / ((self.factorValue) as i64))) as i32);
    }
    

    pub fn getScaled(&mut self) -> i32 {
        return self.scaled.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return (self.units.clone()).to_string();
    }
    

    pub fn getScaledFactor(&mut self) -> i32 {
        return self.factor.clone();
    }
    

    pub fn getScaledFactorValue(&mut self) -> i32 {
        return self.factorValue.clone();
    }
}

