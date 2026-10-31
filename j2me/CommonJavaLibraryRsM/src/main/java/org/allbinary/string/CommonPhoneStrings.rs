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
// package org.allbinary.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CommonPhoneStrings {
    pub UP: String,
    pub DOWN: String,
    pub LEFT: String,
    pub RIGHT: String,
    pub ZERO: String,
    pub ONE: String,
    pub TWO: String,
    pub THREE: String,
    pub FOUR: String,
    pub FIVE: String,
    pub SIX: String,
    pub SEVEN: String,
    pub EIGHT: String,
    pub NINE: String,
    pub POUND: String,
    pub STAR: String,
    pub FIRE: String,
}

thread_local! { pub static CommonPhoneStrings_instance: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::new(); }

impl CommonPhoneStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            UP: String::new(),
            DOWN: String::new(),
            LEFT: String::new(),
            RIGHT: String::new(),
            ZERO: String::new(),
            ONE: String::new(),
            TWO: String::new(),
            THREE: String::new(),
            FOUR: String::new(),
            FIVE: String::new(),
            SIX: String::new(),
            SEVEN: String::new(),
            EIGHT: String::new(),
            NINE: String::new(),
            POUND: String::new(),
            STAR: String::new(),
            FIRE: String::new(),
        };
        this.UP = String::from("UP");
        this.DOWN = String::from("DOWN");
        this.LEFT = String::from("LEFT");
        this.RIGHT = String::from("RIGHT");
        this.ZERO = String::from("0");
        this.ONE = String::from("1");
        this.TWO = String::from("2");
        this.THREE = String::from("3");
        this.FOUR = String::from("4");
        this.FIVE = String::from("5");
        this.SIX = String::from("6");
        this.SEVEN = String::from("7");
        this.EIGHT = String::from("8");
        this.NINE = String::from("9");
        this.POUND = String::from("#");
        this.STAR = String::from("*");
        this.FIRE = String::from("FIRE");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonPhoneStrings>>> {
        return CommonPhoneStrings_instance.with(Clone::clone).clone();
    }
}

