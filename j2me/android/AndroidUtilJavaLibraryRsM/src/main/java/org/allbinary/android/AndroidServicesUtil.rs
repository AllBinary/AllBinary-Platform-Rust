// 
// AllBinary Open License Version 1 
// Copyright (c) 2019 AllBinary 
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
// use crate::java::util::List::List;
use crate::android::app::ActivityManager::ActivityManager;
use crate::android::app::ActivityManager::RunningServiceInfo::RunningServiceInfo;
use crate::android::content::ComponentName::ComponentName;
use crate::android::content::Context::Context;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AndroidServicesUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub resourceUtil: Option<Rc<RefCell<ResourceUtil>>>,
    pub SERVICE_LIMIT_MAX: i32,
    pub IS_SERVICE_RUNNING: String,
    pub SERVICE_NOT_FOUND_RUNNING: String,
}

thread_local! { pub static AndroidServicesUtil_instance: Option<Rc<RefCell<AndroidServicesUtil>>> = AndroidServicesUtil::new(); }

impl AndroidServicesUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            resourceUtil: None,
            SERVICE_LIMIT_MAX: 0,
            IS_SERVICE_RUNNING: String::new(),
            SERVICE_NOT_FOUND_RUNNING: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.resourceUtil = ResourceUtil::getInstance();
        this.SERVICE_LIMIT_MAX = 1000;
        this.IS_SERVICE_RUNNING = String::from("isServiceRunning");
        this.SERVICE_NOT_FOUND_RUNNING = String::from("Service not found Running: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<AndroidServicesUtil>>> {
        return AndroidServicesUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn isServiceRunning(&mut self, mut name: String) -> bool {
        let mut activityManager: Option<Rc<RefCell<ActivityManager>>> = (jcall!(jcall!(self.resourceUtil).getContext()).getSystemService(unsafe { crate::android::content::Context::Context_ACTIVITY_SERVICE.clone() }.clone())).clone() /* as ActivityManager */;
        let mut runningServicesList: Option<Rc<RefCell<dyn List<Option<Rc<RefCell<RunningServiceInfo>>>>>>> = jcall!(activityManager).getRunningServices(self.SERVICE_LIMIT_MAX.clone());
        let mut runningServiceInfo: Option<Rc<RefCell<RunningServiceInfo>>> = None;
        let mut serviceComponent: Option<Rc<RefCell<ComponentName>>> = None;
        let mut serviceName: String = String::new();
        let mut size: i32 = jcall!(runningServicesList).size();
        for index in (0..size) {
            runningServiceInfo = jcall!(runningServicesList).get(index.clone());
            serviceComponent = jcall!(runningServiceInfo).service.clone();
            serviceName = jcall!(serviceComponent).toString();
            if serviceName.find(&name.clone()).map(|index| index as i32).unwrap_or(-1) >= 0 {
                return true;
            }
        }
        jcall!(self.logUtil).putF(format!("{}{}", self.SERVICE_NOT_FOUND_RUNNING, name), crate::java_into_object(String::from(std::any::type_name::<Self>())), self.IS_SERVICE_RUNNING.clone());
        return false;
    }
}

