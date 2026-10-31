// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::awt::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ColorUtil {
}

thread_local! { pub static ColorUtil_instance: Option<Rc<RefCell<ColorUtil>>> = ColorUtil::new(); }

impl ColorUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ColorUtil>>> {
        return ColorUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn isGreyishOrWhiteLike(&mut self, mut color: Option<Rc<RefCell<Color>>>) -> bool {
        if (jcall!(color).getRed() - jcall!(color).getGreen()).abs() > 20 || (jcall!(color).getRed() - jcall!(color).getBlue()).abs() > 20 || (jcall!(color).getGreen() - jcall!(color).getBlue()).abs() > 20 {
            return false;
        }
        return true;
    }
    

    pub fn isDark(&mut self, mut color: Option<Rc<RefCell<Color>>>) -> bool {
        if jcall!(color).getRed() < 0x77_u32 as i32 && jcall!(color).getRed() < 0x77_u32 as i32 && jcall!(color).getRed() < 0x77_u32 as i32 {
            return true;
        }
        return false;
    }
}

