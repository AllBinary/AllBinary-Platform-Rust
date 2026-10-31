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
// package org.allbinary.game.physics.acceleration
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

pub struct BasicAccelerationProperties {
    pub forward: i32,
    pub reverse: i32,
}


impl BasicAccelerationProperties {
    

    pub fn new_struct(mut forward: i32, mut reverse: i32) -> Self {
        let mut this = Self {
            forward: 0,
            reverse: 0,
        };
        this.setForward(forward.clone());
        this.setReverse(reverse.clone());
        this
    }

    pub fn new(mut forward: i32, mut reverse: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(forward, reverse))))
    }
    

    pub fn setReverse(&mut self, mut reverse: i32) {
        self.reverse = reverse.clone();
    }
    

    pub fn getReverse(&mut self) -> i32 {
        return self.reverse.clone();
    }
    

    pub fn setForward(&mut self, mut forward: i32) {
        self.forward = forward.clone();
    }
    

    pub fn getForward(&mut self) -> i32 {
        return self.forward.clone();
    }
}

