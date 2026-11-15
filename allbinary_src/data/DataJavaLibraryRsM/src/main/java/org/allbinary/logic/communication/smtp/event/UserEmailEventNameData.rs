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
use super::*;
use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::java::lang::Exception::*;

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

pub static mut UserEmailEventNameData_userNameEvenNameHashMap: std::cell::LazyCell<Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>> = std::cell::LazyCell::new(|| jcall!(StdUtil::getInstance()).createHashMap());
pub static mut UserEmailEventNameData_nextId: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| 0);
pub static UserEmailEventNameData_INSTALLER: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Installer")));
pub static UserEmailEventNameData_EXCEPTION: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Exception")));
pub static UserEmailEventNameData_LOGGEDEXCEPTION: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Logged Exception")));
pub static UserEmailEventNameData_ERROR: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Error")));
pub static UserEmailEventNameData_OUTOFMEMORY: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Out Of Memory Error")));
pub static UserEmailEventNameData_ADMINGENERATINGSTATICPAGES: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Admin Generating Static Pages")));
pub static UserEmailEventNameData_STOREERROROCCURRED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Error Occurred")));
pub static UserEmailEventNameData_STOREOUTOFMEMORY: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Out Of Memory Error")));
pub static UserEmailEventNameData_STORECREATED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Created")));
pub static UserEmailEventNameData_STOREOPENED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Opened")));
pub static UserEmailEventNameData_STORECLOSED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Closed")));
pub static UserEmailEventNameData_STOREGENERATINGSTATICPAGES: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Store Generating Static Pages")));
pub static UserEmailEventNameData_PRODUCTISOUT: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Product Is Out")));
pub static UserEmailEventNameData_PRODUCTISAVAILABLE: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Product Is Available")));
pub static UserEmailEventNameData_PRODUCTISOLD: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Product Is Old")));
pub static UserEmailEventNameData_PRODUCTISONSALE: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Product Is On Sale")));
pub static UserEmailEventNameData_PRODUCTSALEISOVER: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Product Sale Is Over")));
pub static UserEmailEventNameData_ORDERPREPROCESSING: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()));
pub static UserEmailEventNameData_ORDERPROCESSING: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PROCESSING.with(Clone::clone).clone()));
pub static UserEmailEventNameData_ORDERCANCELLED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()));
pub static UserEmailEventNameData_ORDERPARTIALLYSHIPPED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PARTIALLYSHIPPED.with(Clone::clone).clone()));
pub static UserEmailEventNameData_ORDERSHIPPED: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()));
pub static UserEmailEventNameData_QUOTEREQUEST: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("Quote Request")));
pub static UserEmailEventNameData_NEWPASSWORD: crate::JavaStatic<Option<Rc<RefCell<UserEmailEventNameData>>>> = crate::JavaStatic::new(|| UserEmailEventNameData::new(String::from("New Password")));

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
        let this = Some(Rc::new(RefCell::new(Self::new_struct(eventHandlerName.clone()))));
            jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).put(crate::java_into_object(jcall!(this).eventHandlerName.clone()), (|java_value: Option<Rc<RefCell<_>>>| java_value.map(|reference| Rc::new(RefCell::new(Box::new(Some(reference)) as Box<dyn std::any::Any>))))(this.clone()));
        this
    }
    

    // synchronized
    pub fn initNextId(&mut self) {
        self.eventNameId = { let java_value = (unsafe { UserEmailEventNameData_nextId.clone() }).clone(); java_value }.clone();
        unsafe { *UserEmailEventNameData_nextId += 1 };
    }
    

    pub fn isEvent(&mut self, mut userEmailEventNameString: String) -> bool {
        return jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).containsKey(crate::java_into_object(userEmailEventNameString.clone()));
    }
    

    // throws Exception
    // synchronized
    pub fn getInstance(mut userEmailEventNameString: String) -> Option<Rc<RefCell<UserEmailEventNameData>>> {
        let mut userEmailEventNameData: Option<Rc<RefCell<UserEmailEventNameData>>> = { let java_object = (jcall!(unsafe { UserEmailEventNameData_userNameEvenNameHashMap.clone() }).get(crate::java_into_object(userEmailEventNameString.clone()))).clone(); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<UserEmailEventNameData>>>>().expect("ClassCastException").clone()) };
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

