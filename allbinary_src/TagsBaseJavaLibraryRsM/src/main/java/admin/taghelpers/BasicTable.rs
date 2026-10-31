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
// package admin.taghelpers
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use super::BasicTableInterface::BasicTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicTable {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl BasicTableInterface for BasicTable {
    fn drop(&mut self) -> String { Self::drop(self) }
    fn create(&mut self) -> String { Self::create(self) }
    fn restore(&mut self) -> String { Self::restore(self) }
    fn backup(&mut self) -> String { Self::backup(self) }
}

impl BasicTable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    // throws Exception
    pub fn drop(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn create(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn restore(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn backup(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait BasicTableDyn: crate::admin::taghelpers::BasicTableInterface::BasicTableInterface {
    fn as_BasicTable(&self) -> &BasicTable;
    fn as_BasicTable_mut(&mut self) -> &mut BasicTable;
}
impl BasicTableDyn for BasicTable {
    fn as_BasicTable(&self) -> &BasicTable { self }
    fn as_BasicTable_mut(&mut self) -> &mut BasicTable { self }
}


