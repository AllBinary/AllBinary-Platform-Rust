// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BaseImageLoadingProcessor {
}

thread_local! { pub static BaseImageLoadingProcessor_instance: Option<Rc<RefCell<dyn BaseImageLoadingProcessorDyn>>> = BaseImageLoadingProcessor::new().map(|reference| reference as Rc<RefCell<dyn BaseImageLoadingProcessorDyn>>); }

impl BaseImageLoadingProcessor {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn BaseImageLoadingProcessorDyn>>> {
        return BaseImageLoadingProcessor_instance.with(Clone::clone).clone();
    }
    

    pub fn runTask(&mut self) {
    }
}
pub trait BaseImageLoadingProcessorDyn {
    fn as_BaseImageLoadingProcessor(&self) -> &BaseImageLoadingProcessor;
    fn as_BaseImageLoadingProcessor_mut(&mut self) -> &mut BaseImageLoadingProcessor;
    fn runTask(&mut self) { BaseImageLoadingProcessor::runTask(self.as_BaseImageLoadingProcessor_mut()) }
}
impl BaseImageLoadingProcessorDyn for BaseImageLoadingProcessor {
    fn as_BaseImageLoadingProcessor(&self) -> &BaseImageLoadingProcessor { self }
    fn as_BaseImageLoadingProcessor_mut(&mut self) -> &mut BaseImageLoadingProcessor { self }
}


