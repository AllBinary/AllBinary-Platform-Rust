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
// package org.allbinary.logic.communication.http
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

fn java_string_hash(text: &str) -> i32 {
    text.chars().fold(0i32, |hash, character| hash.wrapping_mul(31).wrapping_add(character as i32))
}

pub struct AcceptableResponseUtil {
    pub candidateVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub tagNameVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}

thread_local! { pub static AcceptableResponseUtil_instance: Option<Rc<RefCell<AcceptableResponseUtil>>> = AcceptableResponseUtil::new(); }

impl AcceptableResponseUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            candidateVector: None,
            tagNameVector: None,
        };
        this.candidateVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.tagNameVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AcceptableResponseUtil>>> {
        return AcceptableResponseUtil_instance.with(Clone::clone).clone();
    }
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        jcall!(TsUtil::getInstance()).hashCode(crate::java_into_object(jcall!(AcceptableResponseFactory::AcceptableResponseFactory::getInstance()).HTML.clone()));
    }
    

    pub fn get(&mut self, mut index: i32) -> String {
        return (jcall!(self.candidateVector).get(index.clone())).clone() /* as String */;
    }
    

    pub fn getTagName(&mut self, mut index: i32) -> String {
        return (jcall!(self.tagNameVector).get(index.clone())).clone() /* as String */;
    }
    

    pub fn size(&mut self) -> i32 {
        return jcall!(self.candidateVector).size();
    }
}

