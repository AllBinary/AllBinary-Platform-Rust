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
// package org.allbinary.game.ag.life
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

pub trait LifeInterface {
    

    fn get(&mut self) -> i16;
    

    fn set(&mut self, lives: i16);
    

    fn reset(&mut self);
    

    fn add(&mut self, lives: i16);
    

    fn deaths(&mut self, lives: i16);
    

    fn getMaxlives(&mut self) -> i16;
    

    fn isAlive(&mut self) -> bool;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


