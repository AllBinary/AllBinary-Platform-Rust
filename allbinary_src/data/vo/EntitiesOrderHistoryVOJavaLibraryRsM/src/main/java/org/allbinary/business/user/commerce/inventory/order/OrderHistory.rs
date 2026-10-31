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
// package org.allbinary.business.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Date;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Date::Date;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::business::user::commerce::money::payment::Payment::Payment;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::Order::Order;
use super::OrderHistoryInterface::OrderHistoryInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHistory {
    pub base: Order,
    pub userName: String,
    pub shippingAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub billingAddress: Option<Rc<RefCell<StreetAddress>>>,
    pub payment: Option<Rc<RefCell<Payment>>>,
    pub shipDate: String,
    pub orderDate: String,
    pub transDate: String,
    pub cancelDate: String,
    pub status: String,
    pub subTotal: Option<Rc<RefCell<Money>>>,
    pub shippingCost: Option<Rc<RefCell<Money>>>,
    pub taxes: Option<Rc<RefCell<Money>>>,
    pub total: Option<Rc<RefCell<Money>>>,
}

// impl OrderHistoryInterface for OrderHistory with the methods of the trait

impl OrderHistory {
    

    pub fn new_struct1(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>) -> Self {
        let mut this = Self {
            base: Order::new_struct1(basketInterface.clone()),
            userName: String::new(),
            shippingAddress: None,
            billingAddress: None,
            payment: None,
            shipDate: String::new(),
            orderDate: String::new(),
            transDate: String::new(),
            cancelDate: String::new(),
            status: String::new(),
            subTotal: None,
            shippingCost: None,
            taxes: None,
            total: None,
        };
        // super(...) is in the struct literal above
        this
    }

