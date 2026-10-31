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
// package org.allbinary.logic.string
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::math::MathData::MathData;
use crate::org::allbinary::string::CommonPhoneStrings::CommonPhoneStrings;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SpecialCharacterUtil {
}


impl SpecialCharacterUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getHashMap() -> Option<Rc<RefCell<HashMap>>> {
        let mut commonSeps: Option<Rc<RefCell<CommonSeps>>> = CommonSeps::getInstance();
        let mut commonPhoneStrings: Option<Rc<RefCell<CommonPhoneStrings>>> = CommonPhoneStrings::getInstance();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(String::from("!"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("@"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonPhoneStrings).POUND.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("$"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("%"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("^"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).AMPERSAND.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonPhoneStrings).STAR.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).PARENTHESIS_OPEN.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).PARENTHESIS_CLOSE.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).DASH.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).UNDERSCORE.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(MathData::getInstance()).PLUS.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(commonSeps).EQUALS.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("\\"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("|"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(AbPathData::getInstance()).EXTENSION_SEP.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from(","), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("<"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from(">"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("?"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(jcall!(AbPathData::getInstance()).SEPARATOR.clone(), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("~"), jcall!(stringUtil).EMPTY_STRING.clone());
        jcall!(hashMap).put(String::from("`"), jcall!(stringUtil).EMPTY_STRING.clone());
        return hashMap.clone();
    }
}

