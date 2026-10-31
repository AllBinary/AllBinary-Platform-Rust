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
// package org.allbinary.logic.communication.smtp.event
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::logic::StdUtil::StdUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UserEmailEventNameData {
    pub eventNameId: i32,
    pub eventHandlerName: String,
}

pub static mut UserEmailEventNameData_userNameEvenNameHashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());
pub static mut UserEmailEventNameData_nextId: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
thread_local! { pub static UserEmailEventNameData_INSTALLER: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Installer")); }
thread_local! { pub static UserEmailEventNameData_EXCEPTION: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Exception")); }
thread_local! { pub static UserEmailEventNameData_LOGGEDEXCEPTION: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Logged Exception")); }
thread_local! { pub static UserEmailEventNameData_ERROR: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Error")); }
thread_local! { pub static UserEmailEventNameData_OUTOFMEMORY: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Out Of Memory Error")); }
thread_local! { pub static UserEmailEventNameData_ADMINGENERATINGSTATICPAGES: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Admin Generating Static Pages")); }
thread_local! { pub static UserEmailEventNameData_STOREERROROCCURRED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Error Occurred")); }
thread_local! { pub static UserEmailEventNameData_STOREOUTOFMEMORY: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Out Of Memory Error")); }
thread_local! { pub static UserEmailEventNameData_STORECREATED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Created")); }
thread_local! { pub static UserEmailEventNameData_STOREOPENED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Opened")); }
thread_local! { pub static UserEmailEventNameData_STORECLOSED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Closed")); }
thread_local! { pub static UserEmailEventNameData_STOREGENERATINGSTATICPAGES: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Store Generating Static Pages")); }
thread_local! { pub static UserEmailEventNameData_PRODUCTISOUT: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Product Is Out")); }
thread_local! { pub static UserEmailEventNameData_PRODUCTISAVAILABLE: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Product Is Available")); }
thread_local! { pub static UserEmailEventNameData_PRODUCTISOLD: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Product Is Old")); }
thread_local! { pub static UserEmailEventNameData_PRODUCTISONSALE: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Product Is On Sale")); }
thread_local! { pub static UserEmailEventNameData_PRODUCTSALEISOVER: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Product Sale Is Over")); }
thread_local! { pub static UserEmailEventNameData_ORDERPREPROCESSING: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()); }
thread_local! { pub static UserEmailEventNameData_ORDERPROCESSING: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone()); }
thread_local! { pub static UserEmailEventNameData_ORDERCANCELLED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()); }
thread_local! { pub static UserEmailEventNameData_ORDERPARTIALLYSHIPPED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone()); }
thread_local! { pub static UserEmailEventNameData_ORDERSHIPPED: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()); }
thread_local! { pub static UserEmailEventNameData_QUOTEREQUEST: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("Quote Request")); }
thread_local! { pub static UserEmailEventNameData_NEWPASSWORD: Option<Rc<RefCell<UserEmailEventNameData>>> = UserEmailEventNameData::new(String::from("New Password")); }

impl UserEmailEventNameData {
    

    pub fn new_struct(mut eventHandlerName: String) -> Self {
        let mut this = Self {
            eventNameId: 0,
            eventHandlerName: String::new(),
        };
        this.eventHandlerName = eventHandlerName.clone();
        this.initNextId();
        this
    }

    pub fn new(mut eventHandlerName: String) -> Option<Rc<RefCell<Self>>> {
        let this = Some(Rc::new(RefCell::new(Self::new_struct(eventHandlerName))));
            jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).put(this.eventHandlerName.clone(), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // synchronized
    pub fn initNextId(&mut self) {
        self.eventNameId = unsafe { UserEmailEventNameData_nextId.clone() }.clone();
        unsafe { *UserEmailEventNameData_nextId += 1 };
    }
    

    pub fn isEvent(&mut self, mut userEmailEventNameString: String) -> bool {
        return jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).containsKey(userEmailEventNameString.clone());
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(mut userEmailEventNameString: String) -> Option<Rc<RefCell<UserEmailEventNameData>>> {
        let mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>> = (jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).get(userEmailEventNameString.clone())).clone() /* as UserEmailEventNameData */;
        if userEmailEventNameData.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", ()));
        } else {
            return userEmailEventNameData.clone();
        }
    }
    

    pub fn toString(&mut self) -> String {
        return self.eventHandlerName.clone();
    }
}

