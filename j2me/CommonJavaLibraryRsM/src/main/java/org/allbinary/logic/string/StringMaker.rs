// Generated Code Do Not Modify
// package org.allbinary.logic.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::jsinterop::annotations::JsMethod;
use crate::jsinterop::annotations::JsConstructor;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StringMaker {
    pub charArray: Vec<char>,
    pub currentLength: i32,
}


impl StringMaker {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            charArray: Vec::new(),
            currentLength: 0,
        };
        this.charArray = vec!['\0'; (20) as usize];
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn append(&mut self, mut string: String) -> &mut Self {
        let mut stringLength: i32 = (string.chars().count() as i32);
        { let java_arguments = (self.currentLength + stringLength,); self.ensureCapacity(java_arguments.0) };
        { let chars: Vec<char> = string.chars().collect(); self.charArray[(self.currentLength) as usize..(self.currentLength + stringLength - 0) as usize].clone_from_slice(&chars[(0) as usize..(stringLength) as usize]); };
        self.currentLength += stringLength.clone();
        return self;
    }
    

    pub fn appendchar(&mut self, mut c: char) -> &mut Self {
        { let java_arguments = (self.currentLength + 1,); self.ensureCapacity(java_arguments.0) };
        { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = c.clone(); };
        return self;
    }
    

    pub fn appendCharArray(&mut self, mut charArray: Vec<char>, mut start: i32, mut end: i32) -> &mut Self {
        { let java_arguments = (self.currentLength + (end - start),); self.ensureCapacity(java_arguments.0) };
        for index in (start..end) {
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = { let java_value = (charArray[(index) as usize]).clone(); java_value }.clone(); };
        }
        return self;
    }
    

    pub fn appendbyte(&mut self, mut b: i8) -> &mut Self {
        { let java_arguments = ((b.clone()).to_string(),); self.append(java_arguments.0) };
        return self;
    }
    

    pub fn appendshort(&mut self, mut b: i16) -> &mut Self {
        { let java_arguments = ((b.clone()).to_string(),); self.append(java_arguments.0) };
        return self;
    }
    

    pub fn appendint(&mut self, mut i: i32) -> &mut Self {
        { let java_arguments = ((i.clone()).to_string(),); self.append(java_arguments.0) };
        return self;
    }
    

    pub fn appendlong(&mut self, mut l: i64) -> &mut Self {
        { let java_arguments = ((l.clone()).to_string(),); self.append(java_arguments.0) };
        return self;
    }
    

    pub fn appendfloat(&mut self, mut f: f32) -> &mut Self {
        { let java_arguments = ((f.clone()).to_string(),); self.append(java_arguments.0) };
        return self;
    }
    

    pub fn appendboolean(&mut self, mut bool: bool) -> &mut Self {
        if bool {
            { let java_arguments = (self.currentLength + 4,); self.ensureCapacity(java_arguments.0) };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 't'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'r'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'u'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'e'; };
        } else {
            { let java_arguments = (self.currentLength + 5,); self.ensureCapacity(java_arguments.0) };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'f'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'a'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'l'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 's'; };
            { let java_index = ({ let value = self.currentLength; self.currentLength += 1; value }) as usize; self.charArray[java_index] = 'e'; };
        }
        return self;
    }
    

    pub fn ensureCapacity(&mut self, mut minSize: i32) {
        let mut oldCapacity: i32 = (self.charArray.len() as i32).clone();
        if minSize > oldCapacity {
            let mut newCapacity: i32 = oldCapacity * 3;
            if newCapacity < minSize {
                newCapacity = minSize.clone();
            }
            let mut copy: Vec<char> = vec!['\0'; (newCapacity) as usize];
            let mut min: i32 = { let java_arguments = ((self.charArray.len() as i32).clone(), newCapacity.clone(),); self.min(java_arguments.0, java_arguments.1) };
            
                { let java_copy_source = self.charArray[(0) as usize..(0 + min) as usize].to_vec(); copy[(0) as usize..(0 + min) as usize].clone_from_slice(&java_copy_source); };
            self.charArray = copy.clone();
        }
    }
    

    pub fn min(&mut self, mut value: i32, mut value2: i32) -> i32 {
        return (if (value <= value2) { value.clone() } else { value2.clone() });
    }
    

    pub fn delete(&mut self, mut start: i32, mut end: i32) -> &mut Self {
        self.currentLength -= (end - start);
        return self;
    }
    

    pub fn length(&mut self) -> i32 {
        return self.currentLength.clone();
    }
    

    pub fn toString(&mut self) -> String {
        return self.charArray[(0) as usize..(0 + self.currentLength) as usize].iter().collect::<String>();
    }
}

