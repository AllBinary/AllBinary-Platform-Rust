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
// package org.allbinary.media.graphics.geography.map.racetrack
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

pub struct FinalPositionRaceTrackFactory {
}

thread_local! { pub static FinalPositionRaceTrackFactory_FIRST_POSITION: i32 = 1; }
pub static mut FinalPositionRaceTrackFactory_index: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| FinalPositionRaceTrackFactory_FIRST_POSITION.with(Clone::clone).clone());

impl FinalPositionRaceTrackFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getInstance() -> i32 {
        return unsafe { let value = FinalPositionRaceTrackFactory_index.clone(); *FinalPositionRaceTrackFactory_index += 1; value };
    }
    

    // synchronized
    pub fn reset() {
        unsafe { *FinalPositionRaceTrackFactory_index = FinalPositionRaceTrackFactory_FIRST_POSITION.with(Clone::clone).clone() };
    }
}

