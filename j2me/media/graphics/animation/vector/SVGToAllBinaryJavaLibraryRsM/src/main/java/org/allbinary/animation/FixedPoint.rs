// Generated Code Do Not Modify
// package org.allbinary.animation
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

pub struct FixedPoint {
    pub ONE: i32,
}

thread_local! { pub static FixedPoint_instance: Option<Rc<RefCell<FixedPoint>>> = FixedPoint::new(); }

impl FixedPoint {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ONE: 0,
        };
        this.ONE = 1 << 16;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<FixedPoint>>> {
        return FixedPoint_instance.with(Clone::clone).clone();
    }
    

    pub fn toFixed(&mut self, mut val: f64) -> i32 {
        return (((val * 65536.0)) as i32);
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn toFixed(&mut self, mut val: f32) -> i32 {
        return (((val * 65536.0_f32)) as i32);
    }
    

    pub fn mul(&mut self, mut a: i32, mut b: i32) -> i32 {
        return ((((((a) as i64) * ((b) as i64)) >> 16)) as i32);
    }
    

    pub fn div(&mut self, mut a: i32, mut b: i32) -> i32 {
        return (((((((a) as i64)) << 16) / ((b) as i64))) as i32);
    }
    

    pub fn toInt(&mut self, mut fixed: i32) -> i32 {
        return fixed >> 16;
    }
}

