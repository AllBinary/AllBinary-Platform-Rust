// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.android.view
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

pub struct BaseViewWrapper {
}

thread_local! { pub static BaseViewWrapper_NULL_VIEW_WRAPPER: Option<Rc<RefCell<dyn BaseViewWrapperDyn>>> = BaseViewWrapper::new().map(|reference| reference as Rc<RefCell<dyn BaseViewWrapperDyn>>); }

impl BaseViewWrapper {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn invalidate(&mut self) {
    }
    

    pub fn postInvalidate(&mut self) {
    }
}
pub trait BaseViewWrapperDyn {
    fn as_BaseViewWrapper(&self) -> &BaseViewWrapper;
    fn as_BaseViewWrapper_mut(&mut self) -> &mut BaseViewWrapper;
    fn invalidate(&mut self) { BaseViewWrapper::invalidate(self.as_BaseViewWrapper_mut()) }
    fn postInvalidate(&mut self) { BaseViewWrapper::postInvalidate(self.as_BaseViewWrapper_mut()) }
}
impl BaseViewWrapperDyn for BaseViewWrapper {
    fn as_BaseViewWrapper(&self) -> &BaseViewWrapper { self }
    fn as_BaseViewWrapper_mut(&mut self) -> &mut BaseViewWrapper { self }
}


