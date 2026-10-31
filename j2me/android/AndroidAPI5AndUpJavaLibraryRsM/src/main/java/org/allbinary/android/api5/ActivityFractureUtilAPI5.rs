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
// package org.allbinary.android.api5
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::android::app::Activity::Activity;
use crate::org::allbinary::input::motion::button::TouchScreenTypeFactory::TouchScreenTypeFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::allbinary::logic::communication::log::PreLogUtil::PreLogUtil;
use crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1;
use crate::org::allbinary::input::motion::button::TouchScreenFactory::TouchScreenFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ActivityFractureUtilAPI5 {
    pub base: ActivityFractureUtilAPI1,
    pub FEATURE_TOUCHSCREEN_MULTITOUCH: String,
    pub FEATURE_TOUCHSCREEN: String,
    pub FEATURE_TOUCHSCREEN_MULTITOUCH_DISTINCT: String,
}

thread_local! { pub static ActivityFractureUtilAPI5_instance5: Option<Rc<RefCell<dyn ActivityFractureUtilAPI5Dyn>>> = ActivityFractureUtilAPI5::new().map(|reference| reference as Rc<RefCell<dyn ActivityFractureUtilAPI5Dyn>>); }

impl ActivityFractureUtilAPI5 {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: ActivityFractureUtilAPI1::new_struct(),
            FEATURE_TOUCHSCREEN_MULTITOUCH: String::new(),
            FEATURE_TOUCHSCREEN: String::new(),
            FEATURE_TOUCHSCREEN_MULTITOUCH_DISTINCT: String::new(),
        };
        this.FEATURE_TOUCHSCREEN_MULTITOUCH = String::from("android.hardware.touchscreen.multitouch");
        this.FEATURE_TOUCHSCREEN = String::from("android.hardware.touchscreen");
        this.FEATURE_TOUCHSCREEN_MULTITOUCH_DISTINCT = String::from("android.hardware.touchscreen.multitouch.distinct");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn ActivityFractureUtilAPI5Dyn>>> {
        return ActivityFractureUtilAPI5_instance5.with(Clone::clone).clone();
    }
    

    pub fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) {
        let mut touchScreenFactory: Option<Rc<RefCell<TouchScreenFactory>>> = TouchScreenFactory::getInstance();
        let mut touchScreenTypeFactory: Option<Rc<RefCell<TouchScreenTypeFactory>>> = TouchScreenTypeFactory::getInstance();
        jcall!(touchScreenFactory).setTouch(self.isTouch(activity.clone()));
        jcall!(touchScreenFactory).setMultiTouch(self.isMultiTouch(activity.clone()));
        jcall!(touchScreenFactory).setMultiTouchDistinct(self.isMultiTouchDistinct(activity.clone()));
        jcall!(touchScreenTypeFactory).update(activity.clone());
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        PreLogUtil::put(jcall!(touchScreenFactory).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).PROCESS.clone());
    }
    

    pub fn isTouch(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool {
        return self.isFeature(activity.clone(), self.FEATURE_TOUCHSCREEN.clone());
    }
    

    pub fn isMultiTouch(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool {
        return self.isFeature(activity.clone(), self.FEATURE_TOUCHSCREEN_MULTITOUCH.clone());
    }
    

    pub fn isMultiTouchDistinct(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool {
        return self.isFeature(activity.clone(), self.FEATURE_TOUCHSCREEN_MULTITOUCH_DISTINCT.clone());
    }
    

    pub fn isFeature(&mut self, mut activity: Option<Rc<RefCell<Activity>>>, mut property: String) -> bool {
        return jcall!(jcall!(activity).getPackageManager()).hasSystemFeature(property.clone());
    }
}
impl std::ops::Deref for ActivityFractureUtilAPI5 {
    type Target = ActivityFractureUtilAPI1;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for ActivityFractureUtilAPI5 {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait ActivityFractureUtilAPI5Dyn: crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1Dyn {
    fn as_ActivityFractureUtilAPI5(&self) -> &ActivityFractureUtilAPI5;
    fn as_ActivityFractureUtilAPI5_mut(&mut self) -> &mut ActivityFractureUtilAPI5;
    fn isTouch(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool { ActivityFractureUtilAPI5::isTouch(self.as_ActivityFractureUtilAPI5_mut(), activity) }
    fn isMultiTouch(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool { ActivityFractureUtilAPI5::isMultiTouch(self.as_ActivityFractureUtilAPI5_mut(), activity) }
    fn isMultiTouchDistinct(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) -> bool { ActivityFractureUtilAPI5::isMultiTouchDistinct(self.as_ActivityFractureUtilAPI5_mut(), activity) }
    fn isFeature(&mut self, mut activity: Option<Rc<RefCell<Activity>>>, mut property: String) -> bool { ActivityFractureUtilAPI5::isFeature(self.as_ActivityFractureUtilAPI5_mut(), activity, property) }
}
impl ActivityFractureUtilAPI5Dyn for ActivityFractureUtilAPI5 {
    fn as_ActivityFractureUtilAPI5(&self) -> &ActivityFractureUtilAPI5 { self }
    fn as_ActivityFractureUtilAPI5_mut(&mut self) -> &mut ActivityFractureUtilAPI5 { self }
}

impl crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1Dyn for ActivityFractureUtilAPI5 {
    fn as_ActivityFractureUtilAPI1(&self) -> &crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &self.base }
    fn as_ActivityFractureUtilAPI1_mut(&mut self) -> &mut crate::org::allbinary::android::api1::ActivityFractureUtilAPI1::ActivityFractureUtilAPI1 { &mut self.base }
}
impl crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtilDyn for ActivityFractureUtilAPI5 {
    fn as_ActivityFractureUtil(&self) -> &crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &self.base.base }
    fn as_ActivityFractureUtil_mut(&mut self) -> &mut crate::org::allbinary::android::ActivityFractureUtil::ActivityFractureUtil { &mut self.base.base }
    fn process(&mut self, mut activity: Option<Rc<RefCell<Activity>>>) { Self::process(self, activity) }
}


