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
// package org.allbinary.logic.communication.http.request.session
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

pub struct WeblisketSessionData {
}

thread_local! { pub static WeblisketSessionData_INVALIDATESESSION: String = String::from("SESSION_INVALIDATE_SESSION"); }
thread_local! { pub static WeblisketSessionData_ATTEMPTS: String = String::from("SESSION_ATTEMPTS"); }
thread_local! { pub static WeblisketSessionData_TIMEOUT: String = String::from("SESSION_TIMEOUT"); }
thread_local! { pub static WeblisketSessionData_REMOVABLEUSERNAME: String = String::from("SESSION_REMOVABLE_USER_NAME"); }
thread_local! { pub static WeblisketSessionData_REMOVABLEPASSWORD: String = String::from("SESSION_REMOVABLE_PASSWORD"); }
thread_local! { pub static WeblisketSessionData_REMOVABLENEWPASSWORD: String = String::from("SESSION_REMOVABLE_NEW_PASSWORD"); }
thread_local! { pub static WeblisketSessionData_REMOVABLEREENTERNEWPASSWORD: String = String::from("SESSION_REMOVABLE_REENTER_NEW_PASSWORD"); }
thread_local! { pub static WeblisketSessionData_AUTHENTICATED: String = String::from("SESSION_AUTHENTICATED"); }

impl WeblisketSessionData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

