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
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use super::NullRunnable::NullRunnable;
use super::ThreadPoolStrings::ThreadPoolStrings;
use super::ThreadObjectUtil::ThreadObjectUtil;
use super::PriorityRunnable::PriorityRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThreadPool {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub NULL_RUNNABLE: Option<Rc<RefCell<NullRunnable>>>,
    pub threadPoolStrings: Option<Rc<RefCell<ThreadPoolStrings>>>,
    pub threadObjectUtil: Option<Rc<RefCell<ThreadObjectUtil>>>,
    pub isAlive: bool,
    pub taskQueue: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
    pub numThreads: i32,
    pub runningTask: bool,
    pub currentPriorityRunnable: Option<Rc<RefCell<dyn PriorityRunnable>>>,
}

thread_local! { pub static ThreadPool_NORMAL_PRIORITY: i32 = 5; }

impl ThreadPool {
    

    pub fn new_struct(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            NULL_RUNNABLE: None,
            threadPoolStrings: None,
            threadObjectUtil: None,
            isAlive: false,
            taskQueue: None,
            numThreads: 0,
            runningTask: false,
            currentPriorityRunnable: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.NULL_RUNNABLE = NullRunnable::getInstance();
        this.threadPoolStrings = ThreadPoolStrings::getInstance();
        this.threadObjectUtil = ThreadObjectUtil::getInstance();
        this.taskQueue = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this.currentPriorityRunnable = jcall!(this.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone();
        this
    }

    pub fn new(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(poolName, numThreads, priority))))
    }
    

    // throws Exception
    pub fn runAPriorityTask(&mut self) {
        if !jcall!(self.currentPriorityRunnable).isDone() {
            jcall!(self.currentPriorityRunnable).run();
        } else {
            let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = self.getTask();
            if match (&runnable, &self.NULL_RUNNABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                return;
            }
            self.currentPriorityRunnable = panic!("ClassCastException: a Rust dyn Runnable cannot be cast to dyn PriorityRunnable");
            if !(match (&self.currentPriorityRunnable, &jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false }) {
                jcall!(self.currentPriorityRunnable).reset();
                jcall!(self.currentPriorityRunnable).run();
            }
        }
    }
    

    // throws Exception
    pub fn runATask(&mut self) {
        let mut runnable: Option<Rc<RefCell<dyn Runnable>>> = self.getTask();
        if runnable.is_some() {
            jcall!(runnable).run();
        }
    }
    

    pub fn init(&mut self) {
        if !self.isAlive {
            self.isAlive = true;
            self.taskQueue = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        }
    }
    

    // synchronized
    pub fn runTaskWithPriority(&mut self, mut task: Option<Rc<RefCell<dyn PriorityRunnable>>>) {
        if !self.isAlive {
            self.init();
        }
        if task.is_some() {
            let mut size: i32 = jcall!(self.taskQueue).size();
            let mut runnable: Option<Rc<RefCell<dyn PriorityRunnable>>> = None;
            let mut lowerPriorityRunnable: Option<Rc<RefCell<dyn PriorityRunnable>>> = jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone();
            for index in (0..size) {
                runnable = { let java_object = jcall!(self.taskQueue).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn PriorityRunnable>>>>().expect("ClassCastException").clone()) };
                if jcall!(runnable).getPriority() > jcall!(task).getPriority() {
                    lowerPriorityRunnable = runnable.clone();
                    break;
                }
            }
            if match (&lowerPriorityRunnable, &jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(self.taskQueue).add(crate::java_into_object(task.clone()));
            } else {
                let mut index: i32 = jcall!(self.taskQueue).indexOf(crate::java_into_object(lowerPriorityRunnable.clone()));
                jcall!(self.taskQueue).addAt(index.clone(), crate::java_into_object(task.clone()));
            }
        }
    }
    

    // synchronized
    pub fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        if !self.isAlive {
            self.init();
        }
        if task.is_some() {
            jcall!(self.taskQueue).add(crate::java_into_object(task.clone()));
        }
    }
    

    // throws InterruptedException
    // synchronized
    pub fn getTask(&mut self) -> Option<Rc<RefCell<dyn Runnable>>> {
        if jcall!(self.taskQueue).isEmpty() {
            return self.NULL_RUNNABLE.clone().map(|reference| reference as Rc<RefCell<dyn Runnable>>);
        }
        return { let java_object = jcall!(self.taskQueue).removeAt(0); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn Runnable>>>>().expect("ClassCastException").clone()) };
    }
    

    // synchronized
    pub fn clear(&mut self) {
        jcall!(self.taskQueue).clear();
    }
    

    // synchronized
    pub fn close(&mut self) {
        if self.isAlive {
            self.isAlive = false;
            jcall!(self.taskQueue).clear();
            self.currentPriorityRunnable = jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone();
        }
    }
    

    pub fn join(&mut self) {
        self.isAlive = false;
        jcall!(self.taskQueue).clear();
        self.currentPriorityRunnable = jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone();
    }
    

    pub fn isBusy(&mut self) -> bool {
        if !self.isAlive {
            return false;
        }
        if jcall!(self.taskQueue).size() > 0 {
            return true;
        }
        if self.runningTask {
            return true;
        }
        return false;
    }
    

    pub fn threadStarted(&mut self) {
    }
    

    pub fn threadStopped(&mut self) {
        if self.numThreads == 1 {
            self.isAlive = false;
            jcall!(self.taskQueue).clear();
            self.currentPriorityRunnable = jcall!(self.threadObjectUtil).NULL_PRIORITY_RUNNABLE.clone();
        }
    }
    

    pub fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
    

    pub fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
    }
}
pub trait ThreadPoolDyn {
    fn as_ThreadPool(&self) -> &ThreadPool;
    fn as_ThreadPool_mut(&mut self) -> &mut ThreadPool;
    fn runAPriorityTask(&mut self) { ThreadPool::runAPriorityTask(self.as_ThreadPool_mut()) }
    fn runATask(&mut self) { ThreadPool::runATask(self.as_ThreadPool_mut()) }
    fn init(&mut self) { ThreadPool::init(self.as_ThreadPool_mut()) }
    fn runTaskWithPriority(&mut self, mut task: Option<Rc<RefCell<dyn PriorityRunnable>>>) { ThreadPool::runTaskWithPriority(self.as_ThreadPool_mut(), task) }
    fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { ThreadPool::runTask(self.as_ThreadPool_mut(), task) }
    fn getTask(&mut self) -> Option<Rc<RefCell<dyn Runnable>>> { ThreadPool::getTask(self.as_ThreadPool_mut()) }
    fn clear(&mut self) { ThreadPool::clear(self.as_ThreadPool_mut()) }
    fn close(&mut self) { ThreadPool::close(self.as_ThreadPool_mut()) }
    fn join(&mut self) { ThreadPool::join(self.as_ThreadPool_mut()) }
    fn isBusy(&mut self) -> bool { ThreadPool::isBusy(self.as_ThreadPool_mut()) }
    fn threadStarted(&mut self) { ThreadPool::threadStarted(self.as_ThreadPool_mut()) }
    fn threadStopped(&mut self) { ThreadPool::threadStopped(self.as_ThreadPool_mut()) }
    fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { ThreadPool::startTask(self.as_ThreadPool_mut(), task) }
    fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { ThreadPool::completedTask(self.as_ThreadPool_mut(), task) }
}
impl ThreadPoolDyn for ThreadPool {
    fn as_ThreadPool(&self) -> &ThreadPool { self }
    fn as_ThreadPool_mut(&mut self) -> &mut ThreadPool { self }
}


