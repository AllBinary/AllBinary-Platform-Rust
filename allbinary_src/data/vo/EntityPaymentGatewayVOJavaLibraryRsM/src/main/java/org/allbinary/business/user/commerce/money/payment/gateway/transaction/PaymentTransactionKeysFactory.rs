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
// package org.allbinary.business.user.commerce.money.payment.gateway.transaction
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use super::PaymentTransactionKeys::PaymentTransactionKeys;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentTransactionKeysFactory {
    pub TRXTYPE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub TENDER: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ACCT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXPDATE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub AMT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub AUTHCODE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub MICR: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CHECKNUM: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub NAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub STREET: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CITY: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub STATE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ZIP: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DL: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EMAIL: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COMMENT1: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COMMENT2: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ORIGID: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PONUM: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DESC: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DESC1TO4: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub INVNUM: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOZIP: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SWIPE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub TAXAMT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COMMCARD: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DUTYAMT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub FRIEGHTAMT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ORDERDATE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub TAXEXEMPT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COUNTRYCODE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CUSTCODE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CVV2: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ABA: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ACCTTYPE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DISCOUNT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub FIRSTNAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub LASTNAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPFROMZIP: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PRENOTE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CHKTYPE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub DOB: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PHONENUM: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SS: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COMPANYNAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub COUNTRY: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub FREIGHTAMT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOCITY: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOFIRSTNAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOLASTNAME: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOSTATE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SHIPTOSTREET: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SPECIAL1: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SPECIAL2: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SPECIAL3: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PARTNER: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub VENDOR: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub USER: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PWD: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub CHKNUM: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub AVSADDR: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub AVSZIP: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub PNREF: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub RESPMSG: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub RESULT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ORIGRESULT: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub STATUS: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub FRAUDCODE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub FRAUDMSG: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub ERRCODE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub SCORE: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub REASON1: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub REASON2: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub REASON3: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION1: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION2: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION3: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION4: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION5: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION6: Option<Rc<RefCell<PaymentTransactionKeys>>>,
    pub EXCEPTION7: Option<Rc<RefCell<PaymentTransactionKeys>>>,
}

thread_local! { pub static PaymentTransactionKeysFactory_instance: Option<Rc<RefCell<PaymentTransactionKeysFactory>>> = PaymentTransactionKeysFactory::new(); }

impl PaymentTransactionKeysFactory {

