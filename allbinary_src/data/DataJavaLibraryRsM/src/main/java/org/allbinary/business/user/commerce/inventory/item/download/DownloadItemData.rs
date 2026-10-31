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
// package org.allbinary.business.user.commerce.inventory.item.download
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

pub struct DownloadItemData {
}

thread_local! { pub static DownloadItemData_NAME: String = String::from("DOWNLOADITEM_NAME"); }
thread_local! { pub static DownloadItemData_ID: String = String::from("DOWNLOADITEM_ID"); }
thread_local! { pub static DownloadItemData_SYSTEM: String = String::from("DOWNLOADITEM_SYSTEM"); }
thread_local! { pub static DownloadItemData_PLATFORM: String = String::from("DOWNLOADITEM_PLATFORM"); }
thread_local! { pub static DownloadItemData_SPECIAL_NAME: String = String::from("DOWNLOADITEM_SPECIAL_NAME"); }
thread_local! { pub static DownloadItemData_VERSION: String = String::from("DOWNLOADITEM_VERSION"); }
thread_local! { pub static DownloadItemData_CHANGES: String = String::from("DOWNLOADITEM_CHANGES"); }
thread_local! { pub static DownloadItemData_LICENSE_FILE: String = String::from("DOWNLOADITEM_LICENSE_FILE"); }
thread_local! { pub static DownloadItemData_FILE: String = String::from("DOWNLOADITEM_FILE"); }
thread_local! { pub static DownloadItemData_EXTRA_FILES: String = String::from("DOWNLOADITEM_EXTRA_FILES"); }
thread_local! { pub static DownloadItemData_SIZE: String = String::from("DOWNLOADITEM_SIZE"); }
thread_local! { pub static DownloadItemData_VALID_TIME: String = String::from("DOWNLOADITEM_VALID_TIME"); }
thread_local! { pub static DownloadItemData_VALID_TIME_YEARS: String = String::from("DOWNLOADITEM_VALID_TIME_YEAR"); }
thread_local! { pub static DownloadItemData_VALID_TIME_MONTHS: String = String::from("DOWNLOADITEM_VALID_TIME_MONTH"); }
thread_local! { pub static DownloadItemData_VALID_TIME_DAYS: String = String::from("DOWNLOADITEM_VALID_TIME_DAY"); }
thread_local! { pub static DownloadItemData_VALID_TIME_HOURS: String = String::from("DOWNLOADITEM_VALID_TIME_HOUR"); }
thread_local! { pub static DownloadItemData_VALID_TIME_MINUTES: String = String::from("DOWNLOADITEM_VALID_TIME_MINUTES"); }
thread_local! { pub static DownloadItemData_VALID_TIME_SECONDS: String = String::from("DOWNLOADITEM_VALID_TIME_SECONDS"); }
thread_local! { pub static DownloadItemData_RETRIES: String = String::from("DOWNLOADITEM_RETRIES"); }

impl DownloadItemData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

