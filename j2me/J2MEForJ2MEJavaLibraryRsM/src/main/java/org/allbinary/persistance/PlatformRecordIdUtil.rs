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
// package org.allbinary.persistance
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PlatformRecordIdUtil {
    pub PLATFORM_SHORT_NAME: String,
}

thread_local! { pub static PlatformRecordIdUtil_instance: Option<Rc<RefCell<PlatformRecordIdUtil>>> = PlatformRecordIdUtil::new(); }

impl PlatformRecordIdUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            PLATFORM_SHORT_NAME: String::new(),
        };
        this.PLATFORM_SHORT_NAME = String::from("JME");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PlatformRecordIdUtil>>> {
        return PlatformRecordIdUtil_instance.with(Clone::clone).clone();
    }
    

    pub fn getRecordId(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut baseRecordId: String) -> String {
        return jcall!(StringMaker::new()).append(jcall!(abeClientInformation).toShortString()).append(jcall!(CommonSeps::getInstance()).UNDERSCORE.clone()).append(self.PLATFORM_SHORT_NAME.clone()).append(baseRecordId.clone()).toString();
    }
}

