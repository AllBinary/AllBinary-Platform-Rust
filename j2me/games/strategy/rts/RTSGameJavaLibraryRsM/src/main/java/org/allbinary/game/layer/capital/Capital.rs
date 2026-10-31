// 
// AllBinary Open License Version 1 
// Copyright (c) 2002 AllBinary 
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
// package org.allbinary.game.layer.capital
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

pub struct Capital {
    pub value: i32,
}


impl Capital {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            value: 0,
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getTotalMoney(&mut self) -> i32 {
        return self.value.clone();
    }
    

    pub fn removeMoney(&mut self, mut points: i32) {
        self.value -= points.clone();
    }
    

    pub fn addMoney(&mut self, mut points: i32) {
        self.value += points.clone();
    }
}
pub trait CapitalDyn {
    fn as_Capital(&self) -> &Capital;
    fn as_Capital_mut(&mut self) -> &mut Capital;
    fn getTotalMoney(&mut self) -> i32 { Capital::getTotalMoney(self.as_Capital_mut()) }
    fn removeMoney(&mut self, mut points: i32) { Capital::removeMoney(self.as_Capital_mut(), points) }
    fn addMoney(&mut self, mut points: i32) { Capital::addMoney(self.as_Capital_mut(), points) }
}
impl CapitalDyn for Capital {
    fn as_Capital(&self) -> &Capital { self }
    fn as_Capital_mut(&mut self) -> &mut Capital { self }
}


