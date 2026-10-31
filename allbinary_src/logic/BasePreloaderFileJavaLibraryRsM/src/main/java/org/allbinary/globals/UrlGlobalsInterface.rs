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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UrlGlobalsInterface {
}


impl UrlGlobalsInterface {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn getMainPath(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn getTestHtmlPath(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn getWebappPath(&mut self) -> String {
        return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
    }
    

    pub fn isTestingMode(&mut self) -> bool {
        return false;
    }
}
pub trait UrlGlobalsInterfaceDyn {
    fn as_UrlGlobalsInterface(&self) -> &UrlGlobalsInterface;
    fn as_UrlGlobalsInterface_mut(&mut self) -> &mut UrlGlobalsInterface;
    fn getMainPath(&mut self) -> String { UrlGlobalsInterface::getMainPath(self.as_UrlGlobalsInterface_mut()) }
    fn getTestHtmlPath(&mut self) -> String { UrlGlobalsInterface::getTestHtmlPath(self.as_UrlGlobalsInterface_mut()) }
    fn getWebappPath(&mut self) -> String { UrlGlobalsInterface::getWebappPath(self.as_UrlGlobalsInterface_mut()) }
    fn isTestingMode(&mut self) -> bool { UrlGlobalsInterface::isTestingMode(self.as_UrlGlobalsInterface_mut()) }
}
impl UrlGlobalsInterfaceDyn for UrlGlobalsInterface {
    fn as_UrlGlobalsInterface(&self) -> &UrlGlobalsInterface { self }
    fn as_UrlGlobalsInterface_mut(&mut self) -> &mut UrlGlobalsInterface { self }
}


