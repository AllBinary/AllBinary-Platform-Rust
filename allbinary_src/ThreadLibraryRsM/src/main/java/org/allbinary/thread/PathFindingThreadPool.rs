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
use super::ThreadPool::ThreadPool;
use super::PriorityRunnable::PriorityRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//This is mainly for network communications.
pub struct PathFindingThreadPool {
    pub base: ThreadPool,
}

thread_local! { pub static PathFindingThreadPool_instance: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = PathFindingThreadPool::new(String::from("PathFinding"), 1, 1).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>); }

impl PathFindingThreadPool {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> {
        return PathFindingThreadPool_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Self {
        let mut this = Self {
            base: ThreadPool::new_struct(poolName.clone(), numThreads.clone(), ThreadPool_NORMAL_PRIORITY.with(Clone::clone).clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(poolName, numThreads, priority))))
    }
    

    pub fn runTaskWithPriority(&mut self, mut task: Option<Rc<RefCell<dyn PriorityRunnable>>>) {
        self.base.runTaskWithPriority(task.clone());
    }
    

    pub fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        self.base.runTask(task.clone());
    }
    

    pub fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
    

    pub fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
}
impl std::ops::Deref for PathFindingThreadPool {
    type Target = ThreadPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PathFindingThreadPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn for PathFindingThreadPool {
    fn as_ThreadPool(&self) -> &crate::org::allbinary::thread::ThreadPool::ThreadPool { &self.base }
    fn as_ThreadPool_mut(&mut self) -> &mut crate::org::allbinary::thread::ThreadPool::ThreadPool { &mut self.base }
    fn runTaskWithPriority(&mut self, mut task: Option<Rc<RefCell<dyn PriorityRunnable>>>) { Self::runTaskWithPriority(self, task) }
    fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::runTask(self, task) }
    fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::startTask(self, task) }
    fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::completedTask(self, task) }
}


