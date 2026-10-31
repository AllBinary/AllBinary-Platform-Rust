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
// package org.allbinary.data.tables.user.address
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait StreetAddressesEntityInterface: BasicDataTableInterface {
    

    fn remove(&mut self, index: Option<i32>);
    

    fn add(&mut self, address: Option<Rc<RefCell<StreetAddress>>>);
    

    fn update(&mut self, address: Option<Rc<RefCell<StreetAddress>>>);
    

    fn getLastId(&mut self) -> String;
    

    fn getDefault(&mut self) -> Option<Rc<RefCell<StreetAddress>>>;
    

    fn setDefault(&mut self, value: String);
}


