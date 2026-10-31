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
// package org.allbinary.logic.util.queue
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicQueue {
    pub queueVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl BasicQueue {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            queueVector: None,
        };
        this.queueVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    // synchronized
    pub fn offer(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool {
        jcall!(self.queueVector).add(crate::java_into_object(anyType.clone()));
        return true;
    }
    

    // synchronized
    pub fn remove(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) {
        jcall!(self.queueVector).remove(crate::java_into_object(anyType.clone()));
    }
    

    // synchronized
    pub fn removeLast(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = { let java_receiver = self.queueVector.clone(); let java_arguments = (jcall!(self.queueVector).size() - 1,); let java_result = jcall!(java_receiver).get(java_arguments.0); java_result };
        jcall!(self.queueVector).remove(crate::java_into_object(anyType.clone()));
        return anyType.clone();
    }
}
pub trait BasicQueueDyn {
    fn as_BasicQueue(&self) -> &BasicQueue;
    fn as_BasicQueue_mut(&mut self) -> &mut BasicQueue;
    fn offer(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) -> bool { BasicQueue::offer(self.as_BasicQueue_mut(), anyType) }
    fn remove(&mut self, mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>>) { BasicQueue::remove(self.as_BasicQueue_mut(), anyType) }
    fn removeLast(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { BasicQueue::removeLast(self.as_BasicQueue_mut()) }
}
impl BasicQueueDyn for BasicQueue {
    fn as_BasicQueue(&self) -> &BasicQueue { self }
    fn as_BasicQueue_mut(&mut self) -> &mut BasicQueue { self }
}


