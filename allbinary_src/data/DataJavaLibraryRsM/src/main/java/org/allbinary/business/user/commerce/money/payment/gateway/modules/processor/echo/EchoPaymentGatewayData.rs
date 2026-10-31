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
// package org.allbinary.business.user.commerce.money.payment.gateway.modules.processor.echo
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

pub struct EchoPaymentGatewayData {
}

pub static mut EchoPaymentGatewayData_TRANSACTION_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("transaction_type"));
pub static mut EchoPaymentGatewayData_MERCHANT_ECHO_ID: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("merchant_echo_id"));
pub static mut EchoPaymentGatewayData_ORDER_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("order_type"));
pub static mut EchoPaymentGatewayData_MERCHANT_PIN: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("merchant_pin"));
pub static mut EchoPaymentGatewayData_ISP_ECHO_ID: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("isp_echo_id"));
pub static mut EchoPaymentGatewayData_ISP_PIN: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("isp_pin"));
pub static mut EchoPaymentGatewayData_BILLING_IP_ADDRESS: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_ip_address"));
pub static mut EchoPaymentGatewayData_MERCHANT_EMAIL: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("merchant_email"));
pub static mut EchoPaymentGatewayData_AUTHORIZATION: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("authorization"));
pub static mut EchoPaymentGatewayData_BILLING_PREFIX: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_prefix"));
pub static mut EchoPaymentGatewayData_BILLING_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_name"));
pub static mut EchoPaymentGatewayData_BILLING_FIRST_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_first_name"));
pub static mut EchoPaymentGatewayData_BILLING_LAST_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_last_name"));
pub static mut EchoPaymentGatewayData_BILLING_COMPANY_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_company_name"));
pub static mut EchoPaymentGatewayData_BILLING_ADDRESS1: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_address1"));
pub static mut EchoPaymentGatewayData_BILLING_ADDRESS2: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_address2"));
pub static mut EchoPaymentGatewayData_BILLING_CITY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_city"));
pub static mut EchoPaymentGatewayData_BILLING_STATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_state"));
pub static mut EchoPaymentGatewayData_BILLING_ZIP: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_zip"));
pub static mut EchoPaymentGatewayData_BILLING_COUNTRY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_country"));
pub static mut EchoPaymentGatewayData_BILLING_PHONE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_phone"));
pub static mut EchoPaymentGatewayData_BILLING_FAX: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_fax"));
pub static mut EchoPaymentGatewayData_BILLING_EMAIL: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("billing_email"));
pub static mut EchoPaymentGatewayData_CC_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("cc_number"));
pub static mut EchoPaymentGatewayData_CCEXP_MONTH: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ccexp_month"));
pub static mut EchoPaymentGatewayData_CCEXP_YEAR: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ccexp_year"));
pub static mut EchoPaymentGatewayData_CNP_RECURRING: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("cnp_recurring"));
pub static mut EchoPaymentGatewayData_cnp_security: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("cnp_security"));
pub static mut EchoPaymentGatewayData_COUNTER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("counter"));
pub static mut EchoPaymentGatewayData_DEBUG: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("debug"));
pub static mut EchoPaymentGatewayData_EC_ACCOUNT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_account"));
pub static mut EchoPaymentGatewayData_EC_ACCOUNT_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_account_type"));
pub static mut EchoPaymentGatewayData_EC_ADDRESS1: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_address1"));
pub static mut EchoPaymentGatewayData_EC_ADDRESS2: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_address2"));
pub static mut EchoPaymentGatewayData_EC_BANK_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_bank_name"));
pub static mut EchoPaymentGatewayData_EC_CITY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_city"));
pub static mut EchoPaymentGatewayData_EC_EMAIL: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_email"));
pub static mut EchoPaymentGatewayData_EC_FIRST_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_first_name"));
pub static mut EchoPaymentGatewayData_EC_ID_COUNTRY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_country"));
pub static mut EchoPaymentGatewayData_EC_ID_EXP_MM: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_exp_mm"));
pub static mut EchoPaymentGatewayData_EC_ID_EXP_DD: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_exp_dd"));
pub static mut EchoPaymentGatewayData_EC_ID_EXP_YY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_exp_yy"));
pub static mut EchoPaymentGatewayData_EC_ID_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_number"));
pub static mut EchoPaymentGatewayData_EC_ID_STATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_state"));
pub static mut EchoPaymentGatewayData_EC_ID_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_id_type"));
pub static mut EchoPaymentGatewayData_EC_LAST_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_last_name"));
pub static mut EchoPaymentGatewayData_EC_BUSINESS_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_BUSINESS_NAME"));
pub static mut EchoPaymentGatewayData_EC_LICENSE_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_LICENSE_NUMBER"));
pub static mut EchoPaymentGatewayData_EC_LICENSE_STATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_LICENSE_STATE"));
pub static mut EchoPaymentGatewayData_EC_MERCHANT_REF: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_MERCHANT_REF"));
pub static mut EchoPaymentGatewayData_EC_NBDS_CODE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_NBDS_CODE"));
pub static mut EchoPaymentGatewayData_EC_OTHER_NAME: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_other_name"));
pub static mut EchoPaymentGatewayData_EC_PAYEE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_payee"));
pub static mut EchoPaymentGatewayData_EC_PAYMENT_TYPE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_payment_type"));
pub static mut EchoPaymentGatewayData_EC_RT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_rt"));
pub static mut EchoPaymentGatewayData_EC_SERIAL_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_serial_number"));
pub static mut EchoPaymentGatewayData_EC_STATE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_state"));
pub static mut EchoPaymentGatewayData_EC_TRANSACTION_DT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_transaction_dt"));
pub static mut EchoPaymentGatewayData_EC_ZIP: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("ec_zip"));
pub static mut EchoPaymentGatewayData_GRAND_TOTAL: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("grand_total"));
pub static mut EchoPaymentGatewayData_MERCHANT_TRACE_NBR: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("merchant_trace_nbr"));
pub static mut EchoPaymentGatewayData_ORDER_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("order_number"));
pub static mut EchoPaymentGatewayData_ORIGINAL_AMOUNT: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("original_amount"));
pub static mut EchoPaymentGatewayData_ORIGINAL_TRANDATE_MM: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("original_trandate_mm"));
pub static mut EchoPaymentGatewayData_ORIGINAL_TRANDATE_DD: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("original_trandate_dd"));
pub static mut EchoPaymentGatewayData_ORIGINAL_TRANDATE_YYYY: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("original_trandate_yyyy"));
pub static mut EchoPaymentGatewayData_ORIGINAL_REFERENCE: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("original_reference"));
pub static mut EchoPaymentGatewayData_PRODUCT_DESCRIPTION: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("product_description"));
pub static mut EchoPaymentGatewayData_PURCHASE_ORDER_NUMBER: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("purchase_order_number"));
pub static mut EchoPaymentGatewayData_SALES_TAX: std::cell::LazyCell<String> = std::cell::LazyCell::new(|| String::from("sales_tax"));

impl EchoPaymentGatewayData {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
        };
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

