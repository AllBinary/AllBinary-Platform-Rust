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
// package org.allbinary.logic.java.byteutil
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

pub struct ByteUtil {
}

thread_local! { pub static ByteUtil_instance: Option<Rc<RefCell<ByteUtil>>> = ByteUtil::new(); }

impl ByteUtil {
    

    pub fn getInstance() -> Option<Rc<RefCell<ByteUtil>>> {
        return ByteUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn xor(&mut self, mut array: Vec<i8>, mut mask: i8) -> Vec<i8> {
        let mut bytes: Vec<i8> = vec![0; ((array.len() as i32)) as usize];
        let mut size: i32 = (array.len() as i32).clone();
        let mut index: i32 = 0;
        let mut currentByte: i8 = 0;
        while index < size {
            currentByte = array[(index) as usize].clone();
            bytes[(index) as usize] = (((((currentByte) as i32) ^ ((mask) as i32))) as i8);
            index += 1;
        }
        return bytes.clone();
    }
    

    pub fn not(&mut self, mut array: Vec<i8>) -> Vec<i8> {
        let mut bytes: Vec<i8> = vec![0; ((array.len() as i32)) as usize];
        let mut size: i32 = (array.len() as i32).clone();
        let mut index: i32 = 0;
        while index < size {
            bytes[(index) as usize] = ((!array[(index) as usize]) as i8);
            index += 1;
        }
        return bytes.clone();
    }
    

    pub fn xorByte(&mut self, mut array: Vec<i8>, mut index: i32) -> Vec<i8> {
        let mut mask: i8 = 0;
        if index == 0 {
            mask = 1;
        }
        if index == 1 {
            mask = 2;
        }
        if index == 2 {
            mask = 4;
        }
        if index == 3 {
            mask = 8;
        }
        if index == 4 {
            mask = 16;
        }
        if index == 5 {
            mask = 32;
        }
        if index == 6 {
            mask = 64;
        }
        if index == 7 {
            mask = ((-127) as i8);
        }
        return self.xor(array.clone(), mask.clone());
    }
    

    pub fn compare(&mut self, mut newByteArray: Vec<i8>, mut oldByteArray: Vec<i8>) -> bool {
        if (newByteArray.len() as i32) != (oldByteArray.len() as i32) {
            return false;
        } else {
        }
        let mut size: i32 = (newByteArray.len() as i32).clone();
        for index in (0..size) {
            if ((newByteArray[(index) as usize]) as i32) != ((oldByteArray[(index) as usize]) as i32) {
                return false;
            }
        }
        return true;
    }
    

    pub fn compareWithStats(&mut self, mut newByteArray: Vec<i8>, mut newSize: i32, mut oldByteArray: Vec<i8>, mut stats: Vec<i32>) -> bool {
        if newSize != (oldByteArray.len() as i32) {
            stats[(0) as usize] = -1;
            stats[(2) as usize] = newSize.clone();
            stats[(3) as usize] = (oldByteArray.len() as i32).clone();
            return false;
        } else {
            stats[(0) as usize] = newSize.clone();
        }
        stats[(1) as usize] = 0;
        let mut size: i32 = newSize.clone();
        for index in (0..size) {
            if ((newByteArray[(index) as usize]) as i32) != ((oldByteArray[(index) as usize]) as i32) {
                return false;
            }
            stats[(1) as usize] += 1;
        }
        return true;
    }
}

