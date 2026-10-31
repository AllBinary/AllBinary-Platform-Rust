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
// package org.allbinary.business.init
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::debug::DebugFactory::DebugFactory;
use crate::org::allbinary::debug::NoDebug::NoDebug;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct LicenseInitInfo {
    pub id: String,
    pub serverVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>,
}


impl LicenseInitInfo {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            id: String::new(),
            serverVector: None,
        };
        this.id = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this.serverVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getLicenseId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn getServer(&mut self, mut index: i32) -> String {
        return (jcall!(self.serverVector).as_BasicArrayList_mut().objectArray[(index) as usize]).clone() /* as String */;
    }
    

    pub fn getNumberOfServers(&mut self) -> i32 {
        return jcall!(self.serverVector).size();
    }
    

    pub fn setLicenseId(&mut self, mut value: String) {
        self.id = value.clone();
    }
    

    pub fn clearServers(&mut self) {
        self.serverVector = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
    }
    

    pub fn setServer(&mut self, mut value: String, mut index: i32) {
        jcall!(self.serverVector).addAt(index.clone(), crate::java_into_object(value.clone()));
    }
    

    pub fn addServer(&mut self, mut value: String) {
        jcall!(self.serverVector).add(crate::java_into_object(value.clone()));
    }
    

    pub fn getServerList(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        return self.serverVector.clone();
    }
    

    pub fn setServerList(&mut self, mut servers: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        if match (&DebugFactory::getInstance(), &NoDebug::getInstance()) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
            self.serverVector = servers.clone();
        }
    }
}

