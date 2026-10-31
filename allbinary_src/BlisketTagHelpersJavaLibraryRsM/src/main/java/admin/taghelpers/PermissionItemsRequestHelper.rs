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
// package admin.taghelpers
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
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::permissions::PermissionItemsEntityFactory::PermissionItemsEntityFactory;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use super::ModifyTable::ModifyTable;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PermissionItemsRequestHelper {
    pub base: ModifyTable,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub id: String,
    pub enabled: String,
    pub number: String,
    pub who: String,
    pub what: String,
    pub r#type: String,
    pub remoteAddr: String,
    pub startTime: String,
    pub endTime: String,
    pub price: String,
    pub timeEntered: String,
    pub lastModified: String,
}


impl PermissionItemsRequestHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: ModifyTable::new_struct(),
            logUtil: None,
            basicItemData: None,
            request: None,
            id: String::new(),
            enabled: String::new(),
            number: String::new(),
            who: String::new(),
            what: String::new(),
            r#type: String::new(),
            remoteAddr: String::new(),
            startTime: String::new(),
            endTime: String::new(),
            price: String::new(),
            timeEntered: String::new(),
            lastModified: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.id = jcall!(self.request).getParameter(jcall!(self.basicItemData).ID.clone());
        self.number = jcall!(self.request).getParameter(jcall!(self.basicItemData).NUMBER.clone());
        self.enabled = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).ENABLE.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.who = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_WHO.with(Clone::clone).clone());
        self.what = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_WHAT.with(Clone::clone).clone());
        self.r#type = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_TYPE.with(Clone::clone).clone());
        self.remoteAddr = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_REMOTE_ADDR.with(Clone::clone).clone());
        self.startTime = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_START_TIME.with(Clone::clone).clone());
        self.endTime = jcall!(self.request).getParameter(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_END_TIME.with(Clone::clone).clone());
        self.price = jcall!(self.request).getParameter(jcall!(self.basicItemData).PRICE.clone());
        self.timeEntered = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).TIMECREATED.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
        self.lastModified = { let java_receiver = self.request.clone(); let java_arguments = (jcall!(EntryData::getInstance()).LASTMODIFIED.clone(),); let java_result = jcall!(java_receiver).getParameter(java_arguments.0); java_result };
    }
    

    pub fn getHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut values: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(values).put(jcall!(self.basicItemData).ID.clone(), self.id.clone());
        jcall!(values).put(jcall!(self.basicItemData).NUMBER.clone(), self.number.clone());
        jcall!(values).put(jcall!(EntryData::getInstance()).ENABLE.clone(), self.enabled.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_WHO.with(Clone::clone).clone(), self.who.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_WHAT.with(Clone::clone).clone(), self.what.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_TYPE.with(Clone::clone).clone(), self.r#type.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_REMOTE_ADDR.with(Clone::clone).clone(), self.remoteAddr.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_START_TIME.with(Clone::clone).clone(), self.startTime.clone());
        jcall!(values).put(crate::org::allbinary::business::user::commerce::inventory::item::permission::PermissionItemData::PermissionItemData_END_TIME.with(Clone::clone).clone(), self.endTime.clone());
        jcall!(values).put(jcall!(self.basicItemData).PRICE.clone(), self.price.clone());
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(values).put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        return values.clone();
    }
    

    pub fn insert(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
            let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
            let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
            jcall!(values).add(self.id.clone());
            jcall!(values).add(self.number.clone());
            jcall!(values).add(self.enabled.clone());
            jcall!(values).add(self.who.clone());
            jcall!(values).add(self.what.clone());
            jcall!(values).add(self.r#type.clone());
            jcall!(values).add(self.remoteAddr.clone());
            jcall!(values).add(self.startTime.clone());
            jcall!(values).add(self.endTime.clone());
            jcall!(values).add(self.price.clone());
            jcall!(values).add(time.clone());
            jcall!(values).add(time.clone());
            jcall!(jcall!(PermissionItemsEntityFactory::getInstance()).getPermissionItemsEntityInstance()).insert(values.clone());
            let mut success: String = format!("{}{}", format!("{}{}", String::from("Successfully inserted "), self.id), String::from(" into items table"));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("insert()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = format!("{}{}", format!("{}{}", String::from("Failed to insert "), self.id), String::from(" into items table"));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("inserts()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn delete(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            jcall!(jcall!(PermissionItemsEntityFactory::getInstance()).getPermissionItemsEntityInstance()).delete(self.id.clone());
            let mut success: String = String::from("Successfully deleted");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to delete");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("delete()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn update(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Update Pricing Successful");
            let mut values: Option<Rc<RefCell<HashMap>>> = self.getHashMap();
            jcall!(jcall!(PermissionItemsEntityFactory::getInstance()).getPermissionItemsEntityInstance()).update(values.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(StringMaker::new()).append(self.id.clone()).append(jcall!(CommonSeps::getInstance()).SPACE.clone()).append(success.clone()).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = format!("{}{}", String::from("Failed to update: "), self.id);
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("update()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for PermissionItemsRequestHelper {
    type Target = ModifyTable;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PermissionItemsRequestHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::ModifyTable::ModifyTableDyn for PermissionItemsRequestHelper {
    fn as_ModifyTable(&self) -> &crate::admin::taghelpers::ModifyTable::ModifyTable { &self.base }
    fn as_ModifyTable_mut(&mut self) -> &mut crate::admin::taghelpers::ModifyTable::ModifyTable { &mut self.base }
}
impl crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface for PermissionItemsRequestHelper {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}


