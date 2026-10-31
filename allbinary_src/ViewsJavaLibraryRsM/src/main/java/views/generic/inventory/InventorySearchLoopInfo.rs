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
// package views.generic.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventorySearchLoopInfo {
    pub lastPage: i32,
    pub currentPage: i32,
    pub numberOfResultsOnCurrentPage: i32,
    pub numberOfResults: i32,
    pub CURRENT: String,
    pub NUM: String,
    pub LAST: String,
}


impl InventorySearchLoopInfo {

    pub fn new_struct() -> Self {
        let mut this = Self {
            lastPage: 0,
            currentPage: 0,
            numberOfResultsOnCurrentPage: 0,
            numberOfResults: 0,
            CURRENT: String::new(),
            NUM: String::new(),
            LAST: String::new(),
        };
        this.lastPage = -1;
        this.currentPage = -1;
        this.numberOfResultsOnCurrentPage = 0;
        this.numberOfResults = 0;
        this.CURRENT = String::from("current: ");
        this.NUM = String::from(" num: ");
        this.LAST = String::from(" last: ");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn toString(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(self.CURRENT.clone());
        jcall!(stringBuffer).appendint(self.currentPage.clone());
        jcall!(stringBuffer).append(self.NUM.clone());
        jcall!(stringBuffer).appendint(self.numberOfResults.clone());
        jcall!(stringBuffer).append(self.LAST.clone());
        jcall!(stringBuffer).appendint(self.lastPage.clone());
        return jcall!(stringBuffer).toString();
    }
}

