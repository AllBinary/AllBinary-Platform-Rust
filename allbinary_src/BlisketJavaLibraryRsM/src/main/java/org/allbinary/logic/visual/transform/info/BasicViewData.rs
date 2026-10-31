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
// package org.allbinary.logic.visual.transform.info
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

pub struct BasicViewData {
    pub GENERICDIR: String,
    pub USERDIR: String,
    pub ADDRESSDIR: String,
    pub BILLINGADDRESSDIR: String,
    pub SHIPPINGADDRESSDIR: String,
    pub COMMERCEDIR: String,
    pub INVENTORYDIR: String,
    pub BASKETDIR: String,
    pub ITEMDIR: String,
    pub MONEYDIR: String,
    pub SHIPPINGDIR: String,
    pub ORDERDIR: String,
    pub ORDEREMAILDIR: String,
    pub ORDERHISTORYDIR: String,
    pub ORDERREVIEWDIR: String,
    pub PAYMENTDIR: String,
    pub GENERATIONDIR: String,
    pub PRODUCTSDIR: String,
    pub IGNOREXMLXSL: String,
    pub REVIEWORDERVIEWXSL: String,
    pub ORDERVIEWXSL: String,
    pub INVENTORYXSL: String,
    pub BASKETXSL: String,
    pub CHECKOUTBASKETXSL: String,
    pub MINIBASKETXSL: String,
    pub BILLINGADDRESSCHANGEFORMXSL: String,
    pub BILLINGADDRESSINPUTFORMXSL: String,
    pub BILLINGADDRESSVIEWXSL: String,
    pub SHIPPINGADDRESSCHANGEFORMXSL: String,
    pub SHIPPINGADDRESSINPUTFORMXSL: String,
    pub SHIPPINGADDRESSVIEWXSL: String,
    pub NEWTAXADDRESSXSL: String,
    pub EDITTAXADDRESSXSL: String,
    pub SHIPPINGMETHODSXSL: String,
    pub ORDERCUSTOMEREMAILVIEWXSL: String,
    pub ORDERMANAGEREMAILVIEWXSL: String,
    pub EDITCUSTOMERXSL: String,
    pub ORDERHISTORYXSL: String,
    pub PAYMENTVIEWXSL: String,
    pub SUMMARYXSL: String,
    pub PAYMENTGATEWAYOPTIONSXSL: String,
    pub MAKEPAYMENTVIEWXSL: String,
}

thread_local! { pub static BasicViewData_instance: Option<Rc<RefCell<BasicViewData>>> = BasicViewData::new(); }

impl BasicViewData {
    

