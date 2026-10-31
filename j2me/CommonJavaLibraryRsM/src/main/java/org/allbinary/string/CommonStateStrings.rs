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
// package org.allbinary.string
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

pub struct CommonStateStrings {
    pub CONTEXT: String,
    pub FINISH: String,
    pub CREATE: String,
    pub START: String,
    pub RESTART: String,
    pub RESUME: String,
    pub PAUSE: String,
    pub STOP: String,
    pub DESTROY: String,
    pub BIND: String,
    pub ON_SAVE_INSTANCE_STATE: String,
    pub ON_START_COMMAND: String,
}

thread_local! { pub static CommonStateStrings_instance: Option<Rc<RefCell<CommonStateStrings>>> = CommonStateStrings::new(); }

impl CommonStateStrings {

    pub fn new_struct() -> Self {
        let mut this = Self {
            CONTEXT: String::new(),
            FINISH: String::new(),
            CREATE: String::new(),
            START: String::new(),
            RESTART: String::new(),
            RESUME: String::new(),
            PAUSE: String::new(),
            STOP: String::new(),
            DESTROY: String::new(),
            BIND: String::new(),
            ON_SAVE_INSTANCE_STATE: String::new(),
            ON_START_COMMAND: String::new(),
        };
        this.CONTEXT = String::from("Context: ");
        this.FINISH = String::from("finish");
        this.CREATE = String::from("onCreate");
        this.START = String::from("onStart");
        this.RESTART = String::from("onRestart");
        this.RESUME = String::from("onResume");
        this.PAUSE = String::from("onPause");
        this.STOP = String::from("onStop");
        this.DESTROY = String::from("onDestroy");
        this.BIND = String::from("onBind");
        this.ON_SAVE_INSTANCE_STATE = String::from("onSaveInstanceState");
        this.ON_START_COMMAND = String::from("onStartCommand");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<CommonStateStrings>>> {
        return CommonStateStrings_instance.with(Clone::clone).clone();
    }
}

