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
// package org.allbinary.logic.system.security.licensing
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystem;
use crate::org::allbinary::util::ABHashtable::ABHashtable;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait AbeClientInformationInterface {
    

    fn init(&mut self);
    

    fn getOperatingSystemInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::system::os::GenericOperatingSystem::GenericOperatingSystemDyn>>>;
    

    fn getName(&mut self) -> String;
    

    fn getSpecialName(&mut self) -> String;
    

    fn getVersion(&mut self) -> String;
    

    fn getLicenseId(&mut self) -> String;
    

    fn getLicenseServers(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>;
    

    fn getLicenseServer(&mut self, index: i32) -> String;
    

    fn getNumberOfLicenseServers(&mut self) -> i32;
    

    fn toHashtable(&mut self) -> Option<Rc<RefCell<ABHashtable<Option<Rc<RefCell<Box<dyn std::any::Any>>>>, Option<Rc<RefCell<Box<dyn std::any::Any>>>>>>>>;
    

    fn isSameId(&mut self, alicenseId: String) -> bool;
    

    fn isLargerOrDifferentServerList(&mut self, vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) -> bool;
    

    fn isHardSale(&mut self) -> bool;
    

    fn toShortString(&mut self) -> String;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


