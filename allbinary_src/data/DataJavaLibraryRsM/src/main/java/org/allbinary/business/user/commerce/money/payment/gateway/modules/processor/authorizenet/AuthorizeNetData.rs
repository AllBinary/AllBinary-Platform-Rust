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
// package org.allbinary.business.user.commerce.money.payment.gateway.modules.processor.authorizenet
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

pub struct AuthorizeNetData {
}

pub static AuthorizeNetData_X_LOGIN: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_login"));
pub static AuthorizeNetData_X_TRAN_KEY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_tran_key"));
pub static AuthorizeNetData_X_VERSION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_version"));
pub static AuthorizeNetData_X_TEST_REQUEST: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_test_request"));
pub static AuthorizeNetData_X_RELAY_RESPONSE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_relay_response"));
pub static AuthorizeNetData_X_DUPLICATE_WINDOW: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_duplicate_window"));
pub static AuthorizeNetData_X_FIRST_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_first_name"));
pub static AuthorizeNetData_X_LAST_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_last_name"));
pub static AuthorizeNetData_X_COMPANY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_company"));
pub static AuthorizeNetData_X_ADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_address"));
pub static AuthorizeNetData_X_CITY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_city"));
pub static AuthorizeNetData_X_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_state"));
pub static AuthorizeNetData_X_ZIP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_zip"));
pub static AuthorizeNetData_X_COUNTRY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_country"));
pub static AuthorizeNetData_X_PHONE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_phone"));
pub static AuthorizeNetData_X_FAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_fax"));
pub static AuthorizeNetData_X_CUST_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_cust_id"));
pub static AuthorizeNetData_X_CUSTOMER_IP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_customer_ip"));
pub static AuthorizeNetData_X_CUSTOMER_TAX_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_customer_tax_id"));
pub static AuthorizeNetData_X_EMAIL_CUSTOMER: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_email_customer"));
pub static AuthorizeNetData_X_EMAIL: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_email"));
pub static AuthorizeNetData_X_EMAIL_MERCHANT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_email_merchant"));
pub static AuthorizeNetData_X_INVOICE_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_invoice_num"));
pub static AuthorizeNetData_X_DESCRIPTION: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_description"));
pub static AuthorizeNetData_X_SHIP_TO_FIRST_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_first_name"));
pub static AuthorizeNetData_X_SHIP_TO_LAST_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_last_name"));
pub static AuthorizeNetData_X_SHIP_TO_COMPANY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_company"));
pub static AuthorizeNetData_X_SHIP_TO_ADDRESS: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_address"));
pub static AuthorizeNetData_X_SHIP_TO_CITY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_city"));
pub static AuthorizeNetData_X_SHIP_TO_STATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_state"));
pub static AuthorizeNetData_X_SHIP_TO_ZIP: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_zip"));
pub static AuthorizeNetData_X_SHIP_TO_COUNTRY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_ship_to_country"));
pub static AuthorizeNetData_X_AMOUNT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_amount"));
pub static AuthorizeNetData_X_CURRENCY_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_currency_code"));
pub static AuthorizeNetData_X_METHOD: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_method"));
pub static AuthorizeNetData_X_TYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_type"));
pub static AuthorizeNetData_X_RECURRING_BILLING: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_recurring_billing"));
pub static AuthorizeNetData_X_BANK_ABA_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_bank_aba_code"));
pub static AuthorizeNetData_X_BANK_ACCT_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_bank_acct_num"));
pub static AuthorizeNetData_X_BANK_ACCT_TYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_bank_acct_type"));
pub static AuthorizeNetData_X_BANK_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_bank_name"));
pub static AuthorizeNetData_X_BANK_ACCT_NAME: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_bank_acct_name"));
pub static AuthorizeNetData_X_ECHECK_TYPE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_echeck_type"));
pub static AuthorizeNetData_X_CARD_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_card_num"));
pub static AuthorizeNetData_X_EXP_DATE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_exp_date"));
pub static AuthorizeNetData_X_CARD_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_card_code"));
pub static AuthorizeNetData_X_TRANS_ID: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_trans_id"));
pub static AuthorizeNetData_X_AUTH_CODE: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_auth_code"));
pub static AuthorizeNetData_X_PO_NUM: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_po_num"));
pub static AuthorizeNetData_X_TAX: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_tax"));
pub static AuthorizeNetData_X_TAX_EXEMPT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_tax_exempt"));
pub static AuthorizeNetData_X_FREIGHT: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_freight"));
pub static AuthorizeNetData_X_DUTY: crate::JavaStatic<String> = crate::JavaStatic::new(|| String::from("x_duty"));

impl AuthorizeNetData {

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

