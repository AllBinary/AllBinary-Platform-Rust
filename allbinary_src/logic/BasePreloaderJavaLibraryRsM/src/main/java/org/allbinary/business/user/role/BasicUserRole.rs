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
// package org.allbinary.business.user.role
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
// use crate::java::io::Serializable::Serializable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicUserRole {
    pub m_int_RoleId: i32,
    pub role: String,
    pub displayValue: String,
    pub sessionTimeout: i64,
    pub sessionInactivityTimeout: i64,
}

// impl Serializable for BasicUserRole with the methods of the trait

impl BasicUserRole {
    

    pub fn new_struct4(mut aRole: String, mut a_int_RoleId: i32, mut aSessionTimeout: i64, mut aSessionInactivityTimeout: i64) -> Self {
        let mut this = Self::new_struct5(aRole.clone(), aRole.clone(), a_int_RoleId.clone(), aSessionTimeout.clone(), aSessionInactivityTimeout.clone());
        // this(...) is in the struct literal above
        this
    }

    pub fn new4(mut aRole: String, mut a_int_RoleId: i32, mut aSessionTimeout: i64, mut aSessionInactivityTimeout: i64) -> Option<Rc<RefCell<Self>>> {
        Self::new5(aRole.clone(), aRole.clone(), a_int_RoleId.clone(), aSessionTimeout.clone(), aSessionInactivityTimeout.clone())
    }
    

    pub fn new_struct5(mut aRole: String, mut aDisplayValue: String, mut a_int_RoleId: i32, mut aSessionTimeout: i64, mut aSessionInactivityTimeout: i64) -> Self {
        let mut this = Self {
            m_int_RoleId: 0,
            role: String::new(),
            displayValue: String::new(),
            sessionTimeout: 0_i64,
            sessionInactivityTimeout: 0_i64,
        };
        this.m_int_RoleId = a_int_RoleId.clone();
        this.role = aRole.clone();
        this.displayValue = aDisplayValue.clone();
        this.sessionTimeout = aSessionTimeout.clone();
        this.sessionInactivityTimeout = aSessionInactivityTimeout.clone();
        this
    }

    pub fn new5(mut aRole: String, mut aDisplayValue: String, mut a_int_RoleId: i32, mut aSessionTimeout: i64, mut aSessionInactivityTimeout: i64) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct5(aRole.clone(), aDisplayValue.clone(), a_int_RoleId.clone(), aSessionTimeout.clone(), aSessionInactivityTimeout.clone()))));
            jcall!({ let java_field = jcall!(BasicUserRoles::BasicUserRoles::getInstance()).roleVector.clone(); java_field }).add((|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    pub fn getRoleId(&mut self) -> i32 {
        return self.m_int_RoleId.clone();
    }
    

    pub fn getRole(&mut self) -> String {
        return self.role.clone();
    }
    

    pub fn getSessionTimeout(&mut self) -> i64 {
        return self.sessionTimeout.clone();
    }
    

    pub fn getSessionInactivityTimeout(&mut self) -> i64 {
        return self.sessionInactivityTimeout.clone();
    }
    

    pub fn equals(&mut self, mut basicUserRole: Option<Rc<RefCell<BasicUserRole>>>) -> bool {
        if self.getRoleId() == { let java_value = jcall!(basicUserRole).getRoleId(); java_value } {
            return true;
        }
        return false;
    }
    

    pub fn toString(&mut self) -> String {
        return self.getRole();
    }
}

