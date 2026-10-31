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
// package org.allbinary.data.tables.user.commerce.money.payment.gateway
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use crate::Calendar;
use std::collections::HashMap;
use super::*;
// use crate::java::util::Calendar::Calendar;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayEncryptedMapping::PaymentGatewayEncryptedMapping;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterface::PaymentGatewayInterface;
use crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayInterfaceFactory::PaymentGatewayInterfaceFactory;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentType::BasicPaymentType;
use crate::org::allbinary::business::user::commerce::money::payment::types::BasicPaymentTypeUtil::BasicPaymentTypeUtil;
use crate::org::allbinary::data::generator::PaymentGatewayIdGenerator::PaymentGatewayIdGenerator;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use super::PaymentGatewayEntityInterface::PaymentGatewayEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentGatewayEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub tableName: String,
}

impl PaymentGatewayEntityInterface for PaymentGatewayEntity {
    fn getPaymentGatewayInterface(&mut self, mut storeName: String, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> Option<Rc<RefCell<dyn PaymentGatewayInterface>>> { Self::getPaymentGatewayInterface(self, storeName, paymentType) }
    fn findPaymentTypeVectorByStore(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::findPaymentTypeVectorByStore(self, storeName) }
    fn remove(&mut self, mut storeName: String, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) { Self::remove(self, storeName, paymentType) }
    fn add(&mut self, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) { Self::add(self, paymentGatewayInterface) }
    fn update(&mut self, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) { Self::update(self, paymentGatewayInterface) }
}
impl crate::org::allbinary::data::tables::BasicDataTableInterface::BasicDataTableInterface for PaymentGatewayEntity {
    fn createTable(&mut self) -> String { Self::createTable(self) }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}

impl PaymentGatewayEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            tableName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.tableName = String::from("paymentgateways");
        // super(...) is in the struct literal above
        this.setTableName(this.tableName.clone());
        if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
            jcall!(this.logUtil).putF(jcall!(this.commonStrings).CONSTRUCTOR.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(this.commonStrings).CONSTRUCTOR.clone());
        }
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn add(&mut self, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).START.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
            }
            let mut vector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(vector).add(crate::java_into_object(jcall!(PaymentGatewayIdGenerator::new()).getNext()));
            jcall!(vector).addAllList(jcall!(PaymentGatewayEncryptedMapping::new(paymentGatewayInterface.clone())).toVector());
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            jcall!(vector).add(crate::java_into_object(time.clone()));
            jcall!(vector).add(crate::java_into_object(time.clone()));
            self.base.insert(vector.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).END.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn update(&mut self, mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            jcall!(paymentGatewayInterface).setLastModified(time.clone());
            let mut whereKeyValuePairs: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereKeyValuePairs).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), jcall!(paymentGatewayInterface).getStoreName());
            jcall!(whereKeyValuePairs).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), jcall!(paymentGatewayInterface).getName());
            if jcall!(whereKeyValuePairs).get(jcall!(EntryData::getInstance()).TIMECREATED.clone()).is_some() {
                panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Don't update TimeCreated for update"))));
            }
            let mut updateHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(PaymentGatewayEncryptedMapping::new(paymentGatewayInterface.clone())).toHashMap();
            self.base.updateWhere(whereKeyValuePairs.clone(), updateHashMap.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getPaymentGatewayInterface(&mut self, mut storeName: String, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) -> Option<Rc<RefCell<dyn PaymentGatewayInterface>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentGatewayHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut whereKeyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereKeyAndValue).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone());
            jcall!(whereKeyAndValue).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), jcall!(paymentType).getName());
            paymentGatewayHashMap = self.base.getRow(whereKeyAndValue.clone());
            if paymentGatewayHashMap.is_some() {
                let mut userName: String = (jcall!(paymentGatewayHashMap).get(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone())).clone() /* as String */;
                let mut password: String = (jcall!(paymentGatewayHashMap).get(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone())).clone() /* as String */;
                let mut special1: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special2: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special3: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special4: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special5: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special6: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special7: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special8: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString())).clone() /* as String */;
                let mut special9: String = (jcall!(paymentGatewayHashMap).get(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL9.with(Clone::clone)).toString())).clone() /* as String */;
                let mut superCrypt: Option<Rc<RefCell<SuperCrypt>>> = SuperCrypt::new(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_CRYPTNUM.with(Clone::clone).clone());
                jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), jcall!(superCrypt).decrypt(userName.clone()));
                jcall!(paymentGatewayHashMap).put(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone(), jcall!(superCrypt).decrypt(password.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special1.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special2.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special3.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special4.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special5.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special6.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special7.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special8.clone()));
                jcall!(paymentGatewayHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL9.with(Clone::clone)).toString(), jcall!(superCrypt).decrypt(special9.clone()));
                let mut paymentGatewayInterface: Option<Rc<RefCell<dyn PaymentGatewayInterface>>> = jcall!(PaymentGatewayInterfaceFactory::new()).getInstance(paymentGatewayHashMap.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPaymentGatewayInterface()"));
                }
                return paymentGatewayInterface.clone();
            } else {
                return jcall!(PaymentGatewayInterfaceFactory::new()).getInstance(paymentType.clone());
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (format!("{}{}", String::from("Command Failed for paymentType: "), jcall!(paymentType).getName()), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getPaymentGatewayInterface"), crate::java_into_object(e.clone()),); jcall!(java_receiver).put(java_arguments.0, java_arguments.1, java_arguments.2, java_arguments.3); };
            }
            return None;
            }
        }
    }
    

    pub fn findPaymentTypeVectorByStore(&mut self, mut storeName: String) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentGatewayNameVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            paymentGatewayNameVector = self.base.getColumnWhere(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone());
            if paymentGatewayNameVector.is_some() {
                let mut paymentGatewayVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
                let mut size: i32 = jcall!(paymentGatewayNameVector).size();
                for i in (0..size) {
                    let mut paymentTypeString: String = (jcall!(paymentGatewayNameVector).get(i.clone())).clone() /* as String */;
                    let mut paymentType: Option<Rc<RefCell<BasicPaymentType>>> = jcall!(BasicPaymentTypeUtil::getInstance()).get(paymentTypeString.clone());
                    jcall!(paymentGatewayVector).add(crate::java_into_object(paymentType.clone()));
                }
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findPaymentTypeVectorByStore()"));
                }
                return paymentGatewayVector.clone();
            } else {
                return BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("findPaymentTypeVectorByStore"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn remove(&mut self, mut storeName: String, mut paymentType: Option<Rc<RefCell<BasicPaymentType>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereHashMap).put(jcall!(StoreFrontData::getInstance()).NAME.clone(), storeName.clone());
            jcall!(whereHashMap).put(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString(), jcall!(paymentType).getName());
            self.base.deleteWhere(whereHashMap.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("remove"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone());
        jcall!(stringBuffer).append(self.tableName.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).START.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_ID.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_AUTO_INCREMENT_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).ENABLE.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(StoreFrontData::getInstance()).NAME.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_NAME.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::commerce::money::payment::PaymentData::PaymentData_METHOD.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_MODE.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_TESTPROTOCOL.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_TESTSERVER.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_TESTPORT.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_TESTPATH.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SERVERPROTOCOL.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SERVER.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SERVERPORT.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SERVERPATH.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_TIMEOUT.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYPROTOCOL.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYSERVER.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYPORT.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYPATH.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYUSERNAME.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYPASSWORD.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_PROXYTIMEOUT.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL1.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL2.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL3.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL4.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL5.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL6.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL7.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL8.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_SPECIAL9.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).LASTMODIFIED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(EntryData::getInstance()).TIMECREATED.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone());
        jcall!(stringBuffer).append(jcall!(crate::org::allbinary::business::user::commerce::money::payment::gateway::PaymentGatewayData::PaymentGatewayData_ID.with(Clone::clone)).toString());
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn dropTable(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for PaymentGatewayEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentGatewayEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for PaymentGatewayEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for PaymentGatewayEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for PaymentGatewayEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for PaymentGatewayEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for PaymentGatewayEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
    fn dropTable(&mut self) -> String { Self::dropTable(self) }
}


