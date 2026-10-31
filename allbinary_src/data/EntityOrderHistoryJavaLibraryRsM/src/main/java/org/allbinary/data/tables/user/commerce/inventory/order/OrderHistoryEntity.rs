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
// package org.allbinary.data.tables.user.commerce.inventory.order
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Random::Random;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::HistoryDbInitInfo::HistoryDbInitInfo;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData;
use crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::commerce::inventory::basket::Basket::Basket;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::order::Order::Order;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistory::OrderHistory;
use crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentInterface::PaymentInterface;
use crate::org::allbinary::business::user::commerce::money::tax::TaxFactory::TaxFactory;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData;
use crate::org::allbinary::business::user::commerce::shipping::ShippingMethods::ShippingMethods;
use crate::org::allbinary::business::user::commerce::shipping::modules::ShippingInterface::ShippingInterface;
use crate::org::allbinary::data::generator::OrderHistoryIdGenerator::OrderHistoryIdGenerator;
use crate::org::allbinary::data::tables::TableDataFactory::TableDataFactory;
use crate::org::allbinary::data::tables::user::address::billing::BillingAddressesEntity::BillingAddressesEntity;
use crate::org::allbinary::data::tables::user::address::shipping::ShippingAddressesEntity::ShippingAddressesEntity;
use crate::org::allbinary::data::tables::user::commerce::money::payment::PaymentEntity::PaymentEntity;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::system::security::licensing::ServiceClientInformationInterfaceFactory::ServiceClientInformationInterfaceFactory;

// java.util.Random's seeded 48-bit linear congruential generator.
pub struct Random { seed: u64 }
impl Random {
    pub fn new(seed: i64) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self { seed: (seed as u64 ^ 0x5DEECE66D) & ((1u64 << 48) - 1) })))
    }
    pub fn nextInt(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(11) & ((1u64 << 48) - 1);
        (self.seed >> 16) as u32 as i32
    }
}
use super::OrderHistoryEntityInterface::OrderHistoryEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct OrderHistoryEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

