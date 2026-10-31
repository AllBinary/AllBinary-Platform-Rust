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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::ClientInformation::ClientInformation;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ClientInformationFactory {
}


impl ClientInformationFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn>>> {
        let mut logUtil: Option<Rc<RefCell<LogUtil>>> = LogUtil::getInstance();
        let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
        jcall!(logUtil).putF(jcall!(commonStrings).NOT_IMPLEMENTED.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).GET_INSTANCE.clone());
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        return ClientInformation::new(jcall!(stringUtil).NULL_STRING.clone(), jcall!(stringUtil).NULL_STRING.clone(), jcall!(stringUtil).NULL_STRING.clone(), jcall!(stringUtil).NULL_STRING.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::system::security::licensing::ClientInformation::ClientInformationDyn>>);
    }
}

