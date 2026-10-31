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
// package org.allbinary.android.api1
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil;
use crate::android::app::Activity::Activity;
use crate::android::view::WindowManager::WindowManager;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureUtilAPI1 {
    pub base: ActivityFractureUtil,
}

thread_local! { pub static ActivityFractureUtilAPI1_instance: Option<Rc<RefCell<dyn ActivityFractureUtilAPI1Dyn>>> = ActivityFractureUtilAPI1::new().map(|reference| reference as Rc<RefCell<dyn ActivityFractureUtilAPI1Dyn>>); }

impl ActivityFractureUtilAPI1 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ActivityFractureUtil::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ActivityFractureUtilAPI1Dyn>>> {
        return ActivityFractureUtilAPI1_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
    }
    

    pub fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        jcall!(jcall!(activity).getWindow()).setFlags(jcall!(unsafe { crate::android::view::WindowManager::WindowManager_LayoutParams.clone() }).FLAG_FULLSCREEN.clone(), jcall!(unsafe { crate::android::view::WindowManager::WindowManager_LayoutParams.clone() }).FLAG_FULLSCREEN.clone());
    }
}
impl std::ops::Deref for ActivityFractureUtilAPI1 {
    type Target = ActivityFractureUtil;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActivityFractureUtilAPI1 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ActivityFractureUtilAPI1Dyn: crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn {
    fn as_ActivityFractureUtilAPI1(&self) -> &ActivityFractureUtilAPI1;
    fn as_ActivityFractureUtilAPI1_mut(&mut self) -> &mut ActivityFractureUtilAPI1;
}
impl ActivityFractureUtilAPI1Dyn for ActivityFractureUtilAPI1 {
    fn as_ActivityFractureUtilAPI1(&self) -> &ActivityFractureUtilAPI1 { self }
    fn as_ActivityFractureUtilAPI1_mut(&mut self) -> &mut ActivityFractureUtilAPI1 { self }
}

impl crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn for ActivityFractureUtilAPI1 {
    fn as_ActivityFractureUtil(&self) -> &crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &self.base }
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &mut self.base }
    fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { Self::process(self, activity) }
    fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { Self::setFullScreen(self, activity) }
}


