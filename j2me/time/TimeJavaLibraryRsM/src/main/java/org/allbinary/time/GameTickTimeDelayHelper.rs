// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.time
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

pub struct GameTickTimeDelayHelper {
    pub timeDelta: i64,
    pub startTime: i64,
    pub lastStartTime: i64,
}


impl GameTickTimeDelayHelper {
    

    pub fn new_struct(mut startTime: i64) -> Self {
        let mut this = Self {
            timeDelta: 0_i64,
            startTime: 0_i64,
            lastStartTime: 0_i64,
        };
        this.startTime = startTime.clone();
        this.lastStartTime = 0_i64;
        this
    }

    pub fn new(mut startTime: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(startTime))))
    }
    

    pub fn setStartTime(&mut self) -> i64 {
        self.startTime = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64);
        return self.startTime.clone();
    }
    

    pub fn r#loop(&mut self) {
        if self.lastStartTime == i64::MIN {
            self.timeDelta = 0_i64;
        } else {
            self.timeDelta = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64) - self.lastStartTime;
        }
    }
    

    pub fn getTimeFromStart(&mut self) -> i64 {
        return self.startTime - self.lastStartTime;
    }
}

