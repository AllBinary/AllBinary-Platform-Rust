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
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::UrlGlobalsInterface::UrlGlobalsInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

//implements
pub struct AppUrlGlobals {
    pub base: UrlGlobalsInterface,
    pub path: String,
}

thread_local! { pub static AppUrlGlobals_NULL_APP_URL_GLOBALS: Option<Rc<RefCell<AppUrlGlobals>>> = AppUrlGlobals::new(); }

impl AppUrlGlobals {

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: UrlGlobalsInterface::new_struct(),
            path: String::new(),
        };
        this.path = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isTestingMode(&mut self) -> bool {
        return false;
    }
    

    pub fn getWebappPath(&mut self) -> String {
        return self.path.clone();
    }
    

    // synchronized
    pub fn setWebappPath(&mut self, mut path: String) {
        self.path = path.clone();
        if !(self.path.ends_with(&String::from("\\")) || self.path.ends_with(&String::from("/"))) {
            self.path = format!("{}{}", self.path, String::from("\\"));
        }
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    // throws Exception
    pub fn getMainPath(&mut self) -> String {
        return self.path.clone();
    }
}
impl std::ops::Deref for AppUrlGlobals {
    type Target = UrlGlobalsInterface;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AppUrlGlobals {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterfaceDyn for AppUrlGlobals {
    fn as_UrlGlobalsInterface(&self) -> &crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterface { &self.base }
    fn as_UrlGlobalsInterface_mut(&mut self) -> &mut crate::org::allbinary::globals::UrlGlobalsInterface::UrlGlobalsInterface { &mut self.base }
    fn getMainPath(&mut self) -> String { Self::getMainPath(self) }
    fn getTestHtmlPath(&mut self) -> String { Self::getTestHtmlPath(self) }
    fn getWebappPath(&mut self) -> String { Self::getWebappPath(self) }
    fn isTestingMode(&mut self) -> bool { Self::isTestingMode(self) }
}


