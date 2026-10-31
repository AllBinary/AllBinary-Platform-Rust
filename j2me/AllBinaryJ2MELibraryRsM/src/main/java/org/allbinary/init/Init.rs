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
// package org.allbinary.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::InitInterface::InitInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Init {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl InitInterface for Init {
    fn init(&mut self) { Self::init(self) }
}

impl Init {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn init(&mut self) {
    }
}
pub trait InitDyn: crate::org::allbinary::init::InitInterface::InitInterface {
    fn as_Init(&self) -> &Init;
    fn as_Init_mut(&mut self) -> &mut Init;
}
impl InitDyn for Init {
    fn as_Init(&self) -> &Init { self }
    fn as_Init_mut(&mut self) -> &mut Init { self }
}


