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

thread_local! { pub static AuthorizeNetData_X_LOGIN: String = String::from("x_login"); }
thread_local! { pub static AuthorizeNetData_X_TRAN_KEY: String = String::from("x_tran_key"); }
thread_local! { pub static AuthorizeNetData_X_VERSION: String = String::from("x_version"); }
thread_local! { pub static AuthorizeNetData_X_TEST_REQUEST: String = String::from("x_test_request"); }
thread_local! { pub static AuthorizeNetData_X_RELAY_RESPONSE: String = String::from("x_relay_response"); }
thread_local! { pub static AuthorizeNetData_X_DUPLICATE_WINDOW: String = String::from("x_duplicate_window"); }
thread_local! { pub static AuthorizeNetData_X_FIRST_NAME: String = String::from("x_first_name"); }
thread_local! { pub static AuthorizeNetData_X_LAST_NAME: String = String::from("x_last_name"); }
thread_local! { pub static AuthorizeNetData_X_COMPANY: String = String::from("x_company"); }
thread_local! { pub static AuthorizeNetData_X_ADDRESS: String = String::from("x_address"); }
thread_local! { pub static AuthorizeNetData_X_CITY: String = String::from("x_city"); }
thread_local! { pub static AuthorizeNetData_X_STATE: String = String::from("x_state"); }
thread_local! { pub static AuthorizeNetData_X_ZIP: String = String::from("x_zip"); }
thread_local! { pub static AuthorizeNetData_X_COUNTRY: String = String::from("x_country"); }
thread_local! { pub static AuthorizeNetData_X_PHONE: String = String::from("x_phone"); }
thread_local! { pub static AuthorizeNetData_X_FAX: String = String::from("x_fax"); }
thread_local! { pub static AuthorizeNetData_X_CUST_ID: String = String::from("x_cust_id"); }
thread_local! { pub static AuthorizeNetData_X_CUSTOMER_IP: String = String::from("x_customer_ip"); }
thread_local! { pub static AuthorizeNetData_X_CUSTOMER_TAX_ID: String = String::from("x_customer_tax_id"); }
thread_local! { pub static AuthorizeNetData_X_EMAIL_CUSTOMER: String = String::from("x_email_customer"); }
thread_local! { pub static AuthorizeNetData_X_EMAIL: String = String::from("x_email"); }
thread_local! { pub static AuthorizeNetData_X_EMAIL_MERCHANT: String = String::from("x_email_merchant"); }
thread_local! { pub static AuthorizeNetData_X_INVOICE_NUM: String = String::from("x_invoice_num"); }
thread_local! { pub static AuthorizeNetData_X_DESCRIPTION: String = String::from("x_description"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_FIRST_NAME: String = String::from("x_ship_to_first_name"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_LAST_NAME: String = String::from("x_ship_to_last_name"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_COMPANY: String = String::from("x_ship_to_company"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_ADDRESS: String = String::from("x_ship_to_address"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_CITY: String = String::from("x_ship_to_city"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_STATE: String = String::from("x_ship_to_state"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_ZIP: String = String::from("x_ship_to_zip"); }
thread_local! { pub static AuthorizeNetData_X_SHIP_TO_COUNTRY: String = String::from("x_ship_to_country"); }
thread_local! { pub static AuthorizeNetData_X_AMOUNT: String = String::from("x_amount"); }
thread_local! { pub static AuthorizeNetData_X_CURRENCY_CODE: String = String::from("x_currency_code"); }
thread_local! { pub static AuthorizeNetData_X_METHOD: String = String::from("x_method"); }
thread_local! { pub static AuthorizeNetData_X_TYPE: String = String::from("x_type"); }
thread_local! { pub static AuthorizeNetData_X_RECURRING_BILLING: String = String::from("x_recurring_billing"); }
thread_local! { pub static AuthorizeNetData_X_BANK_ABA_CODE: String = String::from("x_bank_aba_code"); }
thread_local! { pub static AuthorizeNetData_X_BANK_ACCT_NUM: String = String::from("x_bank_acct_num"); }
thread_local! { pub static AuthorizeNetData_X_BANK_ACCT_TYPE: String = String::from("x_bank_acct_type"); }
thread_local! { pub static AuthorizeNetData_X_BANK_NAME: String = String::from("x_bank_name"); }
thread_local! { pub static AuthorizeNetData_X_BANK_ACCT_NAME: String = String::from("x_bank_acct_name"); }
thread_local! { pub static AuthorizeNetData_X_ECHECK_TYPE: String = String::from("x_echeck_type"); }
thread_local! { pub static AuthorizeNetData_X_CARD_NUM: String = String::from("x_card_num"); }
thread_local! { pub static AuthorizeNetData_X_EXP_DATE: String = String::from("x_exp_date"); }
thread_local! { pub static AuthorizeNetData_X_CARD_CODE: String = String::from("x_card_code"); }
thread_local! { pub static AuthorizeNetData_X_TRANS_ID: String = String::from("x_trans_id"); }
thread_local! { pub static AuthorizeNetData_X_AUTH_CODE: String = String::from("x_auth_code"); }
thread_local! { pub static AuthorizeNetData_X_PO_NUM: String = String::from("x_po_num"); }
thread_local! { pub static AuthorizeNetData_X_TAX: String = String::from("x_tax"); }
thread_local! { pub static AuthorizeNetData_X_TAX_EXEMPT: String = String::from("x_tax_exempt"); }
thread_local! { pub static AuthorizeNetData_X_FREIGHT: String = String::from("x_freight"); }
thread_local! { pub static AuthorizeNetData_X_DUTY: String = String::from("x_duty"); }

impl AuthorizeNetData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

