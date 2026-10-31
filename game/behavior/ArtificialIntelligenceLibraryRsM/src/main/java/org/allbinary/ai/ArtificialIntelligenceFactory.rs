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
// package org.allbinary.ai
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::ArtificialIntelligenceFactoryInterface::ArtificialIntelligenceFactoryInterface;
use super::ArtificialIntelligence::ArtificialIntelligence;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ArtificialIntelligenceFactory {
}

impl ArtificialIntelligenceFactoryInterface for ArtificialIntelligenceFactory {
    fn getInstance(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn>>> { Self::getInstance(self, anyType) }
}

impl ArtificialIntelligenceFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getInstance(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::ai::ArtificialIntelligence::ArtificialIntelligenceDyn>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}

