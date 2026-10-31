// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ARunnable {
}

impl Runnable for ARunnable {
    fn run(&mut self) { Self::run(self) }
}

impl ARunnable {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn run(&mut self) {
    }
}
pub trait ARunnableDyn: crate::Runnable {
    fn as_ARunnable(&self) -> &ARunnable;
    fn as_ARunnable_mut(&mut self) -> &mut ARunnable;
}
impl ARunnableDyn for ARunnable {
    fn as_ARunnable(&self) -> &ARunnable { self }
    fn as_ARunnable_mut(&mut self) -> &mut ARunnable { self }
}


