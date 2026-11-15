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

pub static DownloadItemData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_NAME"));
pub static DownloadItemData_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_ID"));
pub static DownloadItemData_SYSTEM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_SYSTEM"));
pub static DownloadItemData_PLATFORM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_PLATFORM"));
pub static DownloadItemData_SPECIAL_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_SPECIAL_NAME"));
pub static DownloadItemData_VERSION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VERSION"));
pub static DownloadItemData_CHANGES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_CHANGES"));
pub static DownloadItemData_LICENSE_FILE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_LICENSE_FILE"));
pub static DownloadItemData_FILE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_FILE"));
pub static DownloadItemData_EXTRA_FILES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_EXTRA_FILES"));
pub static DownloadItemData_SIZE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_SIZE"));
pub static DownloadItemData_VALID_TIME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME"));
pub static DownloadItemData_VALID_TIME_YEARS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_YEAR"));
pub static DownloadItemData_VALID_TIME_MONTHS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_MONTH"));
pub static DownloadItemData_VALID_TIME_DAYS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_DAY"));
pub static DownloadItemData_VALID_TIME_HOURS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_HOUR"));
pub static DownloadItemData_VALID_TIME_MINUTES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_MINUTES"));
pub static DownloadItemData_VALID_TIME_SECONDS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_VALID_TIME_SECONDS"));
pub static DownloadItemData_RETRIES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DOWNLOADITEM_RETRIES"));

impl DownloadItemData {

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

