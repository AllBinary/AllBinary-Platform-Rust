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
// package org.allbinary.business.time.modified
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::time::Time::Time;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TimeLastModified {
    pub base: Time,
}


impl TimeLastModified {
    

    pub fn new_struct1(mut time: String) -> Self {
        let mut this = Self {
            base: Time::new_struct1(time.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut time: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(time))))
    }
    

    pub fn new_struct1(mut time: i64) -> Self {
        let mut this = Self {
            base: Time::new_struct1(time.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut time: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(time))))
    }
}
impl std::ops::Deref for TimeLastModified {
    type Target = Time;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for TimeLastModified {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::time::Time::TimeDyn for TimeLastModified {
    fn as_Time(&self) -> &crate::org::allbinary::business::time::Time::Time { &self.base }
    fn as_Time_mut(&mut self) -> &mut crate::org::allbinary::business::time::Time::Time { &mut self.base }
}


