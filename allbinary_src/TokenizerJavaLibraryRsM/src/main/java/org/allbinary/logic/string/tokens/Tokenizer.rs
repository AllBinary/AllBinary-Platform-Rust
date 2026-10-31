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
// package org.allbinary.logic.string.tokens
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Tokenizer {
    pub sep: String,
    pub endSep: String,
}


impl Tokenizer {
    

    pub fn new_struct(mut sep: String) -> Self {
        let mut this = Self {
            sep: String::new(),
            endSep: String::new(),
        };
        this.endSep = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if false || { let left = &(sep); let right = &(jcall!(StringUtil::getInstance()).EMPTY_STRING); let mut left_units = left.encode_utf16(); let mut right_units = right.encode_utf16(); loop { match (left_units.next(), right_units.next()) { (Some(a), Some(b)) if a != b => break a as i32 - b as i32, (Some(_), Some(_)) => {}, (None, None) => break 0, _ => break left.encode_utf16().count() as i32 - right.encode_utf16().count() as i32 } } } == 0 {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Sep provided is not valid"))));
        }
        this.sep = sep.clone();
        this
    }

    pub fn new(mut sep: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(sep))))
    }
    

    pub fn setSep(&mut self, mut sep: String) {
        self.sep = sep.clone();
    }
    

    pub fn setEndSep(&mut self, mut endSep: String) {
        self.endSep = endSep.clone();
    }
    

    pub fn getInsideSep(&mut self, mut string: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut tokenList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut index: i32 = 0;
        let mut size: i32 = (string.chars().count() as i32);
        while index < size {
            index = jcall!(string).indexOf(self.sep.clone(), index.clone());
            if index != -1 {
                let mut end: i32 = jcall!(string).indexOf(self.endSep.clone(), index + (self.sep.chars().count() as i32));
                if end != -1 {
                    jcall!(tokenList).add(crate::java_into_object(string[(index + (self.sep.chars().count() as i32)) as usize..(end - ((self.endSep.chars().count() as i32) - 1)) as usize].to_string()));
                    index = end + (self.endSep.chars().count() as i32);
                } else {
                }
            } else {
                break;
            }
        }
        return tokenList.clone();
    }
    

    pub fn getTokensFromString(&mut self, mut string: String, mut tokenVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut index: i32 = 0;
        let mut end: i32 = 0;
        while index < (string.chars().count() as i32) {
            end = jcall!(string).indexOf(self.sep.clone(), index.clone());
            if end != -1 {
                jcall!(tokenVector).add(crate::java_into_object(string[(index) as usize..(end) as usize].to_string()));
                index = end + (self.sep.chars().count() as i32);
            } else {
                jcall!(tokenVector).add(crate::java_into_object(string[(index) as usize..((string.chars().count() as i32)) as usize].to_string()));
                break;
            }
        }
        return tokenVector.clone();
    }
    

    pub fn getTokens(&mut self, mut stringVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> {
        let mut tokenHashtable: Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>> = jcall!(StdUtil::getInstance()).createHashtable();
        let mut string: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        let mut end: i32 = 0;
        let mut size: i32 = jcall!(stringVector).size();
        for index in (0..size) {
            string = (jcall!(stringVector).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
            end = string.find(&self.sep.clone()).map(|index| index as i32).unwrap_or(-1);
            if end >= 0 {
                jcall!(tokenHashtable).put(crate::java_into_object(string[(0) as usize..(end) as usize].to_string()), crate::java_into_object(string[(end + 1) as usize..((string.chars().count() as i32)) as usize].to_string()));
            }
        }
        return tokenHashtable.clone();
    }
}

