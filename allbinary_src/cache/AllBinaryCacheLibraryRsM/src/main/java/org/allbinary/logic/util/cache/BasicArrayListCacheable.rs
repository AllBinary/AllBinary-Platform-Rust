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
// package org.allbinary.logic.util.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::CacheableInterface::CacheableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicArrayListCacheable {
    pub base: BasicArrayListD,
}

impl CacheableInterface for BasicArrayListCacheable {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
}

impl BasicArrayListCacheable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicArrayListD::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
}
impl std::ops::Deref for BasicArrayListCacheable {
    type Target = BasicArrayListD;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicArrayListCacheable {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn for BasicArrayListCacheable {
    fn as_BasicArrayList(&self) -> &crate::org::allbinary::util::BasicArrayList::BasicArrayList { &self.base.base.base }
    fn as_BasicArrayList_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayList::BasicArrayList { &mut self.base.base.base }
}
impl crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn for BasicArrayListCacheable {
    fn as_BasicArrayListD(&self) -> &crate::org::allbinary::util::BasicArrayListD::BasicArrayListD { &self.base }
    fn as_BasicArrayListD_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayListD::BasicArrayListD { &mut self.base }
}
impl crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn for BasicArrayListCacheable {
    fn as_BasicArrayListS(&self) -> &crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &self.base.base }
    fn as_BasicArrayListS_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &mut self.base.base }
}


