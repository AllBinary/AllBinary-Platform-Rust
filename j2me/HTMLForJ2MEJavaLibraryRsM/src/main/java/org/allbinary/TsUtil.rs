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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::microedition::lcdui::Displayable::Displayable;
// use crate::javax::microedition::lcdui::Form::Form;
// use crate::javax::microedition::rms::InvalidRecordIDException::InvalidRecordIDException;
// use crate::javax::microedition::rms::RecordStore::RecordStore;
// use crate::javax::microedition::rms::RecordStoreException::RecordStoreException;
// use crate::javax::microedition::rms::RecordStoreNotOpenException::RecordStoreNotOpenException;
use crate::org::allbinary::logic::NullUtil::NullUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

//HTML
pub struct TsUtil {
}

thread_local! { pub static TsUtil_instance: Option<Rc<RefCell<TsUtil>>> = TsUtil::new(); }

impl TsUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TsUtil>>> {
        return TsUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn hashCode(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> i32 {
        return (Rc::as_ptr(anyType.as_ref().expect("null reference")) as usize as i32);
    }
    

    // throws InterruptedException
    pub fn waitFor(&mut self, mut any: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut timeoutMillis: i64) {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws RecordStoreNotOpenException, InvalidRecordIDException, RecordStoreException
    pub fn getRecord(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut recordId: i32) -> Vec<i8> {
        let mut recordStore: Option<Rc<RefCell<RecordStore>>> = (anyType).clone() /* as RecordStore */;
        let mut data: Vec<i8> = Vec::new();
        { // synchronized (self)
            data = vec![0; (jcall!(recordStore).getRecordSize(recordId.clone())) as usize];
            jcall!(recordStore).getRecord(recordId.clone(), data.clone(), 0);
        }
        return (if (data.len() as i32) < 1 { jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY } else { data });
    }
    

    pub fn compareTo(&mut self, mut a: String, mut b: String) -> i32 {
        return (a.cmp(&b.clone()) as i32);
    }
    

    pub fn equalIgnoreCase(&mut self, mut a: String, mut b: String) -> bool {
        return a.eq_ignore_ascii_case(&b.clone());
    }
    

    pub fn equalsNotstring(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut object2: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        return { let java_receiver = (anyType).clone(); let java_argument = object2.clone(); assert!(java_receiver.is_some(), "NullPointerException"); crate::java_object_equals(&java_receiver, &java_argument) };
    }
    

    pub fn toNumberFromChar(&mut self, mut value: char) -> i32 {
        return ((value) as i32);
    }
    

    pub fn toNumber(&mut self, mut value: i32) -> i32 {
        return value.clone();
    }
    

    pub fn getByteArray(&mut self, mut string: String) -> Vec<i8> {
        return string.bytes().map(|value| value as i8).collect::<Vec<i8>>();
    }
    

    pub fn isItemListener(&mut self, mut owner: Option<Rc<RefCell<Displayable>>>) -> bool {
        if (owner).is_some() /* instanceof Form */ {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn gc(&mut self) {
    }
}

