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
// package org.allbinary.android.api16
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::view::View::View;
use crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureUtilAPI16 {
    pub base: ActivityFractureUtilAPI5,
}

thread_local! { pub static ActivityFractureUtilAPI16_instance16: Option<Rc<RefCell<ActivityFractureUtilAPI16>>> = ActivityFractureUtilAPI16::new(); }

impl ActivityFractureUtilAPI16 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ActivityFractureUtilAPI5::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ActivityFractureUtilAPI16>>> {
        return ActivityFractureUtilAPI16_instance16.with(Clone::clone).clone();
    }
    

    pub fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        let mut decorView: Option<Rc<RefCell<View>>> = jcall!(jcall!(activity).getWindow()).getDecorView();
        let mut visibility: i32 = unsafe { crate::android::view::View::View_SYSTEM_UI_FLAG_HIDE_NAVIGATION.clone() } | unsafe { crate::android::view::View::View_SYSTEM_UI_FLAG_FULLSCREEN.clone() };
        jcall!(decorView).setSystemUiVisibility(visibility.clone());
    }
}
impl std::ops::Deref for ActivityFractureUtilAPI16 {
    type Target = ActivityFractureUtilAPI5;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActivityFractureUtilAPI16 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5Dyn for ActivityFractureUtilAPI16 {
    fn as_ActivityFractureUtilAPI5(&self) -> &crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5 { &self.base }
    fn as_ActivityFractureUtilAPI5_mut(&mut self) -> &mut crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5 { &mut self.base }
}
impl crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1Dyn for ActivityFractureUtilAPI16 {
    fn as_ActivityFractureUtilAPI1(&self) -> &crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &self.base.base }
    fn as_ActivityFractureUtilAPI1_mut(&mut self) -> &mut crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &mut self.base.base }
    fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { Self::setFullScreen(self, activity) }
}


