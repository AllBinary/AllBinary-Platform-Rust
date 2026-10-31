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
// package org.allbinary.logic.communication.smtp.queue
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::javax::mail::Transport::Transport;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::smtp::EmailInterface::EmailInterface;
use crate::org::allbinary::logic::util::queue::BasicQueue::BasicQueue;
use super::EmailQueueInterface::EmailQueueInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicEmailQueue {
    pub base: BasicQueue,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static BasicEmailQueue_instance: Option<Rc<RefCell<BasicEmailQueue>>> = BasicEmailQueue::new(); }
pub static mut BasicEmailQueue_basicEmailQueue: std::cell::LazyCell<Option<Rc<RefCell<BasicEmailQueue>>>> = std::cell::LazyCell::new(|| None);
impl EmailQueueInterface for BasicEmailQueue {
    fn offer(&mut self, mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>>) -> bool { Self::offer(self, emailInterface) }
    fn remove(&mut self) -> Option<Rc<RefCell<dyn EmailInterface>>> { Self::remove(self) }
}

impl BasicEmailQueue {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: BasicQueue::new_struct(),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // synchronized
    pub fn getInstance() -> Option<Rc<RefCell<BasicEmailQueue>>> {
        if unsafe { BasicEmailQueue_basicEmailQueue.clone() }.is_none() {
            unsafe { *BasicEmailQueue_basicEmailQueue = BasicEmailQueue::new() };
        }
        return unsafe { BasicEmailQueue_basicEmailQueue.clone() }.clone();
    }
    

    // throws Exception
    // synchronized
    pub fn offer(&mut self, mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>>) -> bool {
        self.offer((emailInterface).clone() /* as Any */);
        self.post(emailInterface.clone());
        return true;
    }
    

    // synchronized
    pub fn remove1(&mut self, mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>>) {
        self.remove1((emailInterface).clone() /* as Any */);
    }
    

    // Java overloaded method: Rust needs a different name
    // synchronized
    pub fn remove(&mut self) -> Option<Rc<RefCell<dyn EmailInterface>>> {
        return None;
    }
    

    // throws Exception
    // synchronized
    pub fn post(&mut self, mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>>) {
        if self.send(emailInterface.clone()) {
            self.remove1(emailInterface.clone());
            self.processAllUnsent();
        }
    }
    

    // throws Exception
    // synchronized
    pub fn processAllUnsent(&mut self) {
        let mut size: i32 = jcall!(self.base.queueVector).size();
        for index in (0..size) {
            let mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>> = { let java_object = jcall!(self.base.queueVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn EmailInterface>>>>().expect("ClassCastException").clone()) };
            if self.send(emailInterface.clone()) {
                self.remove1(emailInterface.clone());
                self.processAllUnsent();
            }
        }
    }
    

    // synchronized
    pub fn send(&mut self, mut emailInterface: Option<Rc<RefCell<dyn EmailInterface>>>) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Sending: "), jcall!(emailInterface).log()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("send"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            Transport::send(jcall!(emailInterface).getMimeMessage());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Email Send Debug: "), jcall!(emailInterface).getDebugInfo()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("send"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).EMAILLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Failed Email Send Debug: "), jcall!(emailInterface).getDebugInfo()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("send"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            return false;
            }
        }
    }
}
impl std::ops::Deref for BasicEmailQueue {
    type Target = BasicQueue;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicEmailQueue {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::util::queue::BasicQueue::BasicQueueDyn for BasicEmailQueue {
    fn as_BasicQueue(&self) -> &crate::org::allbinary::logic::util::queue::BasicQueue::BasicQueue { &self.base }
    fn as_BasicQueue_mut(&mut self) -> &mut crate::org::allbinary::logic::util::queue::BasicQueue::BasicQueue { &mut self.base }
}


