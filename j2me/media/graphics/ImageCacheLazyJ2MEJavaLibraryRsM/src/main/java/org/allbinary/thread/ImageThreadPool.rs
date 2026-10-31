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
use crate::Runnable;
use super::*;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use super::ThreadPool::ThreadPool;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//This is mainly for network communications.
pub struct ImageThreadPool {
    pub base: ThreadPool,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static ImageThreadPool_instance: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = ImageThreadPool::new(String::from("ImageLoader"), 1, ThreadPool_NORMAL_PRIORITY.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>); }

impl ImageThreadPool {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> {
        return ImageThreadPool_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Self {
        let mut this = Self {
            base: ThreadPool::new_struct(poolName.clone(), numThreads.clone(), priority.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(poolName, numThreads, priority))))
    }
    

    // synchronized
    pub fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        self.base.runTask(task.clone());
    }
    

    pub fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
    

    pub fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
}
impl std::ops::Deref for ImageThreadPool {
    type Target = ThreadPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ImageThreadPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn for ImageThreadPool {
    fn as_ThreadPool(&self) -> &crate::org::allbinary::thread::ThreadPool::ThreadPool { &self.base }
    fn as_ThreadPool_mut(&mut self) -> &mut crate::org::allbinary::thread::ThreadPool::ThreadPool { &mut self.base }
    fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::runTask(self, task) }
    fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::startTask(self, task) }
    fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::completedTask(self, task) }
}


