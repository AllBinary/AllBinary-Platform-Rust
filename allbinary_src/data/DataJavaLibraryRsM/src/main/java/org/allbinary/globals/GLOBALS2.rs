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

thread_local! { pub static GLOBALS2_GETCURRENTLOCATION: String = String::from("Get Current Location"); }
thread_local! { pub static GLOBALS2_GETCURRENTHOMELOCATION: String = String::from("Get Current Home Location"); }
thread_local! { pub static GLOBALS2_GENERATEALLSTORES: String = String::from("Generate All Stores"); }
thread_local! { pub static GLOBALS2_LICENSEAGREEMENT: String = String::from("License Agreement"); }
thread_local! { pub static GLOBALS2_CREATEDATABASES: String = String::from("Create Databases"); }
thread_local! { pub static GLOBALS2_VIEWNAME: String = String::from("VIEW_NAME"); }
thread_local! { pub static GLOBALS2_SETSHIPPINGTYPE: String = String::from("Set Shipping Type"); }
thread_local! { pub static GLOBALS2_VIEWSUMMARY: String = String::from("View Summary"); }
thread_local! { pub static GLOBALS2_SET: String = String::from("Set"); }
thread_local! { pub static GLOBALS2_SELECT: String = String::from("Select"); }
thread_local! { pub static GLOBALS2_SELECTNEW: String = String::from("Select New"); }
thread_local! { pub static GLOBALS2_SELECTEXISTING: String = String::from("Select Existing"); }
thread_local! { pub static GLOBALS2_PROCESS: String = String::from("Process"); }
thread_local! { pub static GLOBALS2_LOGINSUCCESS: String = String::from("Login Successful"); }
thread_local! { pub static GLOBALS2_LOGINFAILED: String = String::from("Login Failed"); }
thread_local! { pub static GLOBALS2_NONE: String = String::from("None"); }
thread_local! { pub static GLOBALS2_TOP: String = String::from("Top"); }
thread_local! { pub static GLOBALS2_LEFT: String = String::from("Left"); }
thread_local! { pub static GLOBALS2_RIGHT: String = String::from("Right"); }
thread_local! { pub static GLOBALS2_BACK: String = String::from("Back"); }
thread_local! { pub static GLOBALS2_BOTTOM: String = String::from("Bottom"); }
thread_local! { pub static GLOBALS2_FRONT: String = String::from("Front"); }
thread_local! { pub static GLOBALS2_AUTHORIZE: String = String::from("Authorize"); }
thread_local! { pub static GLOBALS2_AUTHORIZEANDEVALBODY: String = String::from("Authorize AND EVAL BODY"); }
thread_local! { pub static GLOBALS2_AUTHORIZEANDEVALBODYONERROR: String = String::from("Authorize AND EVAL BODY On Error"); }
thread_local! { pub static GLOBALS2_AUTHORIZEORDEREVALBODYONERROR: String = String::from("Authorize Order Eval On Error"); }
thread_local! { pub static GLOBALS2_AUTHORIZEFORMEVALBODYONERROR: String = String::from("Authorize Form Eval On Error"); }
thread_local! { pub static GLOBALS2_AUTHORIZEORDERANDSKIPBODY: String = String::from("Authorize Order Skip"); }
thread_local! { pub static GLOBALS2_AUTHORIZEORDERANDEVALBODY: String = String::from("Authorize Order Eval"); }
thread_local! { pub static GLOBALS2_EDITPRICING: String = String::from("Edit Pricing"); }
thread_local! { pub static GLOBALS2_UPDATEPRICING: String = String::from("Update Pricing"); }
thread_local! { pub static GLOBALS2_SETPAYMENTMETHOD: String = String::from("Set Payment Method"); }
thread_local! { pub static GLOBALS2_BACKUP: String = String::from("Backup"); }
thread_local! { pub static GLOBALS2_RESTORE: String = String::from("Restore"); }
thread_local! { pub static GLOBALS2_SUMMARY: String = String::from("Summary"); }
thread_local! { pub static GLOBALS2_DESCRIPTION: String = String::from("Description"); }
thread_local! { pub static GLOBALS2_IMG: String = String::from("img"); }
thread_local! { pub static GLOBALS2_HREF: String = String::from("href"); }
thread_local! { pub static GLOBALS2_KEYWORD: String = String::from("keyword"); }
thread_local! { pub static GLOBALS2_TYPE: String = String::from("TYPE"); }
thread_local! { pub static GLOBALS2_DATA: String = String::from("DATA"); }
thread_local! { pub static GLOBALS2_NUM: String = String::from("NUM"); }
thread_local! { pub static GLOBALS2_PRICE: String = String::from("PRICE"); }
thread_local! { pub static GLOBALS2_REGISTER: String = String::from("Register"); }
thread_local! { pub static GLOBALS2_CHECKOUT: String = String::from("Checkout"); }
thread_local! { pub static GLOBALS2_ADMINCOMMAND: String = String::from("AdminCommand"); }
thread_local! { pub static GLOBALS2_PROCESSBODYIFNOTAUTHENTICATED: String = String::from("Not Authenticated"); }
thread_local! { pub static GLOBALS2_PROCESSBODYIFAUTHENTICATED: String = String::from("Authenticated"); }
thread_local! { pub static GLOBALS2_PROCESSTRANSACTION: String = String::from("Process Transaction"); }
thread_local! { pub static GLOBALS2_NEWPASSWORD: String = String::from("New Password"); }
thread_local! { pub static GLOBALS2_CHANGEPASSWORD: String = String::from("Change Password"); }
thread_local! { pub static GLOBALS2_JDBCDRIVER: String = String::from("Jdbc Driver"); }
thread_local! { pub static GLOBALS2_WEBAPPPATH: String = String::from("Webapp Path"); }
thread_local! { pub static GLOBALS2_AMOUNTVALUE: String = String::from("Amount Value"); }
thread_local! { pub static GLOBALS2_GETORDERIDFROMSESSION: String = String::from("Get Order Id From Session"); }
thread_local! { pub static GLOBALS2_SETUPPAYMENTMETHODS: String = String::from("Setup Payment Methods"); }
thread_local! { pub static GLOBALS2_CANCELSTOREFRONT: String = String::from("Cancel Store Front"); }
thread_local! { pub static GLOBALS2_ADJUST: String = String::from("Adjust"); }
thread_local! { pub static GLOBALS2_ISASUBSET: String = String::from("Is a Subset"); }
thread_local! { pub static GLOBALS2_NOTASUBSET: String = String::from("Not a Subset"); }
thread_local! { pub static GLOBALS2_GENERATE: String = String::from("Generate"); }
thread_local! { pub static GLOBALS2_VIEW: String = String::from("View"); }
thread_local! { pub static GLOBALS2_EDIT: String = String::from("Edit"); }
thread_local! { pub static GLOBALS2_NEW: String = String::from("New"); }
thread_local! { pub static GLOBALS2_SEARCH: String = String::from("Search"); }

impl GLOBALS2 {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

