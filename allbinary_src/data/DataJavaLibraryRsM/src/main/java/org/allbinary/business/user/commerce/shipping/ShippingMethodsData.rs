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
// package org.allbinary.business.user.commerce.shipping
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

pub struct ShippingMethodsData {
}

pub static ShippingMethodsData_SHIPPINGMETHODSORDERSUMMARIES: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHODS_ORDERSUMMARIES"));
pub static ShippingMethodsData_SHIPPINGMETHODORDERSUMMARY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_ORDERSUMMARY"));
pub static ShippingMethodsData_SHIPPINGMETHODS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHODS"));
pub static ShippingMethodsData_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHODS_NAME"));
pub static ShippingMethodsData_SHIPPINGCOST: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_SHIPPINGCOST"));
pub static ShippingMethodsData_TAXRATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_TAXRATE"));
pub static ShippingMethodsData_SUBTOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_SUBTOTAL"));
pub static ShippingMethodsData_TAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_TAX"));
pub static ShippingMethodsData_TOTAL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("SHIPPINGMETHOD_TOTAL"));

impl ShippingMethodsData {

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

