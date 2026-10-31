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
use std::collections::HashMap;
use super::*;
// use crate::java::io::Serializable::Serializable;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::OrderInterface::OrderInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Order {
    pub basket: Option<Rc<RefCell<dyn BasketInterface>>>,
    pub paymentMethod: String,
    pub shippingType: String,
    pub storeName: String,
    pub special: String,
    pub id: String,
    pub userComment: String,
    pub userCancelComment: String,
    pub storeComment: String,
    pub storeCancelComment: String,
}

impl OrderInterface for Order {
    fn setShippingMethod(&mut self, mut shippingType: String) { Self::setShippingMethod(self, shippingType) }
    fn setPaymentMethod(&mut self, mut value: String) { Self::setPaymentMethod(self, value) }
    fn setSpecial(&mut self, mut special: String) { Self::setSpecial(self, special) }
    fn setStoreName(&mut self, mut value: String) { Self::setStoreName(self, value) }
    fn setUserComments(&mut self, mut value: String) { Self::setUserComments(self, value) }
    fn setUserCancelComments(&mut self, mut value: String) { Self::setUserCancelComments(self, value) }
    fn setStoreComments(&mut self, mut value: String) { Self::setStoreComments(self, value) }
    fn setStoreCancelComments(&mut self, mut value: String) { Self::setStoreCancelComments(self, value) }
    fn getBasket(&mut self) -> Option<Rc<RefCell<dyn BasketInterface>>> { Self::getBasket(self) }
    fn getShippingMethod(&mut self) -> String { Self::getShippingMethod(self) }
    fn getPaymentMethod(&mut self) -> String { Self::getPaymentMethod(self) }
    fn getSpecial(&mut self) -> String { Self::getSpecial(self) }
    fn getId(&mut self) -> String { Self::getId(self) }
    fn getStoreName(&mut self) -> String { Self::getStoreName(self) }
    fn getUserComments(&mut self) -> String { Self::getUserComments(self) }
    fn getUserCancelComments(&mut self) -> String { Self::getUserCancelComments(self) }
    fn getStoreComments(&mut self) -> String { Self::getStoreComments(self) }
    fn getStoreCancelComments(&mut self) -> String { Self::getStoreCancelComments(self) }
}
// impl Serializable for Order with the methods of the trait

impl Order {
    

    pub fn new_struct1(mut basket: Option<Rc<RefCell<dyn BasketInterface>>>) -> Self {
        let mut this = Self {
            basket: None,
            paymentMethod: String::new(),
            shippingType: String::new(),
            storeName: String::new(),
            special: String::new(),
            id: String::new(),
            userComment: String::new(),
            userCancelComment: String::new(),
            storeComment: String::new(),
            storeCancelComment: String::new(),
        };
        this.basket = basket.clone();
        this.id = None;
        this.shippingType = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        this
    }

