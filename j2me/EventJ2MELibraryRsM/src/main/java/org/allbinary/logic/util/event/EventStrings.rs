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
// package org.allbinary.logic.util.event
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

pub struct EventStrings {
    pub PERFORMANCE_MESSAGE: String,
    pub TOTAL_LISTENERS: String,
    pub LISTENER_LABEL: String,
    pub FIRE_EVENT: String,
}

thread_local! { pub static EventStrings_instance: Option<Rc<RefCell<EventStrings>>> = EventStrings::new(); }

impl EventStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            PERFORMANCE_MESSAGE: String::new(),
            TOTAL_LISTENERS: String::new(),
            LISTENER_LABEL: String::new(),
            FIRE_EVENT: String::new(),
        };
        this.PERFORMANCE_MESSAGE = String::from("Use Custom onEvent Methods for needed optimization");
        this.TOTAL_LISTENERS = String::from("Total Listeners: ");
        this.LISTENER_LABEL = String::from(" Listener: ");
        this.FIRE_EVENT = String::from("fireEvent");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<EventStrings>>> {
        return EventStrings_instance.with(Clone::clone).clone();
    }
}

