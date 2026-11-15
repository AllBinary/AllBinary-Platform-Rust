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

pub static WeblisketSessionData_INVALIDATESESSION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_INVALIDATE_SESSION"));
pub static WeblisketSessionData_ATTEMPTS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_ATTEMPTS"));
pub static WeblisketSessionData_TIMEOUT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_TIMEOUT"));
pub static WeblisketSessionData_REMOVABLEUSERNAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_REMOVABLE_USER_NAME"));
pub static WeblisketSessionData_REMOVABLEPASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_REMOVABLE_PASSWORD"));
pub static WeblisketSessionData_REMOVABLENEWPASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_REMOVABLE_NEW_PASSWORD"));
pub static WeblisketSessionData_REMOVABLEREENTERNEWPASSWORD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_REMOVABLE_REENTER_NEW_PASSWORD"));
pub static WeblisketSessionData_AUTHENTICATED: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SESSION_AUTHENTICATED"));

impl WeblisketSessionData {

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

