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
use super::ProgressActivityInterface::ProgressActivityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct NullProgressActivity {
}

thread_local! { pub static NullProgressActivity_NULL_PROGRESS_ACTIVITY: Option<Rc<RefCell<NullProgressActivity>>> = NullProgressActivity::new(); }
impl ProgressActivityInterface for NullProgressActivity {
    fn runOnUiThread(&mut self, mut action: Option<Rc<RefCell<dyn Runnable>>>) { Self::runOnUiThread(self, action) }
    fn onDismissTitleProgressBar(&mut self) { Self::onDismissTitleProgressBar(self) }
    fn onShowTitleProgressBar(&mut self, mut value: i32, mut indeterminate: bool) { Self::onShowTitleProgressBar(self, value, indeterminate) }
    fn onTitleProgressBarSetProgress(&mut self, mut value: i32) { Self::onTitleProgressBarSetProgress(self, value) }
}

impl NullProgressActivity {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn runOnUiThread(&mut self, mut action: Option<Rc<RefCell<dyn Runnable>>>) {
    }
    

    pub fn onDismissTitleProgressBar(&mut self) {
    }
    

    pub fn onShowTitleProgressBar(&mut self, mut value: i32, mut indeterminate: bool) {
    }
    

    pub fn onTitleProgressBarSetProgress(&mut self, mut value: i32) {
    }
}

