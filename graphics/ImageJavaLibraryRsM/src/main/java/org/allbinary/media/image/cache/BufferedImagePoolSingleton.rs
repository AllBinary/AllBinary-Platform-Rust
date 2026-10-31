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
// package org.allbinary.media.image.cache
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::util::cache::CachePolicyFactory::CachePolicyFactory;
use crate::org::allbinary::logic::util::cache::PoolInterface::PoolInterface;
use crate::org::allbinary::logic::util::cache::PoolInterfaceFactory::PoolInterfaceFactory;
use crate::org::allbinary::logic::util::cache::PoolTypeFactory::PoolTypeFactory;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BufferedImageCacheableFactory::BufferedImageCacheableFactory;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BufferedImagePoolSingleton {
}

pub static mut BufferedImagePoolSingleton_poolInterface: std::cell::LazyCell<Option<Rc<RefCell<dyn PoolInterface>>>> = std::cell::LazyCell::new(|| None);

impl BufferedImagePoolSingleton {
    

    // Java static initializer: call this once at startup
    pub fn static_init() {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        let mut STATIC_BLOCK: String = String::from("Static Block");
        let mut instance: String = String::from("BufferedImagePoolSingleton");
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(logUtil).putF(jcall!(commonStrings).START.clone(), crate::java_into_object(instance.clone()), STATIC_BLOCK.clone());
            unsafe { *BufferedImagePoolSingleton_poolInterface = PoolInterfaceFactory::getInstance(BufferedImageCacheableFactory::new(), jcall!(PoolTypeFactory::getInstance()).VECTOR_POOL.clone(), jcall!(CachePolicyFactory::getInstance()).MAX_TIME_THOUSAND_MAX.clone()) };
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
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<dyn PoolInterface>>> {
        return unsafe { BufferedImagePoolSingleton_poolInterface.clone() }.clone();
    }
}

