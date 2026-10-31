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
// package org.allbinary.data
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::string::tokens::Tokenizer::Tokenizer;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CamelCaseUtil {
    pub commonSeps: Option<Rc<RefCell<CommonSeps>>>,
}

thread_local! { pub static CamelCaseUtil_instance: Option<Rc<RefCell<CamelCaseUtil>>> = CamelCaseUtil::new(); }

impl CamelCaseUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonSeps: None,
        };
        this.commonSeps = CommonSeps::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CamelCaseUtil>>> {
        return CamelCaseUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getAsCamelCase(&mut self, mut string: String, mut stringBuilder: Option<Rc<RefCell<StringMaker>>>) -> String {
        if false {
            return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        }
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        jcall!(stringBuilder).delete(0, jcall!(stringBuilder).length());
        let mut list: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut tokenizer: Option<Rc<RefCell<Tokenizer>>> = Tokenizer::new(jcall!(self.commonSeps).UNDERSCORE.clone());
        jcall!(tokenizer).getTokensFromString(string.clone(), list.clone());
        let mut size: i32 = jcall!(list).size();
        let mut word: String = String::new();
        for i in (0..size) {
            word = (jcall!(list).get(i.clone())).clone() /* as String */;
            word = (if jcall!(stringValidationUtil).isEmpty(word.clone()) { word } else { jcall!(StringMaker::new()).appendchar((word.chars().nth((0) as usize).unwrap()).to_ascii_uppercase()).append(word[(1) as usize..].to_string().to_lowercase()).toString() });
            jcall!(stringBuilder).append(word.clone());
        }
        return jcall!(stringBuilder).toString();
    }
}

