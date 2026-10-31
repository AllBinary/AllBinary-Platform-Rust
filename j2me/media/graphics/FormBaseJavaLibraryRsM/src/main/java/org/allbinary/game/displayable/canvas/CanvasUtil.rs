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
// package org.allbinary.game.displayable.canvas
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Canvas::Canvas;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CanvasUtil {
}


impl CanvasUtil {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getKeyName(mut key: i32) -> String {
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_UP.clone() } {
            return jcall!(commonPhoneStrings).UP.clone();
        } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_DOWN.clone() } {
            return jcall!(commonPhoneStrings).DOWN.clone();
        } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_RIGHT.clone() } {
            return jcall!(commonPhoneStrings).RIGHT.clone();
        } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_LEFT.clone() } {
            return jcall!(commonPhoneStrings).LEFT.clone();
        } else if key == unsafe { crate::javax::microedition::lcdui::Canvas::Canvas_FIRE.clone() } {
            return jcall!(commonPhoneStrings).FIRE.clone();
        }
        return String::from("UNKNOWN KEY");
    }
}

