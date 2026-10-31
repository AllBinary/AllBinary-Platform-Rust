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
// package org.allbinary.game.layer.capital.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::game::identification::Group::Group;
use super::CapitalEventHandler::CapitalEventHandler;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct CapitalEventHandlerFactory {
}

thread_local! { pub static CapitalEventHandlerFactory_capitalEventHandlerArray: Vec<Option<Rc<RefCell<CapitalEventHandler>>>> = Vec::new(); }

impl CapitalEventHandlerFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone) = vec![None; (10) as usize];
        for index in (0..=(CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone).len() as i32) - 1).rev() {
            CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone)[(index) as usize] = CapitalEventHandler::new();
        }
    }
    

    pub fn getInstance(mut groupInterface: Option<Rc<RefCell<Group>>>) -> Option<Rc<RefCell<CapitalEventHandler>>> {
        return CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone)[(((jcall!(groupInterface).getGroupId()) as i32)) as usize].clone();
    }
    

    pub fn removeAll() {
        for index in (0..=(CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone).len() as i32) - 1).rev() {
            jcall!(CapitalEventHandlerFactory_capitalEventHandlerArray.with(Clone::clone)[(index) as usize]).removeAllListeners();
        }
    }
}

