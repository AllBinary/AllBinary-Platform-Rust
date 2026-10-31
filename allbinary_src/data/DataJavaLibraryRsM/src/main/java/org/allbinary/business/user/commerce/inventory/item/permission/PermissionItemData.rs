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
// package org.allbinary.business.user.commerce.inventory.item.permission
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

pub struct PermissionItemData {
}

thread_local! { pub static PermissionItemData_WHO: String = String::from("PERMISSIONSITEM_WHO"); }
thread_local! { pub static PermissionItemData_WHAT: String = String::from("PERMISSIONSITEM_WHAT"); }
thread_local! { pub static PermissionItemData_TYPE: String = String::from("PERMISSIONSITEM_TYPE"); }
thread_local! { pub static PermissionItemData_START_TIME: String = String::from("PERMISSIONSITEM_START_TIME"); }
thread_local! { pub static PermissionItemData_END_TIME: String = String::from("PERMISSIONSITEM_END_TIME"); }
thread_local! { pub static PermissionItemData_REMOTE_ADDR: String = String::from("PERMISSIONSITEM_REMOTE_ADDR"); }

impl PermissionItemData {

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

