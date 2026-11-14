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
// package org.allbinary.game.paint.help
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpdateMyFontProperties {
    pub actionStringArray: Vec<String>,
    pub keymappingBeginWidthArray: Vec<i32>,
    pub actionStringDeltaXArray: Vec<i32>,
    pub inputDeltaXArray: Vec<Vec<i32>>,
    pub sepArray: Vec<Vec<String>>,
    pub sepWidthArray: Vec<Vec<i32>>,
    pub charHeight: i32,
}


impl UpdateMyFontProperties {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }

    pub fn new_struct() -> Self {
        let mut this = Self {
            actionStringArray: Vec::new(),
            keymappingBeginWidthArray: Vec::new(),
            actionStringDeltaXArray: Vec::new(),
            inputDeltaXArray: Vec::new(),
            sepArray: Vec::new(),
            sepWidthArray: Vec::new(),
            charHeight: 0,
        };
        this.actionStringArray = jcall!(StringUtil::getInstance()).getArrayInstance();
        this.keymappingBeginWidthArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone(); java_field }.clone();
        this.actionStringDeltaXArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY.clone(); java_field }.clone();
        this.inputDeltaXArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone(); java_field }.clone();
        this.sepArray = { let java_field = jcall!(StringUtil::getInstance()).ONE_EMPTY_STRING_ARRAY_OF_ARRAYS.clone(); java_field }.clone();
        this.sepWidthArray = { let java_field = jcall!(NullUtil::getInstance()).NULL_INT_ARRAY_ARRAY.clone(); java_field }.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

