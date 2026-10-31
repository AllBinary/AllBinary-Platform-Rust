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
// package org.allbinary
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::eclipse::swt::graphics::Resource::Resource;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DisposalUtil {
}

thread_local! { pub static DisposalUtil_instance: Option<Rc<RefCell<DisposalUtil>>> = DisposalUtil::new(); }

impl DisposalUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<DisposalUtil>>> {
        return DisposalUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn dispose(&mut self, mut resource: Option<Rc<RefCell<dyn crate::org::eclipse::swt::graphics::Resource::ResourceDyn>>>) {
        if !jcall!(resource).isDisposed() {
            jcall!(resource).dispose();
        }
    }
}