impl OrderHistoryEntityInterface for OrderHistoryEntity {
    fn insert(&mut self, mut userName: String, mut order: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn>>>) { Self::insert2(self, userName, order) }
    fn setStatus(&mut self, mut orderId: String, mut status: String) { Self::setStatus(self, orderId, status) }
    fn setPaymentMethod(&mut self, mut orderId: String, mut paymentMethod: String) { Self::setPaymentMethod(self, orderId, paymentMethod) }
    fn getStoreOrders(&mut self, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getStoreOrders(self, storeFrontInterface) }
    fn getOrders(&mut self, mut userName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getOrders(self, userName) }
    fn getOrders3(&mut self, mut status: String, mut fromDate: String, mut toDate: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::getOrders3(self, status, fromDate, toDate) }
    fn getOrder(&mut self, mut id: String) -> Option<Rc<RefCell<OrderHistory>>> { Self::getOrder(self, id) }
    fn update(&mut self, mut whereHashMap: Option<Rc<RefCell<HashMap>>>, mut orderHashMap: Option<Rc<RefCell<HashMap>>>) { Self::update(self, whereHashMap, orderHashMap) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for OrderHistoryEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl OrderHistoryEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(HistoryDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("orderhistory");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn insert2(&mut self, mut userName: String, mut order: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::Order::OrderDyn>>>) {
        let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentInterface: Option<Rc<RefCell<dyn PaymentInterface>>> = jcall!(PaymentEntity::new()).getDefault(userName.clone());
            let mut billingAddress: Option<Rc<RefCell<StreetAddress>>> = jcall!(BillingAddressesEntity::new(userName.clone())).getDefault();
            let mut shippingAddress: Option<Rc<RefCell<StreetAddress>>> = jcall!(ShippingAddressesEntity::new(userName.clone())).getDefault();
            let mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>> = ServiceClientInformationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn AbeClientInformationInterface>>);
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(order).getStoreName());
            let mut shippingInterface: Option<Rc<RefCell<dyn ShippingInterface>>> = jcall!(ShippingMethods::new(abeClientInformation.clone(), storeFrontInterface.clone())).getShippingInterface(jcall!(order).getShippingMethod());
            let mut shippingCost: Option<Rc<RefCell<Money>>> = jcall!(shippingInterface).getCost(order.clone().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::business::user::commerce::inventory::order::OrderInterface::OrderInterface>>));
            let mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(order).getBasket();
            let mut subTotal: Option<Rc<RefCell<Money>>> = jcall!(basketInterface).getSubTotal();
            let mut taxRate: Option<f32> = jcall!(jcall!(TaxFactory::getInstance()).getInstance2(abeClientInformation.clone(), storeFrontInterface.clone())).getTaxRate(shippingAddress.clone(), storeFrontInterface.clone());
            let mut tax: Option<Rc<RefCell<Money>>> = Money::new();
            let mut total: Option<Rc<RefCell<Money>>> = Money::new();
            jcall!(total).add(jcall!(shippingCost).toString());
            jcall!(total).add(jcall!(subTotal).toString());
            jcall!(tax).add(jcall!(total).toString());
            jcall!(tax).multiply(taxRate.clone());
            jcall!(total).add(jcall!(tax).toString());
            let mut empty: String = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            jcall!(vector).add(crate::java_into_object(jcall!(OrderHistoryIdGenerator::new()).getNext()));
            let mut ZERO: String = jcall!(TableDataFactory::getInstance()).ZERO_STRING.clone();
            jcall!(vector).add(crate::java_into_object(jcall!(order).getId()));
            jcall!(vector).add(crate::java_into_object(userName.clone()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getStoreName()));
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            jcall!(vector).add(crate::java_into_object(ZERO.clone()));
            jcall!(vector).add(crate::java_into_object(time.clone()));
            jcall!(vector).add(crate::java_into_object(ZERO.clone()));
            jcall!(vector).add(crate::java_into_object(ZERO.clone()));
            jcall!(vector).add(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_PREPROCESSING.with(Clone::clone).clone()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getPaymentMethod()));
            if paymentInterface.is_some() {
                jcall!(vector).add(crate::java_into_object(jcall!(paymentInterface).getName()));
                jcall!(vector).add(crate::java_into_object(jcall!(paymentInterface).getType()));
                jcall!(vector).add(crate::java_into_object(jcall!(paymentInterface).getExpiration()));
                let mut random: i32 = jcall!(Random::new()).nextInt(unsafe { crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt_KEYMAX.clone() }.clone());
                jcall!(vector).add(crate::java_into_object(jcall!(SuperCrypt::new(random.clone())).encrypt(jcall!(paymentInterface).getNumber())));
                jcall!(vector).add(crate::java_into_object(jcall!(Some((random.clone()) as i32)).toString()));
            } else {
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(ZERO.clone()));
            }
            if billingAddress.is_some() {
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getName()));
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getStreet()));
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getCity()));
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getState()));
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getCode()));
                jcall!(vector).add(crate::java_into_object(jcall!(billingAddress).getCountry()));
            } else {
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
            }
            if shippingAddress.is_some() {
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getName()));
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getStreet()));
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getCity()));
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getState()));
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getCode()));
                jcall!(vector).add(crate::java_into_object(jcall!(shippingAddress).getCountry()));
            } else {
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
                jcall!(vector).add(crate::java_into_object(empty.clone()));
            }
            jcall!(vector).add(crate::java_into_object(jcall!(order).getShippingMethod()));
            jcall!(vector).add(crate::java_into_object(jcall!(subTotal).toString()));
            jcall!(vector).add(crate::java_into_object(jcall!(shippingCost).toString()));
            jcall!(vector).add(crate::java_into_object(jcall!(tax).toString()));
            jcall!(vector).add(crate::java_into_object(jcall!(total).toString()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getSpecial()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getUserComments()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getUserCancelComments()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getStoreComments()));
            jcall!(vector).add(crate::java_into_object(jcall!(order).getStoreCancelComments()));
            self.insert(vector.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(format!("{}{}", String::from("Command Failed: "), vector), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.base.insert(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), INSERT.clone(), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn setStatus(&mut self, mut orderId: String, mut status: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            let mut updateHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), status.clone());
            if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELLED.with(Clone::clone).clone()) as i32) == 0 {
                jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone(), time.clone());
            } else if (status.cmp(&crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPED.with(Clone::clone).clone()) as i32) == 0 {
                jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone(), time.clone());
            }
            self.base.updateWhere3(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderId.clone(), updateHashMap.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setStatus"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn setPaymentMethod(&mut self, mut orderId: String, mut paymentMethod: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            let mut updateHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone(), paymentMethod.clone());
            jcall!(updateHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone(), time.clone());
            self.base.updateWhere3(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), orderId.clone(), updateHashMap.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setPaymentMethod"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    // throws Exception
    pub fn getStoreOrders(&mut self, mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>>) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut orderReviewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(whereHashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), jcall!(storeFrontInterface).getName());
        let mut orderHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(whereHashMap.clone());
        let mut size: i32 = jcall!(orderHashMapVector).size();
        for i in (0..size) {
            let mut orderReviewHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(orderHashMapVector).get(i.clone())).clone() /* as HashMap */;
            let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new2(Basket::new(), orderReviewHashMap.clone());
            jcall!(orderReviewVector).add(crate::java_into_object(orderReview.clone()));
        }
        return orderReviewVector.clone();
    }
    

    // throws Exception
    pub fn getOrders(&mut self, mut userName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut orderReviewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(whereHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), userName.clone());
        let mut orderHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(whereHashMap.clone());
        let mut size: i32 = jcall!(orderHashMapVector).size();
        for index in (0..size) {
            let mut orderReviewHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(orderHashMapVector).get(index.clone())).clone() /* as HashMap */;
            let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new2(Basket::new(), orderReviewHashMap.clone());
            jcall!(orderReviewVector).add(crate::java_into_object(orderReview.clone()));
        }
        return orderReviewVector.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getOrders3(&mut self, mut status: String, mut fromDate: String, mut toDate: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut orderReviewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone(), status.clone());
        let mut orderHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRowsWhereBetween4(whereHashMap.clone(), crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone(), fromDate.clone(), toDate.clone());
        let mut size: i32 = jcall!(orderHashMapVector).size();
        for index in (0..size) {
            let mut orderReviewHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(orderHashMapVector).get(index.clone())).clone() /* as HashMap */;
            let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new2(Basket::new(), orderReviewHashMap.clone());
            jcall!(orderReviewVector).add(crate::java_into_object(orderReview.clone()));
        }
        return orderReviewVector.clone();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn getOrders2(&mut self, mut fromDate: String, mut toDate: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut orderReviewVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        let mut orderHashMapVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRowsWhereBetween(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone(), fromDate.clone(), toDate.clone());
        let mut size: i32 = jcall!(orderHashMapVector).size();
        for index in (0..size) {
            let mut orderReviewHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(orderHashMapVector).get(index.clone())).clone() /* as HashMap */;
            let mut orderReview: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new2(Basket::new(), orderReviewHashMap.clone());
            jcall!(orderReviewVector).add(crate::java_into_object(orderReview.clone()));
        }
        return orderReviewVector.clone();
    }
    

    // throws Exception
    pub fn getOrder(&mut self, mut id: String) -> Option<Rc<RefCell<OrderHistory>>> {
        let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(whereHashMap).put(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone(), id.clone());
        let mut orderReviewHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(whereHashMap.clone());
        if orderReviewHashMap.is_some() {
            let mut orderHistory: Option<Rc<RefCell<OrderHistory>>> = OrderHistory::new2(Basket::new(), orderReviewHashMap.clone());
            return orderHistory.clone();
        } else {
            return None;
        }
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut entryData: Option<Rc<RefCell<EntryData>>> = EntryData::getInstance();
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(String::from("CREATE TABLE "));
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(String::from(" ("));
        jcall!(stringBuffer).append(jcall!(entryData).ID.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_ID.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(StoreFrontData::getInstance()).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPEDDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_ORDERDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TRANSDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_CANCELDATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_STATUS.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_TYPE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_EXPIRATION.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_NUMBER.with(Clone::clone).clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(jcall!(entryData).ENCRYPTION.clone());
        jcall!(stringBuffer).append(String::from(" BIGINT(19) UNSIGNED ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STREET.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CITY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_STATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_CODE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::BillingAddressData::BillingAddressData_COUNTRY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STREET.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CITY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_STATE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_CODE.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::address::ShippingAddressData::ShippingAddressData_COUNTRY.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::shipping::ShippingMethodData::ShippingMethodData_NAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SUBTOTAL.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_SHIPPINGCOST.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TAX.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderHistoryData::OrderHistoryData_TOTAL.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(entryData).SPECIAL.clone());
        jcall!(stringBuffer).append(String::from(" VARCHAR(255) ,"));
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCOMMENT.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_CUSTOMERCANCELCOMMENT.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECOMMENT.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::inventory::order::OrderData::OrderData_STORECANCELCOMMENT.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).BLOB_NOT_NULL.clone());
        jcall!(stringBuffer).append(String::from("PRIMARY KEY("));
        jcall!(stringBuffer).append(jcall!(entryData).ID.clone());
        jcall!(stringBuffer).append(String::from(") )"));
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
    

    // throws Exception
    pub fn update(&mut self, mut whereHashMap: Option<Rc<RefCell<HashMap>>>, mut orderHashMap: Option<Rc<RefCell<HashMap>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.updateWhere(whereHashMap.clone(), orderHashMap.clone());
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for OrderHistoryEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for OrderHistoryEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for OrderHistoryEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for OrderHistoryEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for OrderHistoryEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for OrderHistoryEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
    fn insert(&mut self, mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>>) { Self::insert(self, values) }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for OrderHistoryEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


