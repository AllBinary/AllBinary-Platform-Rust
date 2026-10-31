// Generated Code Do Not Modify
// package org.allbinary.logic.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct MathUtil {
    pub ACCURACY: i32,
}

thread_local! { pub static MathUtil_instance: Option<Rc<RefCell<MathUtil>>> = MathUtil::new(); }

impl MathUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ACCURACY: 0,
        };
        this.ACCURACY = 100;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<MathUtil>>> {
        return MathUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getTotalDigits(&mut self, mut digits: i32) -> i32 {
        let mut total: i32 = 0;
        while digits > 0 {
            digits = digits / 10;
            total += 1;
        }
        return total.clone();
    }
    

    pub fn sqrt(&mut self, mut value: i32) -> i32 {
        if value <= 1 {
            return value.clone();
        }
        let mut accumulated: i32 = value >> 1;
        let mut result: i32 = (accumulated + (value / accumulated)) >> 1;
        loop {
            accumulated = result.clone();
            result = (accumulated + (value / accumulated)) >> 1;
            if !(accumulated > result) { break; }
        }
        return accumulated.clone();
    }
    

    pub fn sqrtd(&mut self, mut x: i32) -> f64 {
        if x == 0 {
            return ((0.0_f32) as f64);
        }
        if x < 0 {
            return ((-1.0_f32) as f64);
        }
        let mut result: f64 = ((x) as f64);
        for index in (0..self.ACCURACY) {
            result = (result + (((x) as f64) / result)) / 2.0;
        }
        return result.clone();
    }
    

    pub fn abs(&mut self, mut value: i32) -> i32 {
        return (if (value < 0) { -value } else { value });
    }
    

    pub fn abslong(&mut self, mut value: i64) -> i64 {
        return (if (value < 0_i64) { -value } else { value });
    }
    

    pub fn min(&mut self, mut value: i32, mut value2: i32) -> i32 {
        return (if (value <= value2) { value } else { value2 });
    }
    

    pub fn max(&mut self, mut value: i32, mut value2: i32) -> i32 {
        return (if (value >= value2) { value } else { value2 });
    }
}

