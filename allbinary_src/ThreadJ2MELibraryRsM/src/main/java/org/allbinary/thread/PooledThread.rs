// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
use crate::Runnable;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::java::lang::Thread::*;
use super::ThreadObjectUtil::ThreadObjectUtil;
use super::ThreadPool::ThreadPool;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PooledThread {
    pub base: Thread,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub threadObjectUtil: Option<Rc<RefCell<ThreadObjectUtil>>>,
    pub threadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>>,
    pub INTERRUPT_EXCEPTION: String,
}

impl Runnable for PooledThread {
    fn run(&mut self) { Self::run(self) }
}

impl PooledThread {
    

    pub fn new_struct(mut threadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>>) -> Self {
        let mut this = Self {
            base: Thread::new_struct1(jcall!(threadPool).createName()),
            logUtil: None,
            commonStrings: None,
            threadObjectUtil: None,
            threadPool: None,
            INTERRUPT_EXCEPTION: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.threadObjectUtil = ThreadObjectUtil::getInstance();
        this.INTERRUPT_EXCEPTION = String::from("Exit InterruptedException");
        // super(...) is in the struct literal above
        jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        this.threadPool = threadPool.clone();
        this
    }

    pub fn new(mut threadPool: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(threadPool))))
    }
    

    pub fn run(&mut self) {
        jcall!(self.threadPool).threadStarted();
        while true {
            let mut task2: Option<Rc<RefCell<dyn Runnable>>> = jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>);
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                task2 = jcall!(self.threadPool).getTask();
                jcall!(self.threadPool).as_ThreadPool_mut().runningTask = true;
                jcall!(self.threadPool).startTask(task2.clone());
            })) {
                Ok(_) => {}
                Err(ex) => {
            let ex: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(ex as Box<dyn std::any::Any>)));

                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                jcall!(logUtil).putF(self.INTERRUPT_EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone());
                break;
                }
            }
            if match (&task2, &jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                break;
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                jcall!(task2).run();
                jcall!(self.threadPool).completedTask(task2.clone());
                jcall!(self.threadPool).as_ThreadPool_mut().runningTask = false;
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
                jcall!(logUtil).put(jcall!(StringMaker::new()).append(jcall!(self.commonStrings).EXCEPTION_LABEL.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(task2.clone()))).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(), crate::java_into_object(e.clone()));
                }
            }
        }
        jcall!(self.threadPool).threadStopped();
    }
}
impl std::ops::Deref for PooledThread {
    type Target = Thread;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PooledThread {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}



