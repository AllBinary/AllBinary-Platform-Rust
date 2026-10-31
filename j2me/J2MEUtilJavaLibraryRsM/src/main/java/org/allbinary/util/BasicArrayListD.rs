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
use crate::jsinterop::annotations::JsConstructor;
use super::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicArrayListD {
    pub base: BasicArrayListS,
}


impl BasicArrayListD {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicArrayListS::new_struct(7),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}
impl std::ops::Deref for BasicArrayListD {
    type Target = BasicArrayListS;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicArrayListD {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasicArrayListDDyn: crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn {
    fn as_BasicArrayListD(&self) -> &BasicArrayListD;
    fn as_BasicArrayListD_mut(&mut self) -> &mut BasicArrayListD;
}
impl BasicArrayListDDyn for BasicArrayListD {
    fn as_BasicArrayListD(&self) -> &BasicArrayListD { self }
    fn as_BasicArrayListD_mut(&mut self) -> &mut BasicArrayListD { self }
}

impl crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn for BasicArrayListD {
    fn as_BasicArrayList(&self) -> &crate::org::allbinary::util::BasicArrayList::BasicArrayList { &self.base.base }
    fn as_BasicArrayList_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayList::BasicArrayList { &mut self.base.base }
}
impl crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn for BasicArrayListD {
    fn as_BasicArrayListS(&self) -> &crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &self.base }
    fn as_BasicArrayListS_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &mut self.base }
}


