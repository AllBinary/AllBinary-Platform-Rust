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
// package org.allbinary.media.image.comparison.motion
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::cache::CacheInterface::CacheInterface;
use crate::org::allbinary::logic::util::cache::CacheInterfaceFactory::CacheInterfaceFactory;
use crate::org::allbinary::logic::util::cache::CachePolicyFactory::CachePolicyFactory;
use crate::org::allbinary::logic::util::cache::CacheTypeFactory::CacheTypeFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ConstrainedMotionRectanglesResultsCacheSingleton {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

pub static mut ConstrainedMotionRectanglesResultsCacheSingleton_cacheInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn CacheInterface>>>> = std::cell::LazyCell::new(|| None);

impl ConstrainedMotionRectanglesResultsCacheSingleton {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut STATIC_BLOCK: String = String::from("Static Block");
        let mut instance: String = String::from("MotionRectanglesResultsCacheSingleton");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(instance.clone()), STATIC_BLOCK.clone());
            unsafe { *ConstrainedMotionRectanglesResultsCacheSingleton_cacheInterface = CacheInterfaceFactory::getInstance(jcall!(CacheTypeFactory::getInstance()).CACHE.clone(), jcall!(CachePolicyFactory::getInstance()).ONE_MINUTE_FIVE_MAX.clone()) };
            jcall!(logUtil).putF(jcall!(commonStrings).END.clone(), crate::java_into_object(instance.clone()), STATIC_BLOCK.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            jcall!(logUtil).put(jcall!(commonStrings).EXCEPTION.clone(), crate::java_into_object(instance.clone()), STATIC_BLOCK.clone(), crate::java_into_object(e.clone()));
            }
        }
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn CacheInterface>>> {
        return unsafe { ConstrainedMotionRectanglesResultsCacheSingleton_cacheInterface.clone() }.clone();
    }
}

