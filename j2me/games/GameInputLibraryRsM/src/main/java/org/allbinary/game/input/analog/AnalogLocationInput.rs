// Generated Code Do Not Modify
// package org.allbinary.game.input.analog
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::graphics::CustomGPoint::CustomGPoint;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AnalogLocationInput {
    pub playerId: i32,
    pub customGPoint: Option<Rc<RefCell<CustomGPoint>>>,
    pub rz: i32,
    pub rightTrigger: i32,
    pub leftTrigger: i32,
}

thread_local! { pub static AnalogLocationInput_NULL_ANALOG_LOCATION_INPUT: Option<Rc<RefCell<AnalogLocationInput>>> = AnalogLocationInput::new(-1); }

impl AnalogLocationInput {
    

    pub fn new_struct(mut playerId: i32) -> Self {
        let mut this = Self {
            playerId: 0,
            customGPoint: None,
            rz: 0,
            rightTrigger: 0,
            leftTrigger: 0,
        };
        this.playerId = playerId.clone();
        this.customGPoint = CustomGPoint::new(0, 0, 0);
        this
    }

    pub fn new(mut playerId: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(playerId))))
    }
    

    pub fn getPlayerId(&mut self) -> i32 {
        return self.playerId.clone();
    }
    

    pub fn getCustomGPoint(&mut self) -> Option<Rc<RefCell<CustomGPoint>>> {
        return self.customGPoint.clone();
    }
    

    pub fn getRz(&mut self) -> i32 {
        return self.rz.clone();
    }
    

    pub fn setRz(&mut self, mut rz: i32) {
        self.rz = rz.clone();
    }
    

    pub fn getRightTrigger(&mut self) -> i32 {
        return self.rightTrigger.clone();
    }
    

    pub fn setRightTrigger(&mut self, mut rightTrigger: i32) {
        self.rightTrigger = rightTrigger.clone();
    }
    

    pub fn getLeftTrigger(&mut self) -> i32 {
        return self.leftTrigger.clone();
    }
    

    pub fn setLeftTrigger(&mut self, mut leftTrigger: i32) {
        self.leftTrigger = leftTrigger.clone();
    }
}

