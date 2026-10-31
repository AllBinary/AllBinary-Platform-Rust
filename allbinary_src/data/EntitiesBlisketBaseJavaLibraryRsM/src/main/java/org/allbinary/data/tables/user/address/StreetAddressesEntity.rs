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
// package org.allbinary.data.tables.user.address
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
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::init::db::UserDbInitInfo::UserDbInitInfo;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::address::StreetAddress::StreetAddress;
use crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData;
use crate::org::allbinary::data::tables::TableDataFactory::TableDataFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use super::StreetAddressesEntityInterface::StreetAddressesEntityInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct StreetAddressesEntity {
    pub base: AbSqlBean,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub userName: String,
}

// impl StreetAddressesEntityInterface for StreetAddressesEntity with the methods of the trait

impl StreetAddressesEntity {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            userName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut userName: String) -> Self {
        let mut this = Self {
            base: AbSqlBean::new_struct(UserDbInitInfo::new()),
            logUtil: None,
            userName: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.userName = userName.clone();
        this
    }

    pub fn new1(mut userName: String) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userName))))
    }
    

    pub fn remove(&mut self, mut index: Option<i32>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut whereHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            jcall!(whereHashMap).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), ((index).expect("NullPointerException").to_string()).clone() /* as String */);
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
    

    pub fn add(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>) {
        { let java_arguments = (address.clone(), jcall!(TableDataFactory::getInstance()).INTEGER_MAX_VALUE_STRING.clone(),); self.add2(java_arguments.0, java_arguments.1) };
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn add2(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>, mut index: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(crate::java_into_object((index.clone()).expect("NullPointerException")));
            jcall!(values).add(crate::java_into_object(self.userName.clone()));
            jcall!(values).add(crate::java_into_object(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getName()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getStreet()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getCity()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getState()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getCode()));
            jcall!(values).add(crate::java_into_object(jcall!(address).getCountry()));
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            jcall!(values).add(crate::java_into_object(time.clone()));
            self.base.insert(values.clone());
            { let java_arguments = (self.getLastId(),); self.setDefault(java_arguments.0) };
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("add"));
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
    

    pub fn update(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut addressHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(address).toHashMap();
            let mut whereKeyValuePairs: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereKeyValuePairs).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), jcall!(address).getId());
            jcall!(whereKeyValuePairs).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            self.base.updateWhere(whereKeyValuePairs.clone(), addressHashMap.clone());
            { let java_arguments = (jcall!(address).getId(),); self.setDefault(java_arguments.0) };
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update"));
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
    

    pub fn getLastId(&mut self) -> String {
        return self.base.getLargestIntegerInColumnWhere(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
    }
    

    pub fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut streetAddressVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            let mut keyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            let mut addressList: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = self.base.getRows(keyAndValue.clone());
            let mut size: i32 = jcall!(addressList).size();
            for index in (0..size) {
                let mut addressHashMap: Option<Rc<RefCell<HashMap>>> = (jcall!(addressList).get(index.clone())).clone() /* as HashMap */;
                let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = StreetAddress::new1(addressHashMap.clone());
                if streetAddress.is_some() {
                    jcall!(streetAddressVector).add(crate::java_into_object(streetAddress.clone()));
                }
            }
            return streetAddressVector.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn get1(&mut self, mut index: Option<i32>) -> Option<Rc<RefCell<StreetAddress>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut keyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(keyAndValue).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            jcall!(keyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), (index).expect("NullPointerException").to_string());
            let mut addressHashMap: Option<Rc<RefCell<HashMap>>> = self.base.getRow(keyAndValue.clone());
            if addressHashMap.is_some() {
                return StreetAddress::new1(addressHashMap.clone());
            } else {
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).GET.clone(), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn getDefault(&mut self) -> Option<Rc<RefCell<StreetAddress>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut addressHashMap: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut updateKeyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(updateKeyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone(), crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone());
            jcall!(updateKeyAndValue).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            addressHashMap = self.base.getRow(updateKeyAndValue.clone());
            if addressHashMap.is_some() {
                let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = StreetAddress::new1(addressHashMap.clone());
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDefault"));
                }
                return streetAddress.clone();
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                    jcall!(self.logUtil).putF(String::from("No Default Address Found"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDefault"));
                }
                return None;
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getDefault"), crate::java_into_object(e.clone()));
            }
            return None;
            }
        }
    }
    

    pub fn setDefault(&mut self, mut value: String) {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut updateKeyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            let mut whereKeyAndValue: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
            jcall!(whereKeyAndValue).put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
            let mut streetAddress: Option<Rc<RefCell<StreetAddress>>> = self.getDefault();
            if streetAddress.is_some() {
                jcall!(updateKeyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
                jcall!(whereKeyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), jcall!(streetAddress).getId());
                self.base.updateWhere(whereKeyAndValue.clone(), updateKeyAndValue.clone());
            }
            jcall!(whereKeyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone(), value.clone());
            jcall!(updateKeyAndValue).put(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone(), crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone());
            self.base.updateWhere(whereKeyAndValue.clone(), updateKeyAndValue.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).putF(jcall!(self.commonStrings).SUCCESS.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setDefault"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLLOGGING.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("setShippingAddress"), crate::java_into_object(e.clone()));
            }
            }
        }
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn createTableStatement(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        jcall!(stringBuffer).append(jcall!(self.sqlStrings).CREATE_TABLE.clone()).append(self.getTableName()).append(jcall!(self.sqlStrings).START.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_AUTO_INCREMENT_NOT_NULL.clone()).append(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_DEFAULT.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_NAME.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN_NOT_NULL.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STREET.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CITY.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_STATE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_CODE.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_COUNTRY.with(Clone::clone).clone()).append(jcall!(self.sqlTypeStrings).MAX_CHAR_COLUMN.clone()).append(jcall!(EntryData::getInstance()).TIMECREATED.clone()).append(jcall!(self.sqlTypeStrings).MAX_BIG_INT_UNSIGNED_NOT_NULL.clone()).append(jcall!(self.sqlStrings).PRIMARY_KEY.clone()).append(crate::org::allbinary::business::user::address::StreetAddressData::StreetAddressData_ID.with(Clone::clone).clone()).append(jcall!(self.sqlStrings).END.clone());
        return jcall!(stringBuffer).toString();
    }
    

    pub fn createTable(&mut self) -> String {
        return self.base.createTable(self.createTableStatement());
    }
    

    pub fn drop(&mut self) -> String {
        return self.base.dropTable();
    }
}
impl std::ops::Deref for StreetAddressesEntity {
    type Target = AbSqlBean;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for StreetAddressesEntity {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait StreetAddressesEntityDyn: crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn {
    fn as_StreetAddressesEntity(&self) -> &StreetAddressesEntity;
    fn as_StreetAddressesEntity_mut(&mut self) -> &mut StreetAddressesEntity;
    fn remove(&mut self, mut index: Option<i32>) { StreetAddressesEntity::remove(self.as_StreetAddressesEntity_mut(), index) }
    fn add(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>) { StreetAddressesEntity::add(self.as_StreetAddressesEntity_mut(), address) }
    fn add2(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>, mut index: String) { StreetAddressesEntity::add2(self.as_StreetAddressesEntity_mut(), address, index) }
    fn update(&mut self, mut address: Option<Rc<RefCell<StreetAddress>>>) { StreetAddressesEntity::update(self.as_StreetAddressesEntity_mut(), address) }
    fn getLastId(&mut self) -> String { StreetAddressesEntity::getLastId(self.as_StreetAddressesEntity_mut()) }
    fn get(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { StreetAddressesEntity::get(self.as_StreetAddressesEntity_mut()) }
    fn get1(&mut self, mut index: Option<i32>) -> Option<Rc<RefCell<StreetAddress>>> { StreetAddressesEntity::get1(self.as_StreetAddressesEntity_mut(), index) }
    fn getDefault(&mut self) -> Option<Rc<RefCell<StreetAddress>>> { StreetAddressesEntity::getDefault(self.as_StreetAddressesEntity_mut()) }
    fn setDefault(&mut self, mut value: String) { StreetAddressesEntity::setDefault(self.as_StreetAddressesEntity_mut(), value) }
    fn getUserName(&mut self) -> String { StreetAddressesEntity::getUserName(self.as_StreetAddressesEntity_mut()) }
    fn createTableStatement(&mut self) -> String { StreetAddressesEntity::createTableStatement(self.as_StreetAddressesEntity_mut()) }
    fn createTable(&mut self) -> String { StreetAddressesEntity::createTable(self.as_StreetAddressesEntity_mut()) }
    fn drop(&mut self) -> String { StreetAddressesEntity::drop(self.as_StreetAddressesEntity_mut()) }
}
impl StreetAddressesEntityDyn for StreetAddressesEntity {
    fn as_StreetAddressesEntity(&self) -> &StreetAddressesEntity { self }
    fn as_StreetAddressesEntity_mut(&mut self) -> &mut StreetAddressesEntity { self }
}

impl crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasicDyn for StreetAddressesEntity {
    fn as_AbSqlBasic(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &self.base.base.base.base.base }
    fn as_AbSqlBasic_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBasic::AbSqlBasic { &mut self.base.base.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBeanDyn for StreetAddressesEntity {
    fn as_AbSqlBean(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &self.base }
    fn as_AbSqlBean_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlBean::AbSqlBean { &mut self.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumnDyn for StreetAddressesEntity {
    fn as_AbSqlColumn(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &self.base.base.base }
    fn as_AbSqlColumn_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlColumn::AbSqlColumn { &mut self.base.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRowDyn for StreetAddressesEntity {
    fn as_AbSqlRow(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &self.base.base }
    fn as_AbSqlRow_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlRow::AbSqlRow { &mut self.base.base }
}
impl crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTableDyn for StreetAddressesEntity {
    fn as_AbSqlTable(&self) -> &crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &self.base.base.base.base }
    fn as_AbSqlTable_mut(&mut self) -> &mut crate::org::allbinary::logic::communication::sql::AbSqlTable::AbSqlTable { &mut self.base.base.base.base }
}


