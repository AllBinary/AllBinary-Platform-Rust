// 
// AllBinary Open License Version 1 
// Copyright (c) 2003 AllBinary 
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
// package org.allbinary.game.layer
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

pub trait RTSInterface {
    

    fn isSelfUpgradeable(&mut self) -> bool;
    

    fn isCompleted(&mut self) -> bool;
    

    fn getPercentComplete(&mut self) -> i32;
    

    fn getLevel(&mut self) -> i32;
    

    fn getMaxLevel(&mut self) -> i32;
    

    fn getCost(&mut self) -> i32;
    

    fn downgrade(&mut self);
    

    fn getDowngradeCost(&mut self) -> i32;
    

    fn isDowngradeable(&mut self) -> bool;
    

    fn getUpgradeCost(&mut self) -> i32;
    

    fn isUpgradeable(&mut self) -> bool;
    

    fn upgrade(&mut self);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


