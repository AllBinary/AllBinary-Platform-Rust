// Generated Code Do Not Modify
// package org.allbinary.animation
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::FixedPoint::FixedPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PGUtil {
}

thread_local! { pub static PGUtil_instance: Option<Rc<RefCell<PGUtil>>> = PGUtil::new(); }

impl PGUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PGUtil>>> {
        return PGUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn flattenCubicBezier(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut x3: i32, mut y3: i32, mut outX: Vec<i32>, mut outY: Vec<i32>, mut count: Vec<i32>, mut maxPoints: i32) {
        let mut fixedPoint: Option<Rc<RefCell<FixedPoint>>> = FixedPoint::getInstance();
        let mut steps: i32 = 10;
        for i in (1..=steps) {
            let mut t: i32 = jcall!(fixedPoint).div(jcall!(fixedPoint).toFixed(((i) as f64)), jcall!(fixedPoint).toFixed(steps.clone()));
            let mut mt: i32 = jcall!(fixedPoint).ONE - t;
            let mut mt3: i32 = jcall!(fixedPoint).mul(jcall!(fixedPoint).mul(mt.clone(), mt.clone()), mt.clone());
            let mut mt2t3: i32 = 3 * jcall!(fixedPoint).mul(jcall!(fixedPoint).mul(mt.clone(), mt.clone()), t.clone());
            let mut mtt23: i32 = 3 * jcall!(fixedPoint).mul(jcall!(fixedPoint).mul(mt.clone(), t.clone()), t.clone());
            let mut t3: i32 = jcall!(fixedPoint).mul(jcall!(fixedPoint).mul(t.clone(), t.clone()), t.clone());
            let mut x: i32 = jcall!(fixedPoint).mul(mt3.clone(), x0.clone()) + jcall!(fixedPoint).mul(mt2t3.clone(), x1.clone()) + jcall!(fixedPoint).mul(mtt23.clone(), x2.clone()) + jcall!(fixedPoint).mul(t3.clone(), x3.clone());
            let mut y: i32 = jcall!(fixedPoint).mul(mt3.clone(), y0.clone()) + jcall!(fixedPoint).mul(mt2t3.clone(), y1.clone()) + jcall!(fixedPoint).mul(mtt23.clone(), y2.clone()) + jcall!(fixedPoint).mul(t3.clone(), y3.clone());
            if count[(0) as usize] < maxPoints {
                outX[(count[(0) as usize]) as usize] = x.clone();
                outY[(count[(0) as usize]) as usize] = y.clone();
                count[(0) as usize] += 1;
            }
        }
    }
}

