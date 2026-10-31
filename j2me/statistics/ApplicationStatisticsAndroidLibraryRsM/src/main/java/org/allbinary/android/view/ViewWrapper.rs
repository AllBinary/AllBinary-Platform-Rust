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
use crate::android::view::View::View;
use super::BaseViewWrapper::BaseViewWrapper;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ViewWrapper {
    pub base: BaseViewWrapper,
    pub view: Option<Rc<RefCell<View>>>,
}


impl ViewWrapper {
    

    pub fn new_struct(mut view: Option<Rc<RefCell<View>>>) -> Self {
        let mut this = Self {
            base: BaseViewWrapper::new_struct(),
            view: None,
        };
        this.view = view.clone();
        this
    }

    pub fn new(mut view: Option<Rc<RefCell<View>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(view))))
    }
    

    pub fn invalidate(&mut self) {
        jcall!(self.view).invalidate();
    }
    

    pub fn postInvalidate(&mut self) {
        jcall!(self.view).postInvalidate();
    }
}
impl std::ops::Deref for ViewWrapper {
    type Target = BaseViewWrapper;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ViewWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapperDyn for ViewWrapper {
    fn as_BaseViewWrapper(&self) -> &crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapper { &self.base }
    fn as_BaseViewWrapper_mut(&mut self) -> &mut crate::org::allbinary::android::view::BaseViewWrapper::BaseViewWrapper { &mut self.base }
    fn invalidate(&mut self) { Self::invalidate(self) }
    fn postInvalidate(&mut self) { Self::postInvalidate(self) }
}


