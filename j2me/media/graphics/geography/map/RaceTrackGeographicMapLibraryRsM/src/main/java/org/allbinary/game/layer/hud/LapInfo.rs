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
// package org.allbinary.game.layer.hud
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LapInfo {
    pub numberOfLaps: i32,
    pub totalLaps: i32,
}

pub static mut LapInfo_NULL_LAP_INFO: std::cell::LazyCell<Option<Rc<RefCell<LapInfo>>>> = std::cell::LazyCell::new(|| LapInfo::new(-1));

impl LapInfo {
    

    pub fn new_struct(mut totalLaps: i32) -> Self {
        let mut this = Self {
            numberOfLaps: 0,
            totalLaps: 0,
        };
        this.totalLaps = totalLaps.clone();
        this.numberOfLaps = 1;
        this
    }

    pub fn new(mut totalLaps: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(totalLaps))))
    }
    

    pub fn add(&mut self) {
        self.numberOfLaps += 1;
    }
    

    pub fn getCurrentLap(&mut self) -> i32 {
        return self.numberOfLaps.clone();
    }
    

    pub fn getTotalLaps(&mut self) -> i32 {
        return self.totalLaps.clone();
    }
    

    pub fn isDone(&mut self) -> bool {
        if self.numberOfLaps >= self.totalLaps {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("isDone: "));
        jcall!(stringBuffer).appendboolean(self.isDone());
        jcall!(stringBuffer).append(String::from(" CurrentLap: "));
        jcall!(stringBuffer).appendint(self.getCurrentLap());
        jcall!(stringBuffer).append(String::from(" Total Laps: "));
        jcall!(stringBuffer).appendint(self.getTotalLaps());
        return jcall!(stringBuffer).toString();
    }
}