    pub fn new_struct() -> Self {
        let mut this = Self {
            TRXTYPE: None,
            TENDER: None,
            ACCT: None,
            EXPDATE: None,
            AMT: None,
            AUTHCODE: None,
            MICR: None,
            CHECKNUM: None,
            NAME: None,
            STREET: None,
            CITY: None,
            STATE: None,
            ZIP: None,
            DL: None,
            EMAIL: None,
            COMMENT1: None,
            COMMENT2: None,
            ORIGID: None,
            PONUM: None,
            DESC: None,
            DESC1TO4: None,
            INVNUM: None,
            SHIPTOZIP: None,
            SWIPE: None,
            TAXAMT: None,
            COMMCARD: None,
            DUTYAMT: None,
            FRIEGHTAMT: None,
            ORDERDATE: None,
            TAXEXEMPT: None,
            COUNTRYCODE: None,
            CUSTCODE: None,
            CVV2: None,
            ABA: None,
            ACCTTYPE: None,
            DISCOUNT: None,
            FIRSTNAME: None,
            LASTNAME: None,
            SHIPFROMZIP: None,
            PRENOTE: None,
            CHKTYPE: None,
            DOB: None,
            PHONENUM: None,
            SS: None,
            COMPANYNAME: None,
            COUNTRY: None,
            FREIGHTAMT: None,
            SHIPTOCITY: None,
            SHIPTOFIRSTNAME: None,
            SHIPTOLASTNAME: None,
            SHIPTOSTATE: None,
            SHIPTOSTREET: None,
            SPECIAL1: None,
            SPECIAL2: None,
            SPECIAL3: None,
            PARTNER: None,
            VENDOR: None,
            USER: None,
            PWD: None,
            CHKNUM: None,
            AVSADDR: None,
            AVSZIP: None,
            PNREF: None,
            RESPMSG: None,
            RESULT: None,
            ORIGRESULT: None,
            STATUS: None,
            FRAUDCODE: None,
            FRAUDMSG: None,
            ERRCODE: None,
            SCORE: None,
            REASON1: None,
            REASON2: None,
            REASON3: None,
            EXCEPTION1: None,
            EXCEPTION2: None,
            EXCEPTION3: None,
            EXCEPTION4: None,
            EXCEPTION5: None,
            EXCEPTION6: None,
            EXCEPTION7: None,
        };
        this.TRXTYPE = PaymentTransactionKeys::new(String::from("TRXTYPE"));
        this.TENDER = PaymentTransactionKeys::new(String::from("TENDER"));
        this.ACCT = PaymentTransactionKeys::new(String::from("ACCT"));
        this.EXPDATE = PaymentTransactionKeys::new(String::from("EXPDATE"));
        this.AMT = PaymentTransactionKeys::new(String::from("AMT"));
        this.AUTHCODE = PaymentTransactionKeys::new(String::from("AUTHCODE"));
        this.MICR = PaymentTransactionKeys::new(String::from("MICR"));
        this.CHECKNUM = PaymentTransactionKeys::new(String::from("CHECKNUM"));
        this.NAME = PaymentTransactionKeys::new(String::from("NAME"));
        this.STREET = PaymentTransactionKeys::new(String::from("STREET"));
        this.CITY = PaymentTransactionKeys::new(String::from("CITY"));
        this.STATE = PaymentTransactionKeys::new(String::from("STATE"));
        this.ZIP = PaymentTransactionKeys::new(String::from("ZIP"));
        this.DL = PaymentTransactionKeys::new(String::from("DL"));
        this.EMAIL = PaymentTransactionKeys::new(String::from("EMAIL"));
        this.COMMENT1 = PaymentTransactionKeys::new(String::from("COMMENT1"));
        this.COMMENT2 = PaymentTransactionKeys::new(String::from("COMMENT2"));
        this.ORIGID = PaymentTransactionKeys::new(String::from("ORIGID"));
        this.PONUM = PaymentTransactionKeys::new(String::from("PONUM"));
        this.DESC = PaymentTransactionKeys::new(String::from("PaymentTransactionDESC"));
        this.DESC1TO4 = PaymentTransactionKeys::new(String::from("DESC1TO4"));
        this.INVNUM = PaymentTransactionKeys::new(String::from("INVNUM"));
        this.SHIPTOZIP = PaymentTransactionKeys::new(String::from("SHIPTOZIP"));
        this.SWIPE = PaymentTransactionKeys::new(String::from("SWIPE"));
        this.TAXAMT = PaymentTransactionKeys::new(String::from("TAXAMT"));
        this.COMMCARD = PaymentTransactionKeys::new(String::from("COMMCARD"));
        this.DUTYAMT = PaymentTransactionKeys::new(String::from("DUTYAMT"));
        this.FRIEGHTAMT = PaymentTransactionKeys::new(String::from("FRIEGHTAMT"));
        this.ORDERDATE = PaymentTransactionKeys::new(String::from("ORDERDATE"));
        this.TAXEXEMPT = PaymentTransactionKeys::new(String::from("TAXEXEMPT"));
        this.COUNTRYCODE = PaymentTransactionKeys::new(String::from("COUNTRYCODE"));
        this.CUSTCODE = PaymentTransactionKeys::new(String::from("CUSTCODE"));
        this.CVV2 = PaymentTransactionKeys::new(String::from("CVV2"));
        this.ABA = PaymentTransactionKeys::new(String::from("ABA"));
        this.ACCTTYPE = PaymentTransactionKeys::new(String::from("ACCTTYPE"));
        this.DISCOUNT = PaymentTransactionKeys::new(String::from("DISCOUNT"));
        this.FIRSTNAME = PaymentTransactionKeys::new(String::from("FIRSTNAME"));
        this.LASTNAME = PaymentTransactionKeys::new(String::from("LASNAME"));
        this.SHIPFROMZIP = PaymentTransactionKeys::new(String::from("SHIPFROMZIP"));
        this.PRENOTE = PaymentTransactionKeys::new(String::from("PRENOTE"));
        this.CHKTYPE = PaymentTransactionKeys::new(String::from("CHKTYPE"));
        this.DOB = PaymentTransactionKeys::new(String::from("DOB"));
        this.PHONENUM = PaymentTransactionKeys::new(String::from("PHONENUM"));
        this.SS = PaymentTransactionKeys::new(String::from("SS"));
        this.COMPANYNAME = PaymentTransactionKeys::new(String::from("COMPANYNAME"));
        this.COUNTRY = PaymentTransactionKeys::new(String::from("COUNTRY"));
        this.FREIGHTAMT = PaymentTransactionKeys::new(String::from("FREIGHTAMT"));
        this.SHIPTOCITY = PaymentTransactionKeys::new(String::from("SHIPTOCITY"));
        this.SHIPTOFIRSTNAME = PaymentTransactionKeys::new(String::from("SHIPTOFIRSTNAME"));
        this.SHIPTOLASTNAME = PaymentTransactionKeys::new(String::from("SHIPTOLASTNAME"));
        this.SHIPTOSTATE = PaymentTransactionKeys::new(String::from("SHIPTOSTATE"));
        this.SHIPTOSTREET = PaymentTransactionKeys::new(String::from("SHIPTOSTREET"));
        this.SPECIAL1 = PaymentTransactionKeys::new(String::from("SPECIAL1"));
        this.SPECIAL2 = PaymentTransactionKeys::new(String::from("SPECIAL2"));
        this.SPECIAL3 = PaymentTransactionKeys::new(String::from("SPECIAL3"));
        this.PARTNER = PaymentTransactionKeys::new(String::from("PARTNER"));
        this.VENDOR = PaymentTransactionKeys::new(String::from("VENDOR"));
        this.USER = PaymentTransactionKeys::new(String::from("USER"));
        this.PWD = PaymentTransactionKeys::new(String::from("PWD"));
        this.CHKNUM = PaymentTransactionKeys::new(String::from("CHKNUM"));
        this.AVSADDR = PaymentTransactionKeys::new(String::from("AVSADDR"));
        this.AVSZIP = PaymentTransactionKeys::new(String::from("AVSZIP"));
        this.PNREF = PaymentTransactionKeys::new(String::from("PNREF"));
        this.RESPMSG = PaymentTransactionKeys::new(String::from("RESPMSG"));
        this.RESULT = PaymentTransactionKeys::new(String::from("RESULT"));
        this.ORIGRESULT = PaymentTransactionKeys::new(String::from("ORIGRESULT"));
        this.STATUS = PaymentTransactionKeys::new(String::from("STATUS"));
        this.FRAUDCODE = PaymentTransactionKeys::new(String::from("FRAUDCODE"));
        this.FRAUDMSG = PaymentTransactionKeys::new(String::from("FRAUDMSG"));
        this.ERRCODE = PaymentTransactionKeys::new(String::from("ERRCODE"));
        this.SCORE = PaymentTransactionKeys::new(String::from("SCORE"));
        this.REASON1 = PaymentTransactionKeys::new(String::from("REASON1"));
        this.REASON2 = PaymentTransactionKeys::new(String::from("REASON2"));
        this.REASON3 = PaymentTransactionKeys::new(String::from("REASON3"));
        this.EXCEPTION1 = PaymentTransactionKeys::new(String::from("EXCEPTION1"));
        this.EXCEPTION2 = PaymentTransactionKeys::new(String::from("EXCEPTION2"));
        this.EXCEPTION3 = PaymentTransactionKeys::new(String::from("EXCEPTION3"));
        this.EXCEPTION4 = PaymentTransactionKeys::new(String::from("EXCEPTION4"));
        this.EXCEPTION5 = PaymentTransactionKeys::new(String::from("EXCEPTION5"));
        this.EXCEPTION6 = PaymentTransactionKeys::new(String::from("EXCEPTION6"));
        this.EXCEPTION7 = PaymentTransactionKeys::new(String::from("EXCEPTION7"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<PaymentTransactionKeysFactory>>> {
        return PaymentTransactionKeysFactory_instance.with(Clone::clone).clone();
    }
}

