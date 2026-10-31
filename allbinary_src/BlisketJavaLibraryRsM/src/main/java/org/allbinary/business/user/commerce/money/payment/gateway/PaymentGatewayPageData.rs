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
// package org.allbinary.business.user.commerce.money.payment.gateway
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayPageData {
    pub MAKEPAYMENT: String,
    pub CHECKOUT: String,
    pub SHIPPING: String,
    pub SHIPPINGADDRESS: String,
    pub SHIPPINGADDRESSACTION: String,
    pub BILLINGADDRESS: String,
    pub PAYMENT: String,
    pub AUTHORIZE: String,
    pub FINISH: String,
}

thread_local! { pub static PaymentGatewayPageData_EXTENSION: String = String::from(".jsp"); }
thread_local! { pub static PaymentGatewayPageData_NAME: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); }
thread_local! { pub static PaymentGatewayPageData_FORWARDMAKEPAYMENT: String = String::from("MakePayment.jsp"); }
thread_local! { pub static PaymentGatewayPageData_PAYMENTOPTIONS: String = format!("{}{}", String::from("PaymentOptions"), PaymentGatewayPageData_EXTENSION.with(Clone::clone)); }
thread_local! { pub static PaymentGatewayPageData_STARTCHECKOUT: String = format!("{}{}", String::from("StartCheckout"), PaymentGatewayPageData_EXTENSION.with(Clone::clone)); }

impl PaymentGatewayPageData {
    

    pub fn new_struct(mut location: String, mut paymentMethod: String) -> Self {
        let mut this = Self {
            MAKEPAYMENT: String::new(),
            CHECKOUT: String::new(),
            SHIPPING: String::new(),
            SHIPPINGADDRESS: String::new(),
            SHIPPINGADDRESSACTION: String::new(),
            BILLINGADDRESS: String::new(),
            PAYMENT: String::new(),
            AUTHORIZE: String::new(),
            FINISH: String::new(),
        };
        let mut paymentMethodString: String = format!("{}{}", PaymentGatewayPageData_NAME.with(Clone::clone), paymentMethod);
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(location.clone());
        jcall!(stringBuffer).append(String::from("MakePayment"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.MAKEPAYMENT = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("Checkout"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.CHECKOUT = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("Shipping"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.SHIPPING = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("ShippingAddress"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.SHIPPINGADDRESS = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("ShippingAddressAction"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.SHIPPINGADDRESSACTION = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("BillingAddress"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.BILLINGADDRESS = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("Payment"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.PAYMENT = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("Authorize"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.AUTHORIZE = jcall!(stringBuffer).toString();
        jcall!(stringBuffer).delete(0, jcall!(stringBuffer).length());
        jcall!(stringBuffer).append(paymentMethodString.clone());
        jcall!(stringBuffer).append(String::from("PaymentFinish"));
        jcall!(stringBuffer).append(PaymentGatewayPageData_EXTENSION.with(Clone::clone).clone());
        this.FINISH = jcall!(stringBuffer).toString();
        this
    }

    pub fn new(mut location: String, mut paymentMethod: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(location, paymentMethod))))
    }
}

