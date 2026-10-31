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
use crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface;
use crate::org::allbinary::business::user::role::UserRole::UserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub trait WeblisketSessionInterface {
    

    fn clear(&mut self);
    

    fn getAttempts(&mut self) -> Option<i32>;
    

    fn getAuthentication(&mut self) -> String;
    

    fn getCreationTime(&mut self) -> i64;
    

    fn getId(&mut self) -> String;
    

    fn getLastAccessedTime(&mut self) -> i64;
    

    // throws Exception
    fn getOrder(&mut self) -> Option<Rc<RefCell<dyn OrderInterface>>>;
    

    fn getPassword(&mut self) -> String;
    

    fn getPaymentMethod(&mut self) -> String;
    

    // throws Exception
    fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>;
    

    fn getStoreName(&mut self) -> String;
    

    fn getTimeout(&mut self) -> String;
    

    fn getUserName(&mut self) -> String;
    

    fn getWebAppPath(&mut self) -> String;
    

    fn removeBasket(&mut self);
    

    fn setAttempts(&mut self, value: Option<i32>);
    

    fn setAuthenticated(&mut self);
    

    // Java overloaded method: Rust needs a different name
    fn setAuthenticated1(&mut self, value: bool);
    

    fn setPassword(&mut self, password: String);
    

    fn setPaymentMethod(&mut self, value: String);
    

    fn setRole(&mut self, aRole: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>);
    

    fn setStoreName(&mut self, value: String);
    

    fn setTimeout(&mut self, value: String);
    

    fn setUserName(&mut self, userName: String);

    fn toString(&mut self) -> String { String::from(std::any::type_name::<Self>()) }
}


