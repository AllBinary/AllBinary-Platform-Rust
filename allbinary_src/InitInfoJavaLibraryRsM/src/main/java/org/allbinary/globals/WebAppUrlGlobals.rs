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
// package org.allbinary.globals
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::init::InitInfo::InitInfo;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use super::UrlGlobalsInterface::UrlGlobalsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct WebAppUrlGlobals {
    pub base: UrlGlobalsInterface,
    pub path: String,
    pub isWebappPathSet: bool,
}


impl WebAppUrlGlobals {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: UrlGlobalsInterface::new_struct(),
            path: String::new(),
            isWebappPathSet: false,
        };
        this.isWebappPathSet = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isTestingMode(&mut self) -> bool {
        return jcall!(InitInfo::getInstance()).isTesting();
    }
    

    pub fn getWebappPath(&mut self) -> String {
        return self.path.clone();
    }
    

    // synchronized
    pub fn setWebappPath(&mut self, mut path: String) {
        path = path.clone();
        self.isWebappPathSet = true;
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return jcall!(InitInfo::getInstance()).getTestHtmlPath();
    }
    

    // throws Exception
    pub fn getMainPath(&mut self) -> String {
        let mut mainPath: String = jcall!(InitInfo::getInstance()).getMainPath();
        if !jcall!(StringValidationUtil::getInstance()).isEmpty(mainPath.clone()) {
            return mainPath.clone();
        } else if self.isWebappPathSet {
            return self.getWebappPath();
        } else {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Webapp Path is not set"))));
        }
    }
}
impl std::ops::Deref for WebAppUrlGlobals {
    type Target = UrlGlobalsInterface;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for WebAppUrlGlobals {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterfaceDyn for WebAppUrlGlobals {
    fn as_UrlGlobalsInterface(&self) -> &crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterface { &self.base }
    fn as_UrlGlobalsInterface_mut(&mut self) -> &mut crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterface { &mut self.base }
    fn getMainPath(&mut self) -> String { Self::getMainPath(self) }
    fn getTestHtmlPath(&mut self) -> String { Self::getTestHtmlPath(self) }
    fn getWebappPath(&mut self) -> String { Self::getWebappPath(self) }
    fn isTestingMode(&mut self) -> bool { Self::isTestingMode(self) }
}


