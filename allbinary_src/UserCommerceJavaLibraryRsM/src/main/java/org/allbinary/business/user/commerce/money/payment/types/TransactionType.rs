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

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct TransactionType {
    pub name: String,
}


impl TransactionType {
    

    pub fn new_struct(mut name: String) -> Self {
        let mut this = Self {
            name: String::new(),
        };
        this.name = name.clone();
        this
    }

    pub fn new(mut name: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(name))))
    }
    

    pub fn isEqual(&mut self, mut transactionType: Option<Rc<RefCell<TransactionType>>>) -> bool {
        if (self.toString().cmp(&jcall!(transactionType).toString()) as i32) == 0 {
            return true;
        } else {
            return false;
        }
    }
    

    pub fn toString(&mut self) -> String {
        return self.name.clone();
    }
}

