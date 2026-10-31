// 
// AllBinary Open License Version 1 
// Copyright (c) 2022 AllBinary 
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
// package org.allbinary.logic.system
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::InputStream::InputStream;
use crate::org::allbinary::data::resource::ResourceUtil::ResourceUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformAssetManager {
}

thread_local! { pub static PlatformAssetManager_instance: Option<Rc<RefCell<PlatformAssetManager>>> = PlatformAssetManager::new(); }

impl PlatformAssetManager {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlatformAssetManager>>> {
        return PlatformAssetManager_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getResourceAsStream(&mut self, mut resource: String) -> Option<Rc<RefCell<InputStream>>> {
        let mut resourceUtil: Option<Rc<RefCell<ResourceUtil>>> = ResourceUtil::getInstance();
        let mut inputStream: Option<Rc<RefCell<InputStream>>> = jcall!(resourceUtil).getResourceAsStream(resource.clone());
        return inputStream.clone();
    }
}

