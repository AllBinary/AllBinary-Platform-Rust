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
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::java::lang::Thread::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThreadUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub JOIN: String,
}

thread_local! { pub static ThreadUtil_instance: Option<Rc<RefCell<ThreadUtil>>> = ThreadUtil::new(); }

impl ThreadUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            JOIN: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.JOIN = String::from("join");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThreadUtil>>> {
        return ThreadUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn isRunning(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) -> bool {
        if thread.is_some() && jcall!(thread).isAlive() {
            return true;
        } else {
            return false;
        }
    }
    

    // throws Exception
    pub fn join(&mut self, mut thread: Option<Rc<RefCell<Thread>>>) {
        if jcall!(ThreadUtil::getInstance()).isRunning(thread.clone()) {
            jcall!(self.logUtil).putF(String::from("Waiting for Thread To Join/End"), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.JOIN.clone());
            jcall!(thread).join();
        }
    }
}

