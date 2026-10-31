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
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::ThreadPool::ThreadPool;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//This is mainly for network communications.
pub struct SecondaryThreadPool {
    pub base: ThreadPool,
}

thread_local! { pub static SecondaryThreadPool_instance: Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> = SecondaryThreadPool::new(String::from("Secondary"), 1, ThreadPool_NORMAL_PRIORITY.with(Clone::clone).clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>); }

impl SecondaryThreadPool {
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn>>> {
        return SecondaryThreadPool_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Self {
        let mut this = Self {
            base: ThreadPool::new_struct(poolName.clone(), numThreads.clone(), priority.clone()),
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut poolName: String, mut numThreads: i32, mut priority: i32) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(poolName, numThreads, priority))))
    }
    

    pub fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(task.clone()))).appendlong((((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64)) as i64)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.threadPoolStrings).ADD_TASK.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
        self.base.runTask(task.clone());
    }
    

    pub fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.threadPoolStrings).START_TASK.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(task.clone()))).appendlong((((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64)) as i64)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
    

    pub fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) {
        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(jcall!(self.threadPoolStrings).COMPLETE_TASK.clone()).append(jcall!(StringUtil::getInstance()).toString(crate::java_into_object(task.clone()))).appendlong((((std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64)) as i64)).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).RUN.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
    }
}
impl std::ops::Deref for SecondaryThreadPool {
    type Target = ThreadPool;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for SecondaryThreadPool {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::thread::ThreadPool::ThreadPoolDyn for SecondaryThreadPool {
    fn as_ThreadPool(&self) -> &crate::org::allbinary::thread::ThreadPool::ThreadPool { &self.base }
    fn as_ThreadPool_mut(&mut self) -> &mut crate::org::allbinary::thread::ThreadPool::ThreadPool { &mut self.base }
    fn runTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::runTask(self, task) }
    fn startTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::startTask(self, task) }
    fn completedTask(&mut self, mut task: Option<Rc<RefCell<dyn Runnable>>>) { Self::completedTask(self, task) }
}


