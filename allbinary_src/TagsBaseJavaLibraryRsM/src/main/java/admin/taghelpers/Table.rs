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
use super::TableInterface::TableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Table {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

impl TableInterface for Table {
}
impl crate::admin::taghelpers::BasicTableInterface::BasicTableInterface for Table {
    fn drop(&mut self) -> String { Self::drop(self) }
    fn create(&mut self) -> String { Self::create(self) }
    fn restore(&mut self) -> String { Self::restore(self) }
    fn backup(&mut self) -> String { Self::backup(self) }
}
impl crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface for Table {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}

impl Table {

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
    

    // throws Exception
    pub fn insert(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn delete(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
    

    // throws Exception
    pub fn update(&mut self) -> String {
        panic!("{:?}", format!("{}: {:?}", "RuntimeException", ()));
    }
}
pub trait TableDyn {
    fn as_Table(&self) -> &Table;
    fn as_Table_mut(&mut self) -> &mut Table;
    fn drop(&mut self) -> String { Table::drop(self.as_Table_mut()) }
    fn create(&mut self) -> String { Table::create(self.as_Table_mut()) }
    fn restore(&mut self) -> String { Table::restore(self.as_Table_mut()) }
    fn backup(&mut self) -> String { Table::backup(self.as_Table_mut()) }
    fn insert(&mut self) -> String { Table::insert(self.as_Table_mut()) }
    fn delete(&mut self) -> String { Table::delete(self.as_Table_mut()) }
    fn update(&mut self) -> String { Table::update(self.as_Table_mut()) }
}
impl TableDyn for Table {
    fn as_Table(&self) -> &Table { self }
    fn as_Table_mut(&mut self) -> &mut Table { self }
}


