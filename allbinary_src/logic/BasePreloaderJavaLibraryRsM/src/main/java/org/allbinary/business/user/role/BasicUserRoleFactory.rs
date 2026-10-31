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
use super::BasicUserRole::BasicUserRole;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicUserRoleFactory {
    pub INACTIVITYTIMEOUT: i64,
    pub CUSTOMERTIMEOUT: i64,
    pub CUSTOMERMAXSESSIONTIME: i64,
    pub MAXSESSIONTIME: i64,
    pub INSTALLER: Option<Rc<RefCell<BasicUserRole>>>,
    pub ADMINISTRATOR: Option<Rc<RefCell<BasicUserRole>>>,
    pub CUSTOMER: Option<Rc<RefCell<BasicUserRole>>>,
    pub SUBSCRIBERCUSTOMER: Option<Rc<RefCell<BasicUserRole>>>,
    pub WHOLESALECUSTOMER: Option<Rc<RefCell<BasicUserRole>>>,
    pub STOREMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub PRODUCTMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub CUSTOMERMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub PAYMENTMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub ORDERMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub WEBMANAGER: Option<Rc<RefCell<BasicUserRole>>>,
    pub REVIEWER: Option<Rc<RefCell<BasicUserRole>>>,
    pub SHIPPING: Option<Rc<RefCell<BasicUserRole>>>,
    pub ADJUSTER: Option<Rc<RefCell<BasicUserRole>>>,
    pub WORKFLOWEDITOR: Option<Rc<RefCell<BasicUserRole>>>,
    pub WIZARD: Option<Rc<RefCell<BasicUserRole>>>,
    pub VIEWEDITOR: Option<Rc<RefCell<BasicUserRole>>>,
}

thread_local! { pub static BasicUserRoleFactory_instance: Option<Rc<RefCell<BasicUserRoleFactory>>> = BasicUserRoleFactory::new(); }

impl BasicUserRoleFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            INACTIVITYTIMEOUT: 0_i64,
            CUSTOMERTIMEOUT: 0_i64,
            CUSTOMERMAXSESSIONTIME: 0_i64,
            MAXSESSIONTIME: 0_i64,
            INSTALLER: None,
            ADMINISTRATOR: None,
            CUSTOMER: None,
            SUBSCRIBERCUSTOMER: None,
            WHOLESALECUSTOMER: None,
            STOREMANAGER: None,
            PRODUCTMANAGER: None,
            CUSTOMERMANAGER: None,
            PAYMENTMANAGER: None,
            ORDERMANAGER: None,
            WEBMANAGER: None,
            REVIEWER: None,
            SHIPPING: None,
            ADJUSTER: None,
            WORKFLOWEDITOR: None,
            WIZARD: None,
            VIEWEDITOR: None,
        };
        this.INACTIVITYTIMEOUT = 360000000_i64;
        this.CUSTOMERTIMEOUT = 360000000_i64;
        this.CUSTOMERMAXSESSIONTIME = 360000000_i64;
        this.MAXSESSIONTIME = 360000000_i64;
        this.INSTALLER = BasicUserRole::new4(String::from("Installer"), 16, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.ADMINISTRATOR = BasicUserRole::new4(String::from("Administrator"), 0, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.CUSTOMER = BasicUserRole::new5(String::from("CustomerUser"), String::from("Customer User"), 1, this.CUSTOMERTIMEOUT.clone(), this.CUSTOMERMAXSESSIONTIME.clone());
        this.SUBSCRIBERCUSTOMER = BasicUserRole::new5(String::from("SubscriberUser"), String::from("Subscriber User"), 2, this.CUSTOMERTIMEOUT.clone(), this.CUSTOMERMAXSESSIONTIME.clone());
        this.WHOLESALECUSTOMER = BasicUserRole::new5(String::from("WholesaleUser"), String::from("Wholesale User"), 3, this.CUSTOMERTIMEOUT.clone(), this.CUSTOMERMAXSESSIONTIME.clone());
        this.STOREMANAGER = BasicUserRole::new5(String::from("StoreManager"), String::from("Store Manager"), 4, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.PRODUCTMANAGER = BasicUserRole::new5(String::from("ProductManager"), String::from("Product Manager"), 5, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.CUSTOMERMANAGER = BasicUserRole::new5(String::from("UserManager"), String::from("User Manager"), 6, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.PAYMENTMANAGER = BasicUserRole::new5(String::from("PaymentManager"), String::from("Payment Manager"), 7, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.ORDERMANAGER = BasicUserRole::new5(String::from("OrderManager"), String::from("Order Manager"), 8, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.WEBMANAGER = BasicUserRole::new5(String::from("WebManager"), String::from("Web Manager"), 9, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.REVIEWER = BasicUserRole::new4(String::from("Reviewer"), 10, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.SHIPPING = BasicUserRole::new4(String::from("Shipping"), 11, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.ADJUSTER = BasicUserRole::new4(String::from("Adjuster"), 12, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.WORKFLOWEDITOR = BasicUserRole::new5(String::from("WorkFlowEditor"), String::from("WorkFlow Editor"), 13, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.WIZARD = BasicUserRole::new5(String::from("WizardUser"), String::from("Wizard User"), 14, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this.VIEWEDITOR = BasicUserRole::new5(String::from("ViewEditor"), String::from("View Editor"), 15, this.MAXSESSIONTIME.clone(), this.INACTIVITYTIMEOUT.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicUserRoleFactory>>> {
        return BasicUserRoleFactory_instance.with(Clone::clone).clone();
    }
}

