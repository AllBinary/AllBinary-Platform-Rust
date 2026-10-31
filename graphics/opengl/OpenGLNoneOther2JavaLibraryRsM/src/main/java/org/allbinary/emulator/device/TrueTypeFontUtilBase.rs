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
// package org.allbinary.emulator.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TrueTypeFontUtilBase {
}


impl TrueTypeFontUtilBase {
    

    pub fn new_struct(mut scale: i32) -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new(mut scale: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(scale))))
    }
    

    pub fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 {
        return -1;
    }
    

    pub fn getCellSize(&mut self, mut cellSize: i32) -> i32 {
        return -1;
    }
}
pub trait TrueTypeFontUtilBaseDyn {
    fn as_TrueTypeFontUtilBase(&self) -> &TrueTypeFontUtilBase;
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut TrueTypeFontUtilBase;
    fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 { TrueTypeFontUtilBase::getAsTextureSize(self.as_TrueTypeFontUtilBase_mut(), textureSize) }
    fn getCellSize(&mut self, mut cellSize: i32) -> i32 { TrueTypeFontUtilBase::getCellSize(self.as_TrueTypeFontUtilBase_mut(), cellSize) }
    fn getAsTextureSize(&mut self, mut textureSize: i32) -> i32 { TrueTypeFontUtilBase::getAsTextureSize(self.as_TrueTypeFontUtilBase_mut(), textureSize) }
    fn getCellSize(&mut self, mut cellSize: i32) -> i32 { TrueTypeFontUtilBase::getCellSize(self.as_TrueTypeFontUtilBase_mut(), cellSize) }
}
impl TrueTypeFontUtilBaseDyn for TrueTypeFontUtilBase {
    fn as_TrueTypeFontUtilBase(&self) -> &TrueTypeFontUtilBase { self }
    fn as_TrueTypeFontUtilBase_mut(&mut self) -> &mut TrueTypeFontUtilBase { self }
}


