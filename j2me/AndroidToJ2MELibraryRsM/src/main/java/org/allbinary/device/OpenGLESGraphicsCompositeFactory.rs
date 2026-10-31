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
// package org.allbinary.device
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::OpenGLESGraphicsFactory::OpenGLESGraphicsFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OpenGLESGraphicsCompositeFactory {
    pub openGLESGraphicsFactory: Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphicsFactory::OpenGLESGraphicsFactoryDyn>>>,
}

thread_local! { pub static OpenGLESGraphicsCompositeFactory_instance: Option<Rc<RefCell<OpenGLESGraphicsCompositeFactory>>> = OpenGLESGraphicsCompositeFactory::new(); }

impl OpenGLESGraphicsCompositeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            openGLESGraphicsFactory: None,
        };
        this.openGLESGraphicsFactory = OpenGLESGraphicsFactory::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphicsFactory::OpenGLESGraphicsFactoryDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<OpenGLESGraphicsCompositeFactory>>> {
        return OpenGLESGraphicsCompositeFactory_instance.with(Clone::clone).clone();
    }
    

    pub fn set(&mut self, mut openGLESGraphicsFactory: Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphicsFactory::OpenGLESGraphicsFactoryDyn>>>) {
        self.openGLESGraphicsFactory = openGLESGraphicsFactory.clone();
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::device::OpenGLESGraphicsFactory::OpenGLESGraphicsFactoryDyn>>> {
        return self.openGLESGraphicsFactory.clone();
    }
}

