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
// package org.allbinary.util
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::ForcedLogUtil::ForcedLogUtil;
use super::BasicArrayListS::BasicArrayListS;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ImmutableBasicArrayList {
    pub base: BasicArrayListS,
    pub name: String,
}


impl ImmutableBasicArrayList {
    

    pub fn new_struct(mut name: String, mut size: i32) -> Self {
        let mut this = Self {
            base: BasicArrayListS::new_struct(size.clone()),
            name: String::new(),
        };
        // super(...) is in the struct literal above
        this.name = name.clone();
        this
    }

    pub fn new(mut name: String, mut size: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name, size))))
    }
    

    pub fn add(&mut self, mut e: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        ForcedLogUtil::log(self.name.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
        return true;
    }
    

    pub fn addAt(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        ForcedLogUtil::log(self.name.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())));
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}
impl std::ops::Deref for ImmutableBasicArrayList {
    type Target = BasicArrayListS;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImmutableBasicArrayList {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn for ImmutableBasicArrayList {
    fn as_BasicArrayList(&self) -> &crate::org::allbinary::util::BasicArrayList::BasicArrayList { &self.base.base }
    fn as_BasicArrayList_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayList::BasicArrayList { &mut self.base.base }
    fn addAt(&mut self, mut index: i32, mut element: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { Self::addAt(self, index, element) }
    fn add(&mut self, mut e: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { Self::add(self, e) }
    fn toString(&mut self) -> String { Self::toString(self) }
}
impl crate::org::allbinary::util::BasicArrayListS::BasicArrayListSDyn for ImmutableBasicArrayList {
    fn as_BasicArrayListS(&self) -> &crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &self.base }
    fn as_BasicArrayListS_mut(&mut self) -> &mut crate::org::allbinary::util::BasicArrayListS::BasicArrayListS { &mut self.base }
}


