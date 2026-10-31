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
// package org.allbinary.graphics
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::GPoint::GPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Line {
    pub p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub p2: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>,
    pub deltaX: f64,
    pub deltaY: f64,
}


impl Line {
    

    pub fn new_struct(mut p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut p2: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Self {
        let mut this = Self {
            p1: None,
            p2: None,
            deltaX: 0.0,
            deltaY: 0.0,
        };
        this.p1 = jcall!(PointFactory::PointFactory::getInstance()).ZERO_ZERO.clone();
        this.p2 = this.p1.clone();
        this.setP1(p1.clone());
        this.setP2(p2.clone());
        this
    }

    pub fn new(mut p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>, mut p2: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(p1, p2))))
    }
    

    pub fn getGradient(&mut self) -> f64 {
        return self.getDeltaY() / self.getDeltaX();
    }
    

    pub fn update(&mut self) {
        self.deltaX = ((jcall!(self.getP1()).getX()) as f64) - jcall!(self.getP2()).getX();
        self.deltaY = ((jcall!(self.getP1()).getY()) as f64) - jcall!(self.getP2()).getY();
    }
    

    pub fn getDeltaX(&mut self) -> f64 {
        return self.deltaX.clone();
    }
    

    pub fn getDeltaY(&mut self) -> f64 {
        return self.deltaY.clone();
    }
    

    pub fn getP1(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.p1.clone();
    }
    

    pub fn getP2(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>> {
        return self.p2.clone();
    }
    

    pub fn setP1(&mut self, mut p1: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.p1 = p1.clone();
        self.update();
    }
    

    pub fn setP2(&mut self, mut p2: Option<Rc<RefCell<dyn crate::org::allbinary::graphics::GPoint::GPointDyn>>>) {
        self.p2 = p2.clone();
        self.update();
    }
}

