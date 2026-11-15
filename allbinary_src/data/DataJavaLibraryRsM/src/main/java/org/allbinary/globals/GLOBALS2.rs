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
// package org.allbinary.globals
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

pub struct GLOBALS2 {
}

pub static GLOBALS2_GETCURRENTLOCATION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Get Current Location"));
pub static GLOBALS2_GETCURRENTHOMELOCATION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Get Current Home Location"));
pub static GLOBALS2_GENERATEALLSTORES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Generate All Stores"));
pub static GLOBALS2_LICENSEAGREEMENT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("License Agreement"));
pub static GLOBALS2_CREATEDATABASES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Create Databases"));
pub static GLOBALS2_VIEWNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("VIEW_NAME"));
pub static GLOBALS2_SETSHIPPINGTYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set Shipping Type"));
pub static GLOBALS2_VIEWSUMMARY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("View Summary"));
pub static GLOBALS2_SET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set"));
pub static GLOBALS2_SELECT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Select"));
pub static GLOBALS2_SELECTNEW: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Select New"));
pub static GLOBALS2_SELECTEXISTING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Select Existing"));
pub static GLOBALS2_PROCESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Process"));
pub static GLOBALS2_LOGINSUCCESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Login Successful"));
pub static GLOBALS2_LOGINFAILED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Login Failed"));
pub static GLOBALS2_NONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("None"));
pub static GLOBALS2_TOP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Top"));
pub static GLOBALS2_LEFT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Left"));
pub static GLOBALS2_RIGHT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Right"));
pub static GLOBALS2_BACK: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Back"));
pub static GLOBALS2_BOTTOM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Bottom"));
pub static GLOBALS2_FRONT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Front"));
pub static GLOBALS2_AUTHORIZE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize"));
pub static GLOBALS2_AUTHORIZEANDEVALBODY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize AND EVAL BODY"));
pub static GLOBALS2_AUTHORIZEANDEVALBODYONERROR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize AND EVAL BODY On Error"));
pub static GLOBALS2_AUTHORIZEORDEREVALBODYONERROR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize Order Eval On Error"));
pub static GLOBALS2_AUTHORIZEFORMEVALBODYONERROR: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize Form Eval On Error"));
pub static GLOBALS2_AUTHORIZEORDERANDSKIPBODY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize Order Skip"));
pub static GLOBALS2_AUTHORIZEORDERANDEVALBODY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authorize Order Eval"));
pub static GLOBALS2_EDITPRICING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Edit Pricing"));
pub static GLOBALS2_UPDATEPRICING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Update Pricing"));
pub static GLOBALS2_SETPAYMENTMETHOD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Set Payment Method"));
pub static GLOBALS2_BACKUP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Backup"));
pub static GLOBALS2_RESTORE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Restore"));
pub static GLOBALS2_SUMMARY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Summary"));
pub static GLOBALS2_DESCRIPTION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Description"));
pub static GLOBALS2_IMG: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("img"));
pub static GLOBALS2_HREF: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("href"));
pub static GLOBALS2_KEYWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("keyword"));
pub static GLOBALS2_TYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("TYPE"));
pub static GLOBALS2_DATA: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("DATA"));
pub static GLOBALS2_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("NUM"));
pub static GLOBALS2_PRICE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("PRICE"));
pub static GLOBALS2_REGISTER: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Register"));
pub static GLOBALS2_CHECKOUT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Checkout"));
pub static GLOBALS2_ADMINCOMMAND: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("AdminCommand"));
pub static GLOBALS2_PROCESSBODYIFNOTAUTHENTICATED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Not Authenticated"));
pub static GLOBALS2_PROCESSBODYIFAUTHENTICATED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Authenticated"));
pub static GLOBALS2_PROCESSTRANSACTION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Process Transaction"));
pub static GLOBALS2_NEWPASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("New Password"));
pub static GLOBALS2_CHANGEPASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Change Password"));
pub static GLOBALS2_JDBCDRIVER: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Jdbc Driver"));
pub static GLOBALS2_WEBAPPPATH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Webapp Path"));
pub static GLOBALS2_AMOUNTVALUE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Amount Value"));
pub static GLOBALS2_GETORDERIDFROMSESSION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Get Order Id From Session"));
pub static GLOBALS2_SETUPPAYMENTMETHODS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Setup Payment Methods"));
pub static GLOBALS2_CANCELSTOREFRONT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Cancel Store Front"));
pub static GLOBALS2_ADJUST: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Adjust"));
pub static GLOBALS2_ISASUBSET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Is a Subset"));
pub static GLOBALS2_NOTASUBSET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Not a Subset"));
pub static GLOBALS2_GENERATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Generate"));
pub static GLOBALS2_VIEW: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("View"));
pub static GLOBALS2_EDIT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Edit"));
pub static GLOBALS2_NEW: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("New"));
pub static GLOBALS2_SEARCH: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("Search"));

impl GLOBALS2 {

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

