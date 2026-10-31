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
// package org.allbinary.direction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Direction {
    pub name: String,
    pub otherName: String,
    pub value: i32,
    pub frameFactor: i32,
}

pub static mut Direction_directionArray: std::cell::LazyCell<Vec<Option<Rc<RefCell<Direction>>>>> = std::cell::LazyCell::new(|| vec![None; (13) as usize]);

impl Direction {
    

    pub fn new_struct(mut name: String, mut otherName: String, mut value: i32, mut frameFactor: i32) -> Self {
        let mut this = Self {
            name: String::new(),
            otherName: String::new(),
            value: 0,
            frameFactor: 0,
        };
        this.name = name.clone();
        this.otherName = otherName.clone();
        this.value = value.clone();
        this.frameFactor = frameFactor.clone();
        unsafe { *Direction_directionArray = this };
        this
    }

    pub fn new(mut name: String, mut otherName: String, mut value: i32, mut frameFactor: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, otherName, value, frameFactor))))
    }
    

    pub fn getInstance(mut direction: i32) -> Option<Rc<RefCell<Direction>>> {
        return unsafe { Direction_directionArray.clone() }[(direction) as usize].clone();
    }
    

    pub fn getValue(&mut self) -> i32 {
        return self.value.clone();
    }
    

    pub fn getFrameFactor(&mut self) -> i32 {
        return self.frameFactor.clone();
    }
    

    pub fn getOtherName(&mut self) -> String {
        return self.otherName.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("Direction: "));
        jcall!(stringBuffer).append(self.getName());
        jcall!(stringBuffer).append(String::from(" Or: "));
        jcall!(stringBuffer).append(self.getOtherName());
        jcall!(stringBuffer).append(String::from(" Value: "));
        jcall!(stringBuffer).appendint(self.value.clone());
        return jcall!(stringBuffer).toString();
    }
}

