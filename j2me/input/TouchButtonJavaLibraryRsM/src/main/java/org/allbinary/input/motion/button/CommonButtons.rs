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
use crate::org::allbinary::graphics::PointFactory::PointFactory;
use crate::org::allbinary::graphics::Rectangle::Rectangle;
use crate::org::allbinary::graphics::displayable::DisplayInfoSingleton::DisplayInfoSingleton;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonButtons {
    pub STANDARD_BUTTON_SIZE: i32,
    pub NORMAL_BUTTON: Option<Rc<RefCell<Rectangle>>>,
    pub LARGE_BUTTON: Option<Rc<RefCell<Rectangle>>>,
}

thread_local! { pub static CommonButtons_instance: Option<Rc<RefCell<CommonButtons>>> = CommonButtons::new(); }

impl CommonButtons {
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonButtons>>> {
        return CommonButtons_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            STANDARD_BUTTON_SIZE: 0,
            NORMAL_BUTTON: None,
            LARGE_BUTTON: None,
        };
        let mut displayInfo: Option<Rc<RefCell<DisplayInfoSingleton>>> = DisplayInfoSingleton::getInstance();
        let mut commonButtonSize: i32 = 128;
        while commonButtonSize > 64 {
            let mut totalColumns: i32 = jcall!(displayInfo).getLastWidth() / commonButtonSize;
            let mut totalRows: i32 = jcall!(displayInfo).getLastHeight() / commonButtonSize;
            let mut max: i32 = totalColumns.clone();
            if totalRows > max {
                max = totalRows.clone();
            }
            if max > 9 {
                break;
            }
            commonButtonSize = commonButtonSize >> 1;
        }
        this.STANDARD_BUTTON_SIZE = commonButtonSize.clone();
        this.NORMAL_BUTTON = Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), this.STANDARD_BUTTON_SIZE.clone(), this.STANDARD_BUTTON_SIZE.clone());
        this.LARGE_BUTTON = Rectangle::new(jcall!(PointFactory::getInstance()).ZERO_ZERO.clone(), this.STANDARD_BUTTON_SIZE << 1, this.STANDARD_BUTTON_SIZE << 1);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

