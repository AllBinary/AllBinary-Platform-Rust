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
// package org.allbinary.business.user.commerce.money.payment.types
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::TransactionType::TransactionType;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransactionTypeFactory {
    pub SALE: Option<Rc<RefCell<TransactionType>>>,
    pub CREDIT: Option<Rc<RefCell<TransactionType>>>,
    pub AUTHORIZATION: Option<Rc<RefCell<TransactionType>>>,
    pub DELAYEDCAPTURE: Option<Rc<RefCell<TransactionType>>>,
    pub VOID: Option<Rc<RefCell<TransactionType>>>,
    pub INQUIRY: Option<Rc<RefCell<TransactionType>>>,
}

thread_local! { pub static TransactionTypeFactory_instance: Option<Rc<RefCell<TransactionTypeFactory>>> = TransactionTypeFactory::new(); }

impl TransactionTypeFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            SALE: None,
            CREDIT: None,
            AUTHORIZATION: None,
            DELAYEDCAPTURE: None,
            VOID: None,
            INQUIRY: None,
        };
        this.SALE = TransactionType::new(String::from("Sale"));
        this.CREDIT = TransactionType::new(String::from("Credit"));
        this.AUTHORIZATION = TransactionType::new(String::from("Authorization"));
        this.DELAYEDCAPTURE = TransactionType::new(String::from("Delayed Capture"));
        this.VOID = TransactionType::new(String::from("Void"));
        this.INQUIRY = TransactionType::new(String::from("Inquiry"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<TransactionTypeFactory>>> {
        return TransactionTypeFactory_instance.with(Clone::clone).clone();
    }
}

