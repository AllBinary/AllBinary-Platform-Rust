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

pub static SearchData_TOTAL_NUMBER_PAGES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_TOTALNUMBEROFPAGES"));
pub static SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_TOTALNUMBEROFITEMSONTHISPAGE"));
pub static SearchData_TOTAL_NUMBER_ITEMS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_TOTALNUMBEROFITEMS"));
pub static SearchData_DEFAULT_PAGE_LENGTH: crate::JavaStatic<i32> = crate::JavaStatic::new(|| 10);
pub static SearchData_GENERATESTATICPAGES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Generate Static Pages"));
pub static SearchData_SEARCHBYSUMMARY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Search By Summary"));
pub static SearchData_SEARCHBYDESCRIPTION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Search By Description"));
pub static SearchData_SEARCHBYCATEGORY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Search By Category"));
pub static SearchData_SEARCHCATEGORIES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Search Categories"));
pub static SearchData_FILENAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_BASEFILENAME"));
pub static SearchData_COLUMNNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("COLUMN_NAME"));
pub static SearchData_COLUMNVALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("COLUMN_VALUE"));
pub static SearchData_PARAMS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_PARAMS"));
pub static SearchData_LENGTH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_LENGTH"));
pub static SearchData_ORDER: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_ORDER"));
pub static SearchData_PAGE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_PAGE"));
pub static SearchData_FIELDS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_FIELDS"));
pub static SearchData_FIELD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_FIELD"));
pub static SearchData_SORTBY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_SORTBY"));
pub static SearchData_TYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SEARCHDATA_TYPE"));
pub static SearchData_PAGE_INFO: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("pageInfo"));
pub static SearchData_SEARCH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("search"));
pub static SearchData_MAKEPUBLIC: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Make Public"));

impl SearchData {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

