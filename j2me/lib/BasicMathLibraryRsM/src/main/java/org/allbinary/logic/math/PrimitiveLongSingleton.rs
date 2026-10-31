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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PrimitiveLongSingleton {
    pub UNK: Vec<char>,
    pub NUMBER_ARRAY: Vec<i8>,
    pub NUMBER_CHAR_ARRAY: Vec<char>,
    pub NUMBER_CHAR_ARRAYS: Vec<Vec<char>>,
    pub ZERO: Vec<char>,
}

thread_local! { pub static PrimitiveLongSingleton_instance: Option<Rc<RefCell<PrimitiveLongSingleton>>> = PrimitiveLongSingleton::new(); }

impl PrimitiveLongSingleton {

    pub fn new_struct() -> Self {
        let mut this = Self {
            UNK: Vec::new(),
            NUMBER_ARRAY: Vec::new(),
            NUMBER_CHAR_ARRAY: Vec::new(),
            NUMBER_CHAR_ARRAYS: Vec::new(),
            ZERO: Vec::new(),
        };
        this.UNK = vec!['U', 'n', 'k'];
        this.NUMBER_ARRAY = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        this.NUMBER_CHAR_ARRAY = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        this.NUMBER_CHAR_ARRAYS = vec![vec!['0'], vec!['1'], vec!['2'], vec!['3'], vec!['4'], vec!['5'], vec!['6'], vec!['7'], vec!['8'], vec!['9']];
        this.ZERO = vec![this.NUMBER_CHAR_ARRAY[(0) as usize].clone()];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PrimitiveLongSingleton>>> {
        return PrimitiveLongSingleton_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getNumberFromChar(&mut self, mut numberAsChar: char) -> i8 {
        let mut size: i32 = (self.NUMBER_CHAR_ARRAY.len() as i32).clone();
        for index in (0..size) {
            if self.NUMBER_CHAR_ARRAY[(index) as usize] == numberAsChar {
                return self.NUMBER_ARRAY[(index) as usize].clone();
            }
        }
        panic!("{:?}", format!("{}: {:?}", "Exception", (jcall!(StringMaker::new()).append(String::from("numberAsChar was not in array: ")).appendchar(numberAsChar.clone()).toString())));
    }
}

