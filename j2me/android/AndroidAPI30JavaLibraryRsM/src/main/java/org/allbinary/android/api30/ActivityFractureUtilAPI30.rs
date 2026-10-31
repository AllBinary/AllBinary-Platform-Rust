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
// package org.allbinary.android.api30
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::android::view::Window::Window;
use crate::androidx::core::view::WindowCompat::WindowCompat;
use crate::androidx::core::view::WindowInsetsCompat::WindowInsetsCompat;
use crate::androidx::core::view::WindowInsetsControllerCompat::WindowInsetsControllerCompat;
use crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureUtilAPI30 {
    pub base: ActivityFractureUtilAPI5,
}

thread_local! { pub static ActivityFractureUtilAPI30_instance30: Option<Rc<RefCell<ActivityFractureUtilAPI30>>> = ActivityFractureUtilAPI30::new(); }

impl ActivityFractureUtilAPI30 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ActivityFractureUtilAPI5::new_struct(),
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<ActivityFractureUtilAPI30>>> {
        return ActivityFractureUtilAPI30_instance30.with(Clone::clone).clone();
    }
    

    pub fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        let mut window: Option<Rc<RefCell<Window>>> = jcall!(activity).getWindow();
        let mut windowInsetsController: Option<Rc<RefCell<WindowInsetsControllerCompat>>> = WindowCompat::getInsetsController(window.clone(), jcall!(window).getDecorView());
        jcall!(windowInsetsController).hide(jcall!(unsafe { crate::androidx::core::view::WindowInsetsCompat::WindowInsetsCompat_Type.clone() }).systemBars());
        jcall!(windowInsetsController).setSystemBarsBehavior(unsafe { crate::androidx::core::view::WindowInsetsControllerCompat::WindowInsetsControllerCompat_BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE.clone() }.clone());
    }
}
impl std::ops::Deref for ActivityFractureUtilAPI30 {
    type Target = ActivityFractureUtilAPI5;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActivityFractureUtilAPI30 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5Dyn for ActivityFractureUtilAPI30 {
    fn as_ActivityFractureUtilAPI5(&self) -> &crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5 { &self.base }
    fn as_ActivityFractureUtilAPI5_mut(&mut self) -> &mut crate::org::allbinary::android::api5::ActivityFractureUtilAPI5::ActivityFractureUtilAPI5 { &mut self.base }
}
impl crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1Dyn for ActivityFractureUtilAPI30 {
    fn as_ActivityFractureUtilAPI1(&self) -> &crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &self.base.base }
    fn as_ActivityFractureUtilAPI1_mut(&mut self) -> &mut crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &mut self.base.base }
    fn setFullScreen(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { Self::setFullScreen(self, activity) }
}


