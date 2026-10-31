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
// package org.allbinary.logic.visual.transform.info.objectConfig.generator
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::TransformInfoObjectConfigGeneratorInterface::TransformInfoObjectConfigGeneratorInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransformInfoObjectConfigGenerator {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl TransformInfoObjectConfigGeneratorInterface for TransformInfoObjectConfigGenerator {
    fn process(&mut self, mut input: String) -> String { Self::process(self, input) }
}

impl TransformInfoObjectConfigGenerator {

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
    pub fn process(&mut self, mut input: String) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait TransformInfoObjectConfigGeneratorDyn: crate::org::allbinary::logic::visual::transform::info::objectConfig::generator::TransformInfoObjectConfigGeneratorInterface::TransformInfoObjectConfigGeneratorInterface {
    fn as_TransformInfoObjectConfigGenerator(&self) -> &TransformInfoObjectConfigGenerator;
    fn as_TransformInfoObjectConfigGenerator_mut(&mut self) -> &mut TransformInfoObjectConfigGenerator;
}
impl TransformInfoObjectConfigGeneratorDyn for TransformInfoObjectConfigGenerator {
    fn as_TransformInfoObjectConfigGenerator(&self) -> &TransformInfoObjectConfigGenerator { self }
    fn as_TransformInfoObjectConfigGenerator_mut(&mut self) -> &mut TransformInfoObjectConfigGenerator { self }
}


