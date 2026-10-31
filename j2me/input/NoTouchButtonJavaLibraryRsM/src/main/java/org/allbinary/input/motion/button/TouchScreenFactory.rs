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
// package org.allbinary.input.motion.button
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TouchScreenType::TouchScreenType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TouchScreenFactory {
    pub multiTouch: bool,
    pub touch: bool,
    pub touchScreenType: Option<Rc<RefCell<TouchScreenType>>>,
}

thread_local! { pub static TouchScreenFactory_SINGLETON: Option<Rc<RefCell<TouchScreenFactory>>> = TouchScreenFactory::new(); }

impl TouchScreenFactory {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            multiTouch: false,
            touch: false,
            touchScreenType: None,
        };
        this.touchScreenType = jcall!(TouchScreenTypesFactory::TouchScreenTypesFactory::getInstance()).NOTOUCH.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TouchScreenFactory>>> {
        return TouchScreenFactory_SINGLETON.with(Clone::clone).clone();
    }
    

    pub fn isMultiTouch(&mut self) -> bool {
        return self.multiTouch.clone();
    }
    

    pub fn setTouch(&mut self, mut touch: bool) {
    }
    

    pub fn isTouch(&mut self) -> bool {
        return self.touch.clone();
    }
    

    pub fn getTouchScreenType(&mut self) -> Option<Rc<RefCell<TouchScreenType>>> {
        return self.touchScreenType.clone();
    }
}

