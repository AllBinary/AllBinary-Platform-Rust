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
// package org.allbinary.business.user.commerce.inventory.basket
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

pub struct BasketData {
}

pub static BasketData_BASKET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET"));
pub static BasketData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("NAME"));
pub static BasketData_BASKETREVIEW: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_REVIEW"));
pub static BasketData_TOTALWEIGHT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_TOTALWEIGTH"));
pub static BasketData_ITEMTOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_ITEMTOTAL"));
pub static BasketData_ITEMTOTALINBASKET: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_ITEMTOTALINBASKET"));
pub static BasketData_SUBTOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_SUBTOTAL"));
pub static BasketData_ISEMPTY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_ISEMPTY"));
pub static BasketData_INSERT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_INSERT"));
pub static BasketData_DELETE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_DELETE"));
pub static BasketData_ADJUST: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("BASKET_ADJUST"));

impl BasketData {

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