    pub fn new1(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(basketInterface))))
    }
    

    pub fn new_struct2(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>, mut orderHistory: Option<Rc<RefCell<OrderHistory>>>) -> Self {
        let mut this = Self {
            base: Order::new_struct6(jcall!(orderHistory).getStoreName(), basketInterface.clone(), jcall!(orderHistory).getId(), jcall!(orderHistory).getShippingMethod(), jcall!(orderHistory).getSpecial(), jcall!(orderHistory).getPaymentMethod()),
            userName: String::new(),
            shippingAddress: None,
            billingAddress: None,
            payment: None,
            shipDate: String::new(),
            orderDate: String::new(),
            transDate: String::new(),
            cancelDate: String::new(),
            status: String::new(),
            subTotal: None,
            shippingCost: None,
            taxes: None,
            total: None,
        };
        // super(...) is in the struct literal above
        this.userName = jcall!(orderHistory).getUserName();
        this.shippingAddress = jcall!(orderHistory).getShippingAddress();
        this.billingAddress = jcall!(orderHistory).getBillingAddress();
        this.payment = jcall!(orderHistory).getPaymentInfo();
        this.shipDate = jcall!(orderHistory).getShipDate();
        this.orderDate = jcall!(orderHistory).getOrderDate();
        this.transDate = jcall!(orderHistory).getTransDate();
        this.cancelDate = jcall!(orderHistory).getCancelDate();
        this.status = jcall!(orderHistory).getStatus();
        this.subTotal = jcall!(orderHistory).getSubTotal();
        this.shippingCost = jcall!(orderHistory).getShippingCost();
        this.taxes = jcall!(orderHistory).getTaxes();
        this.total = jcall!(orderHistory).getTotal();
        this.base.setUserComments((jcall!(orderHistory).getUserComments()).clone() /* as String */);
        this.base.setUserCancelComments((jcall!(orderHistory).getUserCancelComments()).clone() /* as String */);
        this.base.setStoreComments((jcall!(orderHistory).getStoreComments()).clone() /* as String */);
        this.base.setStoreCancelComments((jcall!(orderHistory).getStoreCancelComments()).clone() /* as String */);
        this
    }

    pub fn new2(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>, mut orderHistory: Option<Rc<RefCell<OrderHistory>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(basketInterface, orderHistory))))
    }
    

    pub fn new_struct2(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>, mut orderHistoryHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            base: Order::new_struct6((jcall!(orderHistoryHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */, basketInterface.clone(), (jcall!(orderHistoryHashMap).get(OrderData_ID.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(jcall!(EntryData::getInstance()).SPECIAL.clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone())).clone() /* as String */),
            userName: String::new(),
            shippingAddress: None,
            billingAddress: None,
            payment: None,
            shipDate: String::new(),
            orderDate: String::new(),
            transDate: String::new(),
            cancelDate: String::new(),
            status: String::new(),
            subTotal: None,
            shippingCost: None,
            taxes: None,
            total: None,
        };
        // super(...) is in the struct literal above
        this.shippingAddress = StreetAddress::new6((jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone())).clone() /* as String */);
        this.billingAddress = StreetAddress::new6((jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone())).clone() /* as String */, (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone())).clone() /* as String */);
        this.payment = Payment::new1(orderHistoryHashMap.clone());
        this.userName = (jcall!(orderHistoryHashMap).get(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
        this.orderDate = (jcall!(orderHistoryHashMap).get(OrderHistoryData_ORDERDATE.with(Clone::clone).clone())).clone() /* as String */;
        this.shipDate = (jcall!(orderHistoryHashMap).get(OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone())).clone() /* as String */;
        this.transDate = (jcall!(orderHistoryHashMap).get(OrderHistoryData_TRANSDATE.with(Clone::clone).clone())).clone() /* as String */;
        this.cancelDate = (jcall!(orderHistoryHashMap).get(OrderHistoryData_CANCELDATE.with(Clone::clone).clone())).clone() /* as String */;
        this.status = (jcall!(orderHistoryHashMap).get(OrderHistoryData_STATUS.with(Clone::clone).clone())).clone() /* as String */;
        this.subTotal = Money::new1((jcall!(orderHistoryHashMap).get(OrderHistoryData_SUBTOTAL.with(Clone::clone).clone())).clone() /* as String */);
        this.total = Money::new1((jcall!(orderHistoryHashMap).get(OrderHistoryData_TOTAL.with(Clone::clone).clone())).clone() /* as String */);
        this.shippingCost = Money::new1((jcall!(orderHistoryHashMap).get(OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone())).clone() /* as String */);
        this.taxes = Money::new1((jcall!(orderHistoryHashMap).get(OrderHistoryData_TAX.with(Clone::clone).clone())).clone() /* as String */);
        this.base.setUserComments((jcall!(orderHistoryHashMap).get(OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone())).clone() /* as String */);
        this.base.setUserCancelComments((jcall!(orderHistoryHashMap).get(OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone())).clone() /* as String */);
        this.base.setStoreComments((jcall!(orderHistoryHashMap).get(OrderData_STORECOMMENT.with(Clone::clone).clone())).clone() /* as String */);
        this.base.setStoreCancelComments((jcall!(orderHistoryHashMap).get(OrderData_STORECANCELCOMMENT.with(Clone::clone).clone())).clone() /* as String */);
        this
    }

    pub fn new2(mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>>, mut orderHistoryHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct2(basketInterface, orderHistoryHashMap))))
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.userName = value.clone();
    }
    

    pub fn setDefaultShippingAddress(&mut self, mut shippingAddress: Option<Rc<RefCell<StreetAddress>>>) {
        self.shippingAddress = shippingAddress.clone();
    }
    

    pub fn setBillingAddress(&mut self, mut billingAddress: Option<Rc<RefCell<StreetAddress>>>) {
        self.billingAddress = billingAddress.clone();
    }
    

    pub fn setPaymentInfo(&mut self, mut payment: Option<Rc<RefCell<Payment>>>) {
        self.payment = payment.clone();
    }
    

    pub fn setShipDate(&mut self, mut value: String) {
        self.shipDate = value.clone();
    }
    

    pub fn setOrderDate(&mut self, mut value: String) {
        self.orderDate = value.clone();
    }
    

    pub fn setTransDate(&mut self, mut value: String) {
        self.transDate = value.clone();
    }
    

    pub fn setCancelDate(&mut self, mut value: String) {
        self.cancelDate = value.clone();
    }
    

    pub fn setStatus(&mut self, mut value: String) {
        self.status = value.clone();
    }
    

    pub fn setSubTotal(&mut self, mut value: Option<Rc<RefCell<Money>>>) {
        self.subTotal = value.clone();
    }
    

    pub fn setShippingCost(&mut self, mut value: Option<Rc<RefCell<Money>>>) {
        self.shippingCost = value.clone();
    }
    

    pub fn setTaxes(&mut self, mut value: Option<Rc<RefCell<Money>>>) {
        self.taxes = value.clone();
    }
    

    pub fn setTotal(&mut self, mut value: Option<Rc<RefCell<Money>>>) {
        self.total = value.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getShippingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>> {
        return self.shippingAddress.clone();
    }
    

    pub fn getBillingAddress(&mut self) -> Option<Rc<RefCell<StreetAddress>>> {
        return self.billingAddress.clone();
    }
    

    pub fn getPaymentInfo(&mut self) -> Option<Rc<RefCell<Payment>>> {
        return self.payment.clone();
    }
    

    pub fn getShipDate(&mut self) -> String {
        return self.shipDate.clone();
    }
    

    pub fn getOrderDate(&mut self) -> String {
        return self.orderDate.clone();
    }
    

    pub fn getTransDate(&mut self) -> String {
        return self.transDate.clone();
    }
    

    pub fn getCancelDate(&mut self) -> String {
        return self.cancelDate.clone();
    }
    

    pub fn getStatus(&mut self) -> String {
        return self.status.clone();
    }
    

    pub fn getSubTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.subTotal.clone();
    }
    

    pub fn getShippingCost(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.shippingCost.clone();
    }
    

    pub fn getTaxes(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.taxes.clone();
    }
    

    pub fn getTotal(&mut self) -> Option<Rc<RefCell<Money>>> {
        return self.total.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
        jcall!(hashMap).put(OrderHistoryData_ORDERDATE.with(Clone::clone).clone(), self.orderDate.clone());
        jcall!(hashMap).put(OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone(), self.shipDate.clone());
        jcall!(hashMap).put(OrderHistoryData_TRANSDATE.with(Clone::clone).clone(), self.transDate.clone());
        jcall!(hashMap).put(OrderHistoryData_CANCELDATE.with(Clone::clone).clone(), self.cancelDate.clone());
        jcall!(hashMap).put(OrderHistoryData_ORDERDATEFORMATTED.with(Clone::clone).clone(), jcall!(Date::new(((Some((self.orderDate.clone()) as i64)).unwrap() as i64))).toString());
        jcall!(hashMap).put(OrderHistoryData_SHIPPEDDATEFORMATTED.with(Clone::clone).clone(), jcall!(Date::new(((Some((self.shipDate.clone()) as i64)).unwrap() as i64))).toString());
        jcall!(hashMap).put(OrderHistoryData_TRANSDATEFORMATTED.with(Clone::clone).clone(), jcall!(Date::new(((Some((self.transDate.clone()) as i64)).unwrap() as i64))).toString());
        jcall!(hashMap).put(OrderHistoryData_CANCELDATEFORMATTED.with(Clone::clone).clone(), jcall!(Date::new(((Some((self.cancelDate.clone()) as i64)).unwrap() as i64))).toString());
        jcall!(hashMap).put(OrderHistoryData_STATUS.with(Clone::clone).clone(), self.status.clone());
        jcall!(hashMap).put(OrderHistoryData_SUBTOTAL.with(Clone::clone).clone(), jcall!(self.subTotal).toString());
        jcall!(hashMap).put(OrderHistoryData_TOTAL.with(Clone::clone).clone(), jcall!(self.total).toString());
        jcall!(hashMap).put(OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone(), jcall!(self.shippingCost).toString());
        jcall!(hashMap).put(OrderHistoryData_TAX.with(Clone::clone).clone(), jcall!(self.taxes).toString());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMap();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(OrderHistoryData_ORDERHISTORY.with(Clone::clone).clone());
        jcall!(node).appendChild(self.base.toXmlNode(document.clone()));
        let mut billingAddressNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_BILLINGADDRESS.with(Clone::clone).clone());
        jcall!(billingAddressNode).appendChild(jcall!(self.billingAddress).toXmlNode(document.clone()));
        let mut shippingAddressNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_SHIPPINGADDRESS.with(Clone::clone).clone());
        jcall!(shippingAddressNode).appendChild(jcall!(self.shippingAddress).toXmlNode(document.clone()));
        jcall!(node).appendChild(billingAddressNode.clone());
        jcall!(node).appendChild(shippingAddressNode.clone());
        jcall!(node).appendChild(jcall!(self.payment).toXmlNode(document.clone()));
        for i in (0..size) {
            let mut name: String = (keyArray[(i) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
            value = jcall!(StringUtil::getInstance()).getNonNull(value.clone());
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
}
impl std::ops::Deref for OrderHistory {
    type Target = Order;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderHistory {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn for OrderHistory {
    fn as_Order(&self) -> &crate::org::allbinary::business::user::commerce::inventory::order::Order::Order { &self.base }
    fn as_Order_mut(&mut self) -> &mut crate::org::allbinary::business::user::commerce::inventory::order::Order::Order { &mut self.base }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}
impl crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface for OrderHistory {
    fn setShippingMethod(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setShippingMethod(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setPaymentMethod(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setPaymentMethod(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setSpecial(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setSpecial(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setStoreName(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setStoreName(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setUserComments(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setUserComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setUserCancelComments(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setUserCancelComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setStoreComments(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setStoreComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn setStoreCancelComments(&mut self, p1: String) { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::setStoreCancelComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self), p1) }
    fn getBasket(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface>>> { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getBasket(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getShippingMethod(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getShippingMethod(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getPaymentMethod(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getPaymentMethod(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getSpecial(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getSpecial(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getId(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getId(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getStoreName(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getStoreName(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getUserComments(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getUserComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getUserCancelComments(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getUserCancelComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getStoreComments(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getStoreComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
    fn getStoreCancelComments(&mut self) -> String { crate::org::allbinary::business::user::commerce::inventory::order::Order::Order::getStoreCancelComments(crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn::as_Order_mut(self)) }
}


