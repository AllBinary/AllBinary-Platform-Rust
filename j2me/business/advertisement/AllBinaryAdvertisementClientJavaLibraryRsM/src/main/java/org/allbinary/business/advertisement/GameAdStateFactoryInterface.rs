// 
// AllBinary Open License Version 1 
// Copyright (c) 2025 AllBinary 
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
// package org.allbinary.business.advertisement
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::system::SoftwareInformation::SoftwareInformation;
use super::GameAdStateBase::GameAdStateBase;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait GameAdStateFactoryInterface {
    

    // throws Exception
    fn getInstanceForApp(&mut self, softwareInformation: Option<Rc<RefCell<SoftwareInformation>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::advertisement::GameAdStateBase::GameAdStateBaseDyn>>>;

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


