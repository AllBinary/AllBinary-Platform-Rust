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
// package org.allbinary.android.activity
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Runnable;
use super::*;
use crate::android::view::View::View;
use super::KeyInputViewCompositeInterface::KeyInputViewCompositeInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullKeyInputViewComposite {
}

thread_local! { pub static NullKeyInputViewComposite_NULL_KEY_INPUT_VIEW_COMPOSITE: Option<Rc<RefCell<NullKeyInputViewComposite>>> = NullKeyInputViewComposite::new(); }
impl KeyInputViewCompositeInterface for NullKeyInputViewComposite {
    fn setKeyInputView(&mut self, mut keyInputView: Option<Rc<RefCell<View>>>) { Self::setKeyInputView(self, keyInputView) }
}
impl crate::org::allbinary::android::activity::ViewCompositeInterface::ViewCompositeInterface for NullKeyInputViewComposite {
    fn getRootView(&mut self) -> Option<Rc<RefCell<View>>> { Self::getRootView(self) }
    fn getView(&mut self) -> Option<Rc<RefCell<View>>> { Self::getView(self) }
    fn runOnUiThread(&mut self, mut action: Option<Rc<RefCell<dyn Runnable>>>) { Self::runOnUiThread(self, action) }
}

impl NullKeyInputViewComposite {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn setKeyInputView(&mut self, mut keyInputView: Option<Rc<RefCell<View>>>) {
    }
    

    pub fn getRootView(&mut self) -> Option<Rc<RefCell<View>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn getView(&mut self) -> Option<Rc<RefCell<View>>> {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    pub fn runOnUiThread(&mut self, mut action: Option<Rc<RefCell<dyn Runnable>>>) {
    }
}

