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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::jsinterop::annotations::JsType;
use crate::org::allbinary::logic::NullUtil::NullUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::jsinterop::annotations::JsConstructor;
use super::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicArrayListS {
    pub base: BasicArrayList,
}


impl BasicArrayListS {
    

    pub fn new_struct(mut size: i32) -> Self {
        let mut this = Self {
            base: BasicArrayList::new_struct(jcall!(NullUtil::getInstance()).NULL_OBJECT_ARRAY.clone()),
        };
        // super(...) is in the struct literal above
        if size < 0 {
            panic!("{:?}", format!("{}: {:?}", "IllegalArgumentException", (jcall!(StringMaker::new()).append(String::from("Init Size Exception: ")).appendint(size.clone()).toString())));
        }
        this.objectArray = vec![None; (size) as usize];
        this
    }

    pub fn new(mut size: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(size))))
    }
}
impl std::ops::Deref for BasicArrayListS {
    type Target = BasicArrayList;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicArrayListS {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicArrayListSDyn: crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn {
    fn as_BasicArrayListS(&self) -> &BasicArrayListS;
    fn as_BasicArrayListS_mut(&mut self) -> &mut BasicArrayListS;
}
impl BasicArrayListSDyn for BasicArrayListS {
    fn as_BasicArrayListS(&self) -> &BasicArrayListS { self }
    fn as_BasicArrayListS_mut(&mut self) -> &mut BasicArrayListS { self }
}

impl crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn for BasicArrayListS {
    fn as_BasicArrayList(&self) -> &crate::org::allbinary::util::BasicArrayList::BasicArrayList { &self.base }
    fn as_BasicArrayList_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayList::BasicArrayList { &mut self.base }
}


