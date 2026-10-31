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
// package org.allbinary.android
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureUtil {
}


impl ActivityFractureUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
    }
    

    pub fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
    }
}
pub trait ActivityFractureUtilDyn {
    fn as_ActivityFractureUtil(&self) -> &ActivityFractureUtil;
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut ActivityFractureUtil;
    fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { ActivityFractureUtil::process(self.as_ActivityFractureUtil_mut(), activity) }
    fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { ActivityFractureUtil::setFullScreen(self.as_ActivityFractureUtil_mut(), activity) }
}
impl ActivityFractureUtilDyn for ActivityFractureUtil {
    fn as_ActivityFractureUtil(&self) -> &ActivityFractureUtil { self }
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut ActivityFractureUtil { self }
}


