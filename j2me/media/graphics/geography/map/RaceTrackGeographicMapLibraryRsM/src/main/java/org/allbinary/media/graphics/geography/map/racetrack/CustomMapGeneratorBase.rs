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
// package org.allbinary.media.graphics.geography.map.racetrack
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CustomMapGeneratorBase {
}


impl CustomMapGeneratorBase {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn copyMapIntoCustomMap(&mut self) {
    }
    

    pub fn getCustomMapArray(&mut self) -> Vec<Vec<i32>> {
        return jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone();
    }
    

    // throws Exception
    pub fn getCustomType(&mut self, mut column: i32, mut row: i32, mut currentType: i32) -> i32 {
        return -1;
    }
}
pub trait CustomMapGeneratorBaseDyn {
    fn as_CustomMapGeneratorBase(&self) -> &CustomMapGeneratorBase;
    fn as_CustomMapGeneratorBase_mut(&mut self) -> &mut CustomMapGeneratorBase;
    fn copyMapIntoCustomMap(&mut self) { CustomMapGeneratorBase::copyMapIntoCustomMap(self.as_CustomMapGeneratorBase_mut()) }
    fn getCustomMapArray(&mut self) -> Vec<Vec<i32>> { CustomMapGeneratorBase::getCustomMapArray(self.as_CustomMapGeneratorBase_mut()) }
    fn getCustomType(&mut self, mut column: i32, mut row: i32, mut currentType: i32) -> i32 { CustomMapGeneratorBase::getCustomType(self.as_CustomMapGeneratorBase_mut(), column, row, currentType) }
}
impl CustomMapGeneratorBaseDyn for CustomMapGeneratorBase {
    fn as_CustomMapGeneratorBase(&self) -> &CustomMapGeneratorBase { self }
    fn as_CustomMapGeneratorBase_mut(&mut self) -> &mut CustomMapGeneratorBase { self }
}


