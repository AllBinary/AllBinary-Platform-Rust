// Generated Code Do Not Modify
// package org.allbinary.thread
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::org::allbinary::TsUtil::TsUtil;
use crate::java::lang::Thread::*;
use super::NullPriorityRunnable::NullPriorityRunnable;
use super::PriorityRunnable::PriorityRunnable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ThreadObjectUtil {
    pub NULL_PRIORITY_RUNNABLE: Option<Rc<RefCell<dyn PriorityRunnable>>>,
}

thread_local! { pub static ThreadObjectUtil_instance: Option<Rc<RefCell<ThreadObjectUtil>>> = ThreadObjectUtil::new(); }

impl ThreadObjectUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            NULL_PRIORITY_RUNNABLE: None,
        };
        this.NULL_PRIORITY_RUNNABLE = NullPriorityRunnable::new().map(|reference| reference as Rc<RefCell<dyn PriorityRunnable>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ThreadObjectUtil>>> {
        return ThreadObjectUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn notifyObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        crate::java::lang::ObjectMonitor::notify();
    }
    

    // throws Exception
    pub fn waitObject(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        crate::java::lang::ObjectMonitor::wait();
    }
    

    // throws Exception
    pub fn waitObjectFor(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>, mut delay: i64) {
        jcall!(TsUtil::getInstance()).waitFor(crate::java_into_object(anyType.clone()), delay.clone());
    }
    

    pub fn processThread(&mut self, mut runnable: Option<Rc<RefCell<dyn Runnable>>>) {
        let mut thread: Option<Rc<RefCell<Thread>>> = Thread::new1(runnable.clone());
        jcall!(thread).start();
    }
}

