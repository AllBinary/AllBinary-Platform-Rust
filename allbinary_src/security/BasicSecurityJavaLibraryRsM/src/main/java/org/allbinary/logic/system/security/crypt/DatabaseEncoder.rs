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
// package org.allbinary.logic.system.security.crypt
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DatabaseEncoder {
}


impl DatabaseEncoder {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn encode(mut value: Vec<i8>) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut array: Vec<i8> = value.clone();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            for index in (0..(array.len() as i32)) {
                jcall!(stringBuffer).append(jcall!(Some((array[(index) as usize].clone()) as i8)).toString());
                if index < (array.len() as i32) - 1 {
                    jcall!(stringBuffer).append(jcall!(CommonSeps::getInstance()).SPACE.clone());
                }
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putOE(jcall!(commonStrings).EXCEPTION.clone(), String::from("DatabaseEncoder"), String::from("decode"), e.clone());
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            }
        }
    }
    

    pub fn decode(mut value: String) -> Vec<i8> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(CommonSeps::getInstance()).SPACE.clone());
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(tokenizer).getTokensFromString(value.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>));
            let mut byteVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut size: i32 = jcall!(vector).size();
            let mut byteOfData: String = String::new();
            for index in (0..size) {
                byteOfData = (jcall!(vector).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
                jcall!(byteVector).add(crate::java_into_object((Some((byteOfData.clone()) as i8)).expect("NullPointerException")));
            }
            let mut decode: Vec<i8> = vec![0; (jcall!(byteVector).size()) as usize];
            let mut decodeIndex: i32 = 0;
            let mut aByte: Option<i8> = None;
            for index in (0..size) {
                aByte = { let java_object = jcall!(byteVector).as_BasicArrayList_mut().objectArray[(index) as usize]; java_object.as_ref().map(|object| { let object = object.borrow(); object.downcast_ref::<i8>().copied().or_else(|| object.downcast_ref::<Option<i8>>().copied().flatten()).expect("ClassCastException") }) };
                decode[(decodeIndex) as usize] = jcall!(aByte).byteValue();
                decodeIndex += 1;
            }
            return decode.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
            PreLogUtil::putSE(jcall!(commonStrings).EXCEPTION.clone(), String::from("DatabaseEncoder"), String::from("decode"), e.clone());
            return jcall!(NullUtil::getInstance()).NULL_BYTE_ARRAY.clone();
            }
        }
    }
}

