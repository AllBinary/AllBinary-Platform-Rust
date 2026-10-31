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
// package org.allbinary.logic.math
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::BasicDecimal::BasicDecimal;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Orientation {
    pub yaw: Option<Rc<RefCell<BasicDecimal>>>,
    pub pitch: Option<Rc<RefCell<BasicDecimal>>>,
    pub roll: Option<Rc<RefCell<BasicDecimal>>>,
}

pub static mut Orientation_NONE: std::cell::LazyCell<Option<Rc<RefCell<Orientation>>>> = std::cell::LazyCell::new(|| Orientation::new(0, 0, 0));
pub static mut Orientation_SINGLETON: std::cell::LazyCell<Option<Rc<RefCell<Orientation>>>> = std::cell::LazyCell::new(|| Orientation::new(0, 0, 0));

impl Orientation {
    

    pub fn getInstance(mut yaw: i32, mut pitch: i32, mut roll: i32) -> Option<Rc<RefCell<Orientation>>> {
        jcall!(jcall!(unsafe { Orientation_SINGLETON.clone() }).yaw).setint(yaw.clone());
        jcall!(jcall!(unsafe { Orientation_SINGLETON.clone() }).pitch).setint(pitch.clone());
        jcall!(jcall!(unsafe { Orientation_SINGLETON.clone() }).roll).setint(roll.clone());
        return unsafe { Orientation_SINGLETON.clone() }.clone();
    }
    

    pub fn new_struct(mut yaw: i32, mut pitch: i32, mut roll: i32) -> Self {
        let mut this = Self {
            yaw: None,
            pitch: None,
            roll: None,
        };
        this.yaw = BasicDecimal::new(0);
        this.pitch = BasicDecimal::new(0);
        this.roll = BasicDecimal::new(0);
        jcall!(this.yaw).setint(yaw.clone());
        jcall!(this.pitch).setint(pitch.clone());
        jcall!(this.roll).setint(roll.clone());
        this
    }

    pub fn new(mut yaw: i32, mut pitch: i32, mut roll: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(yaw, pitch, roll))))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Y: "));
        jcall!(stringBuffer).append(jcall!(self.yaw).toString());
        jcall!(stringBuffer).append(String::from(" P: "));
        jcall!(stringBuffer).append(jcall!(self.pitch).toString());
        jcall!(stringBuffer).append(String::from(" R: "));
        jcall!(stringBuffer).append(jcall!(self.roll).toString());
        return jcall!(stringBuffer).toString();
    }
}

