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
use crate::tags::TagStrings::TagStrings;
use super::ModifyTableInterface::ModifyTableInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct ModifyTable {
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub tagStrings: Option<Rc<RefCell<TagStrings>>>,
}

impl ModifyTableInterface for ModifyTable {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}

impl ModifyTable {

    pub fn new_struct() -> Self {
        let mut this = Self {
            commonStrings: None,
            tagStrings: None,
        };
        this.commonStrings = CommonStrings::getInstance();
        this.tagStrings = TagStrings::getInstance();
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
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
pub trait ModifyTableDyn: crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface {
    fn as_ModifyTable(&self) -> &ModifyTable;
    fn as_ModifyTable_mut(&mut self) -> &mut ModifyTable;
}
impl ModifyTableDyn for ModifyTable {
    fn as_ModifyTable(&self) -> &ModifyTable { self }
    fn as_ModifyTable_mut(&mut self) -> &mut ModifyTable { self }
}


