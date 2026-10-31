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
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::GameAdState::GameAdState;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BottomLimiter {
    pub bottom: i32,
    pub deltaY: i32,
    pub lower: i32,
    pub higher: i32,
}

thread_local! { pub static BottomLimiter_instance: Option<Rc<RefCell<BottomLimiter>>> = BottomLimiter::new(); }

impl BottomLimiter {
    

    pub fn getInstance() -> Option<Rc<RefCell<BottomLimiter>>> {
        return BottomLimiter_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            bottom: 0,
            deltaY: 0,
            lower: 0,
            higher: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn init(&mut self, mut lower: i32, mut higher: i32) {
        self.lower = lower.clone();
        self.higher = higher.clone();
    }
    

    pub fn update(&mut self) {
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut gameAdState: Option<Rc<RefCell<GameAdState>>> = jcall!(GameAdStateFactory::GameAdStateFactory::getInstance()).getCurrentInstance();
        if jcall!(gameAdState).isShowingAt() {
            self.deltaY = self.higher.clone();
        } else {
            self.deltaY = self.lower.clone();
        }
        self.bottom = jcall!(displayInfo).getLastHeight() - self.deltaY;
    }
    

    pub fn getDeltaY(&mut self) -> i32 {
        return self.deltaY.clone();
    }
    

    pub fn getBottom(&mut self) -> i32 {
        return self.bottom.clone();
    }
}

