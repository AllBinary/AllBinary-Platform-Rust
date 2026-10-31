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
// package org.allbinary.graphics.color
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::util::CircularIndexUtil::CircularIndexUtil;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;
use super::BasicColor::BasicColor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicColorArrayIndexer {
    pub basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>,
    pub circularIndexUtil: Option<Rc<RefCell<CircularIndexUtil>>>,
}


impl BasicColorArrayIndexer {
    

    pub fn new_struct(mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>) -> Self {
        let mut this = Self {
            basicColorArray: Vec::new(),
            circularIndexUtil: None,
        };
        this.basicColorArray = basicColorArray.clone();
        this.circularIndexUtil = CircularIndexUtil::createInstance((this.basicColorArray.len() as i32).clone());
        this
    }

    pub fn new(mut basicColorArray: Vec<Option<Rc<RefCell<BasicColor>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(basicColorArray))))
    }
    

    pub fn next(&mut self) {
        jcall!(self.circularIndexUtil).next();
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<BasicColor>>> {
        return self.basicColorArray[(jcall!(self.circularIndexUtil).getIndex()) as usize].clone();
    }
    

    pub fn getBasicColorArray(&mut self) -> Vec<Option<Rc<RefCell<BasicColor>>>> {
        return self.basicColorArray.clone();
    }
}

