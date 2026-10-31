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
// package org.allbinary.thread
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

pub struct ThreadPoolStrings {
    pub ADD_TASK: String,
    pub START_TASK: String,
    pub COMPLETE_TASK: String,
}

thread_local! { pub static ThreadPoolStrings_instance: Option<Rc<RefCell<ThreadPoolStrings>>> = ThreadPoolStrings::new(); }

impl ThreadPoolStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            ADD_TASK: String::new(),
            START_TASK: String::new(),
            COMPLETE_TASK: String::new(),
        };
        this.ADD_TASK = String::from("addTask");
        this.START_TASK = String::from("Started Task: ");
        this.COMPLETE_TASK = String::from("Completed Task: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThreadPoolStrings>>> {
        return ThreadPoolStrings_instance.with(Clone::clone).clone();
    }
}