    pub fn new1(mut basket: Option<Rc<RefCell<dyn BasketInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(basket))))
    }
    

    pub fn new_struct6(mut storeName: String, mut basket: Option<Rc<RefCell<dyn BasketInterface>>>, mut id: String, mut shippingType: String, mut special: String, mut paymentMethod: String) -> Self {
        let mut this = Self {
            basket: None,
            paymentMethod: String::new(),
            shippingType: String::new(),
            storeName: String::new(),
            special: String::new(),
            id: String::new(),
            userComment: String::new(),
            userCancelComment: String::new(),
            storeComment: String::new(),
            storeCancelComment: String::new(),
        };
        this.storeName = storeName.clone();
        this.basket = basket.clone();
        this.id = id.clone();
        this.shippingType = shippingType.clone();
        this.special = special.clone();
        this.paymentMethod = paymentMethod.clone();
        this
    }

    pub fn new6(mut storeName: String, mut basket: Option<Rc<RefCell<dyn BasketInterface>>>, mut id: String, mut shippingType: String, mut special: String, mut paymentMethod: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct6(storeName, basket, id, shippingType, special, paymentMethod))))
    }
    

    pub fn isIdValid(&mut self) -> Option<bool> {
        if self.getId().is_none() || jcall!(self.getId()).compareTo(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()) == 0 || jcall!(self.getId()).length() > unsafe { OrderHistoryData_MAXIDLEN.clone() } || !jcall!(StringValidationUtil::getInstance()).isNumber(self.getId()) {
            return false;
        }
        return true;
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        return true;
    }
    

    pub fn getIdValidationInfo(&mut self) -> String {
        let mut EMPTY_STRING: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
        if self.getId().is_none() || jcall!(self.getId()).compareTo(EMPTY_STRING.clone()) == 0 || jcall!(self.getId()).length() > unsafe { OrderHistoryData_MAXIDLEN.clone() } || !jcall!(StringValidationUtil::getInstance()).isNumber(self.getId()) {
            return format!("{}{}", format!("{}{}", String::from("Please enter a valid order Id < "), unsafe { OrderHistoryData_MAXIDLEN.clone() }), String::from(" digits in length.<br>"));
        }
        return EMPTY_STRING.clone();
    }
    

    pub fn setShippingMethod(&mut self, mut shippingType: String) {
        self.shippingType = shippingType.clone();
    }
    

    pub fn setPaymentMethod(&mut self, mut value: String) {
        self.paymentMethod = value.clone();
    }
    

    pub fn setSpecial(&mut self, mut special: String) {
        self.special = special.clone();
    }
    

    pub fn setId(&mut self, mut value: String) {
        self.id = value.clone();
    }
    

    pub fn setStoreName(&mut self, mut value: String) {
        self.storeName = value.clone();
    }
    

    pub fn setUserComments(&mut self, mut value: String) {
        self.userComment = value.clone();
    }
    

    pub fn setUserCancelComments(&mut self, mut value: String) {
        self.userCancelComment = value.clone();
    }
    

    pub fn setStoreComments(&mut self, mut value: String) {
        self.storeComment = value.clone();
    }
    

    pub fn setStoreCancelComments(&mut self, mut value: String) {
        self.storeCancelComment = value.clone();
    }
    

    pub fn getBasket(&mut self) -> Option<Rc<RefCell<dyn BasketInterface>>> {
        return self.basket.clone();
    }
    

    pub fn getShippingMethod(&mut self) -> String {
        return self.shippingType.clone();
    }
    

    pub fn getPaymentMethod(&mut self) -> String {
        return self.paymentMethod.clone();
    }
    

    pub fn getSpecial(&mut self) -> String {
        return self.special.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn getStoreName(&mut self) -> String {
        return self.storeName.clone();
    }
    

    pub fn getUserComments(&mut self) -> String {
        return self.userComment.clone();
    }
    

    pub fn getUserCancelComments(&mut self) -> String {
        return self.userCancelComment.clone();
    }
    

    pub fn getStoreComments(&mut self) -> String {
        return self.storeComment.clone();
    }
    

    pub fn getStoreCancelComments(&mut self) -> String {
        return self.storeCancelComment.clone();
    }
    

    pub fn reviewOrder(&mut self, mut paymentMethod: String) -> String {
        return String::from("Create your own Order.reviewOrder");
    }
    

    pub fn viewOrder(&mut self, mut paymentMethod: String) -> String {
        return String::from("Create your own Order.viewOrder");
    }
    

    pub fn emailOrder(&mut self, mut userName: String, mut paymentMethod: String, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> String {
        return String::from("Create your own Order.emailOrder");
    }
    

    pub fn toHashMapOrder(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone(), self.paymentMethod.clone());
        jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone(), self.shippingType.clone());
        jcall!(hashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), self.storeName.clone());
        jcall!(hashMap).put(jcall!(EntryData::getInstance()).SPECIAL.clone(), self.special.clone());
        jcall!(hashMap).put(OrderData_ID.with(Clone::clone).clone(), self.id.clone());
        jcall!(hashMap).put(OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone(), self.userComment.clone());
        jcall!(hashMap).put(OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone(), self.userCancelComment.clone());
        jcall!(hashMap).put(OrderData_STORECOMMENT.with(Clone::clone).clone(), self.storeComment.clone());
        jcall!(hashMap).put(OrderData_STORECANCELCOMMENT.with(Clone::clone).clone(), self.storeCancelComment.clone());
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.toHashMapOrder();
        let mut keySet: Option<Rc<RefCell<dyn Set>>> = jcall!(hashMap).keySet();
        let mut keyArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(keySet).toArray();
        let mut size: i32 = (keyArray.len() as i32).clone();
        let mut node: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(OrderData_ORDER.with(Clone::clone).clone());
        for i in (0..size) {
            let mut name: String = (keyArray[(i) as usize]).clone() /* as String */;
            let mut value: String = (jcall!(hashMap).get(name.clone())).clone() /* as String */;
            value = jcall!(StringUtil::getInstance()).getNonNull(value.clone());
            jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), name.clone(), value.clone()));
        }
        return node.clone();
    }
}
pub trait OrderDyn: crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface {
    fn as_Order(&self) -> &Order;
    fn as_Order_mut(&mut self) -> &mut Order;
    fn isIdValid(&mut self) -> Option<bool> { Order::isIdValid(self.as_Order_mut()) }
    fn isValid(&mut self) -> Option<bool> { Order::isValid(self.as_Order_mut()) }
    fn getIdValidationInfo(&mut self) -> String { Order::getIdValidationInfo(self.as_Order_mut()) }
    fn setId(&mut self, mut value: String) { Order::setId(self.as_Order_mut(), value) }
    fn reviewOrder(&mut self, mut paymentMethod: String) -> String { Order::reviewOrder(self.as_Order_mut(), paymentMethod) }
    fn viewOrder(&mut self, mut paymentMethod: String) -> String { Order::viewOrder(self.as_Order_mut(), paymentMethod) }
    fn emailOrder(&mut self, mut userName: String, mut paymentMethod: String, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> String { Order::emailOrder(self.as_Order_mut(), userName, paymentMethod, storeFrontInterface) }
    fn toHashMapOrder(&mut self) -> Option<Rc<RefCell<HashMap>>> { Order::toHashMapOrder(self.as_Order_mut()) }
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Order::toXmlNode(self.as_Order_mut(), document) }
}
impl OrderDyn for Order {
    fn as_Order(&self) -> &Order { self }
    fn as_Order_mut(&mut self) -> &mut Order { self }
}


