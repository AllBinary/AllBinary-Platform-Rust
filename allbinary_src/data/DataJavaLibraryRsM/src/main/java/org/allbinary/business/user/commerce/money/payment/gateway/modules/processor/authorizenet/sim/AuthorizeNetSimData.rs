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
// package org.allbinary.business.user.commerce.money.payment.gateway.modules.processor.authorizenet.sim
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

pub struct AuthorizeNetSimData {
}

thread_local! { pub static AuthorizeNetSimData_X_FP_HASH: String = String::from("x_fp_hash"); }
thread_local! { pub static AuthorizeNetSimData_X_FP_SEQUENCE: String = String::from("x_fp_sequence"); }
thread_local! { pub static AuthorizeNetSimData_X_FP_TIMESTAMP: String = String::from("x_fp_timestamp"); }
thread_local! { pub static AuthorizeNetSimData_X_SHOW_FORM: String = String::from("x_show_form"); }
thread_local! { pub static AuthorizeNetSimData_X_HEADER_HTML_PAYMENT_FORM: String = String::from("x_header_html_payment_form"); }
thread_local! { pub static AuthorizeNetSimData_X_FOOTER_HTML_PAYMENT_FORM: String = String::from("x_footer_html_payment_form"); }
thread_local! { pub static AuthorizeNetSimData_X_HEADER_HTML_RECEIPT: String = String::from("x_header_html_receipt"); }
thread_local! { pub static AuthorizeNetSimData_X_FOOTER_HTML_RECEIPT: String = String::from("x_footer_html_receipt"); }
thread_local! { pub static AuthorizeNetSimData_X_RECEIPT_LINK_TEXT: String = String::from("x_receipt_link_text"); }
thread_local! { pub static AuthorizeNetSimData_X_RECEIPT_LINK_URL: String = String::from("x_receipt_link_url"); }
thread_local! { pub static AuthorizeNetSimData_X_LOGO_URL: String = String::from("x_logo_url"); }
thread_local! { pub static AuthorizeNetSimData_X_BACKGROUND_URL: String = String::from("x_background_url"); }
thread_local! { pub static AuthorizeNetSimData_X_COLOR_BACKGROUND: String = String::from("x_color_background"); }
thread_local! { pub static AuthorizeNetSimData_X_COLOR_LINK: String = String::from("x_color_link"); }
thread_local! { pub static AuthorizeNetSimData_X_COLOR_TEXT: String = String::from("x_color_text"); }
thread_local! { pub static AuthorizeNetSimData_X_RELAY_URL: String = String::from("x_relay_url"); }

impl AuthorizeNetSimData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

