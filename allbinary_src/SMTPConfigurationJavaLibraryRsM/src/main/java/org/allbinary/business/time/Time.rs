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
// package org.allbinary.business.time
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Time {
    pub timeLong: Option<i64>,
}


impl Time {
    

    pub fn new_struct1(mut time: String) -> Self {
        let mut this = Self {
            timeLong: None,
        };
        this.timeLong = Some((time.clone()) as i64);
        this
    }

    pub fn new1(mut time: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(time))))
    }
    

    pub fn new_struct1(mut time: i64) -> Self {
        let mut this = Self {
            timeLong: None,
        };
        this.timeLong = Some((time.clone()) as i64);
        this
    }

    pub fn new1(mut time: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(time))))
    }
    

    pub fn getLong(&mut self) -> Option<i64> {
        return self.timeLong.clone();
    }
    

    pub fn getlong(&mut self) -> i64 {
        return ((self.timeLong).unwrap() as i64);
    }
    

    pub fn toString(&mut self) -> String {
        return jcall!(self.getLong()).toString();
    }
}
pub trait TimeDyn {
    fn as_Time(&self) -> &Time;
    fn as_Time_mut(&mut self) -> &mut Time;
    fn getLong(&mut self) -> Option<i64> { Time::getLong(self.as_Time_mut()) }
    fn getlong(&mut self) -> i64 { Time::getlong(self.as_Time_mut()) }
    fn toString(&mut self) -> String { Time::toString(self.as_Time_mut()) }
}
impl TimeDyn for Time {
    fn as_Time(&self) -> &Time { self }
    fn as_Time_mut(&mut self) -> &mut Time { self }
}


