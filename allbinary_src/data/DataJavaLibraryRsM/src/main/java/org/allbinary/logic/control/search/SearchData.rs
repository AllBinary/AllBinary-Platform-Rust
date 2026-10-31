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
// package org.allbinary.logic.control.search
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct SearchData {
}

thread_local! { pub static SearchData_TOTAL_NUMBER_PAGES: String = String::from("SEARCHDATA_TOTALNUMBEROFPAGES"); }
thread_local! { pub static SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE: String = String::from("SEARCHDATA_TOTALNUMBEROFITEMSONTHISPAGE"); }
thread_local! { pub static SearchData_TOTAL_NUMBER_ITEMS: String = String::from("SEARCHDATA_TOTALNUMBEROFITEMS"); }
thread_local! { pub static SearchData_DEFAULT_PAGE_LENGTH: i32 = 10; }
thread_local! { pub static SearchData_GENERATESTATICPAGES: String = String::from("Generate Static Pages"); }
thread_local! { pub static SearchData_SEARCHBYSUMMARY: String = String::from("Search By Summary"); }
thread_local! { pub static SearchData_SEARCHBYDESCRIPTION: String = String::from("Search By Description"); }
thread_local! { pub static SearchData_SEARCHBYCATEGORY: String = String::from("Search By Category"); }
thread_local! { pub static SearchData_SEARCHCATEGORIES: String = String::from("Search Categories"); }
thread_local! { pub static SearchData_FILENAME: String = String::from("SEARCHDATA_BASEFILENAME"); }
thread_local! { pub static SearchData_COLUMNNAME: String = String::from("COLUMN_NAME"); }
thread_local! { pub static SearchData_COLUMNVALUE: String = String::from("COLUMN_VALUE"); }
thread_local! { pub static SearchData_PARAMS: String = String::from("SEARCHDATA_PARAMS"); }
thread_local! { pub static SearchData_LENGTH: String = String::from("SEARCHDATA_LENGTH"); }
thread_local! { pub static SearchData_ORDER: String = String::from("SEARCHDATA_ORDER"); }
thread_local! { pub static SearchData_PAGE: String = String::from("SEARCHDATA_PAGE"); }
thread_local! { pub static SearchData_FIELDS: String = String::from("SEARCHDATA_FIELDS"); }
thread_local! { pub static SearchData_FIELD: String = String::from("SEARCHDATA_FIELD"); }
thread_local! { pub static SearchData_SORTBY: String = String::from("SEARCHDATA_SORTBY"); }
thread_local! { pub static SearchData_TYPE: String = String::from("SEARCHDATA_TYPE"); }
thread_local! { pub static SearchData_PAGE_INFO: String = String::from("pageInfo"); }
thread_local! { pub static SearchData_SEARCH: String = String::from("search"); }
thread_local! { pub static SearchData_MAKEPUBLIC: String = String::from("Make Public"); }

impl SearchData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