    pub fn getInstance() -> Option<Rc<RefCell<BasicViewData>>> {
        return BasicViewData_instance.with(Clone::clone).clone();
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            GENERICDIR: String::new(),
            USERDIR: String::new(),
            ADDRESSDIR: String::new(),
            BILLINGADDRESSDIR: String::new(),
            SHIPPINGADDRESSDIR: String::new(),
            COMMERCEDIR: String::new(),
            INVENTORYDIR: String::new(),
            BASKETDIR: String::new(),
            ITEMDIR: String::new(),
            MONEYDIR: String::new(),
            SHIPPINGDIR: String::new(),
            ORDERDIR: String::new(),
            ORDEREMAILDIR: String::new(),
            ORDERHISTORYDIR: String::new(),
            ORDERREVIEWDIR: String::new(),
            PAYMENTDIR: String::new(),
            GENERATIONDIR: String::new(),
            PRODUCTSDIR: String::new(),
            IGNOREXMLXSL: String::new(),
            REVIEWORDERVIEWXSL: String::new(),
            ORDERVIEWXSL: String::new(),
            INVENTORYXSL: String::new(),
            BASKETXSL: String::new(),
            CHECKOUTBASKETXSL: String::new(),
            MINIBASKETXSL: String::new(),
            BILLINGADDRESSCHANGEFORMXSL: String::new(),
            BILLINGADDRESSINPUTFORMXSL: String::new(),
            BILLINGADDRESSVIEWXSL: String::new(),
            SHIPPINGADDRESSCHANGEFORMXSL: String::new(),
            SHIPPINGADDRESSINPUTFORMXSL: String::new(),
            SHIPPINGADDRESSVIEWXSL: String::new(),
            NEWTAXADDRESSXSL: String::new(),
            EDITTAXADDRESSXSL: String::new(),
            SHIPPINGMETHODSXSL: String::new(),
            ORDERCUSTOMEREMAILVIEWXSL: String::new(),
            ORDERMANAGEREMAILVIEWXSL: String::new(),
            EDITCUSTOMERXSL: String::new(),
            ORDERHISTORYXSL: String::new(),
            PAYMENTVIEWXSL: String::new(),
            SUMMARYXSL: String::new(),
            PAYMENTGATEWAYOPTIONSXSL: String::new(),
            MAKEPAYMENTVIEWXSL: String::new(),
        };
        this.GENERICDIR = String::from("generic/");
        this.USERDIR = format!("{}{}", this.GENERICDIR, String::from("user/"));
        this.ADDRESSDIR = format!("{}{}", this.USERDIR, String::from("address/"));
        this.BILLINGADDRESSDIR = format!("{}{}", this.ADDRESSDIR, String::from("billing/"));
        this.SHIPPINGADDRESSDIR = format!("{}{}", this.ADDRESSDIR, String::from("shipping/"));
        this.COMMERCEDIR = format!("{}{}", this.USERDIR, String::from("commerce/"));
        this.INVENTORYDIR = format!("{}{}", this.COMMERCEDIR, String::from("inventory/"));
        this.BASKETDIR = format!("{}{}", this.INVENTORYDIR, String::from("basket/"));
        this.ITEMDIR = format!("{}{}", this.INVENTORYDIR, String::from("item/"));
        this.MONEYDIR = format!("{}{}", this.COMMERCEDIR, String::from("money/"));
        this.SHIPPINGDIR = format!("{}{}", this.COMMERCEDIR, String::from("shipping/"));
        this.ORDERDIR = format!("{}{}", this.INVENTORYDIR, String::from("order/"));
        this.ORDEREMAILDIR = format!("{}{}", this.ORDERDIR, String::from("email/"));
        this.ORDERHISTORYDIR = format!("{}{}", this.ORDERDIR, String::from("history/"));
        this.ORDERREVIEWDIR = format!("{}{}", this.ORDERDIR, String::from("review/"));
        this.PAYMENTDIR = format!("{}{}", this.MONEYDIR, String::from("payment/"));
        this.GENERATIONDIR = format!("{}{}", this.GENERICDIR, String::from("generation/"));
        this.PRODUCTSDIR = format!("{}{}", this.GENERICDIR, String::from("products/"));
        this.IGNOREXMLXSL = format!("{}{}", this.GENERICDIR, String::from("ignoreXmlView.xsl"));
        this.REVIEWORDERVIEWXSL = format!("{}{}", this.ORDERREVIEWDIR, String::from("reviewOrderView.xsl"));
        this.ORDERVIEWXSL = this.REVIEWORDERVIEWXSL.clone();
        this.INVENTORYXSL = format!("{}{}", this.INVENTORYDIR, String::from("inventory.xsl"));
        this.BASKETXSL = format!("{}{}", this.BASKETDIR, String::from("basket.xsl"));
        this.CHECKOUTBASKETXSL = format!("{}{}", this.BASKETDIR, String::from("staticBasket.xsl"));
        this.MINIBASKETXSL = format!("{}{}", this.BASKETDIR, String::from("miniBasket.xsl"));
        this.BILLINGADDRESSCHANGEFORMXSL = format!("{}{}", this.BILLINGADDRESSDIR, String::from("billingAddressChangeForm.xsl"));
        this.BILLINGADDRESSINPUTFORMXSL = format!("{}{}", this.BILLINGADDRESSDIR, String::from("billingAddressInputForm.xsl"));
        this.BILLINGADDRESSVIEWXSL = format!("{}{}", this.BILLINGADDRESSDIR, String::from("billingAddressView.xsl"));
        this.SHIPPINGADDRESSCHANGEFORMXSL = format!("{}{}", this.SHIPPINGADDRESSDIR, String::from("shippingAddressChangeForm.xsl"));
        this.SHIPPINGADDRESSINPUTFORMXSL = format!("{}{}", this.SHIPPINGADDRESSDIR, String::from("shippingAddressInputForm.xsl"));
        this.SHIPPINGADDRESSVIEWXSL = format!("{}{}", this.SHIPPINGADDRESSDIR, String::from("shippingAddressView.xsl"));
        this.NEWTAXADDRESSXSL = format!("{}{}", this.SHIPPINGADDRESSDIR, String::from("newTaxAddressView.xsl"));
        this.EDITTAXADDRESSXSL = format!("{}{}", this.SHIPPINGADDRESSDIR, String::from("editTaxAddressView.xsl"));
        this.SHIPPINGMETHODSXSL = format!("{}{}", this.SHIPPINGDIR, String::from("shippingMethods.xsl"));
        this.ORDERCUSTOMEREMAILVIEWXSL = format!("{}{}", this.ORDEREMAILDIR, String::from("orderCustomerEmailView.xsl"));
        this.ORDERMANAGEREMAILVIEWXSL = format!("{}{}", this.ORDEREMAILDIR, String::from("orderManagerEmailView.xsl"));
        this.EDITCUSTOMERXSL = format!("{}{}", this.USERDIR, String::from("editCustomerInputForm.xsl"));
        this.ORDERHISTORYXSL = format!("{}{}", this.ORDERHISTORYDIR, String::from("orderHistory.xsl"));
        this.PAYMENTVIEWXSL = format!("{}{}", this.PAYMENTDIR, String::from("paymentView.xsl"));
        this.SUMMARYXSL = format!("{}{}", this.ITEMDIR, String::from("summary/summary.xsl"));
        this.PAYMENTGATEWAYOPTIONSXSL = format!("{}{}", this.PAYMENTDIR, String::from("paymentGatewayOptions.xsl"));
        this.MAKEPAYMENTVIEWXSL = format!("{}{}", format!("{}{}", this.PAYMENTDIR, String::from("invoice/")), String::from("makePayment.xsl"));
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
}

