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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
// use crate::javax::servlet::jsp::PageContext::PageContext;
use crate::org::allbinary::business::installer::Portion::Portion;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentFactory::PaymentFactory;
use crate::org::allbinary::business::user::commerce::money::payment::PaymentInterface::PaymentInterface;
use crate::org::allbinary::data::tables::user::commerce::money::payment::PaymentEntity::PaymentEntity;
use crate::org::allbinary::data::tables::user::commerce::money::payment::PaymentEntityFactory::PaymentEntityFactory;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSession::WeblisketSession;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlTableUtil::AbSqlTableUtil;
use crate::org::allbinary::logic::system::security::licensing::LicensingException::LicensingException;
use super::Table::Table;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct PaymentHelper {
    pub base: Table,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub weblisketSession: Option<Rc<RefCell<WeblisketSession>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
    pub value: String,
    pub portion: Option<Rc<RefCell<Portion>>>,
}

impl crate::admin::taghelpers::BasicTableInterface::BasicTableInterface for PaymentHelper {
    fn drop(&mut self) -> String { Self::drop(self) }
    fn create(&mut self) -> String { Self::create(self) }
    fn restore(&mut self) -> String { Self::restore(self) }
    fn backup(&mut self) -> String { Self::backup(self) }
}
impl crate::admin::taghelpers::ModifyTableInterface::ModifyTableInterface for PaymentHelper {
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}
impl crate::admin::taghelpers::TableInterface::TableInterface for PaymentHelper {
}

impl PaymentHelper {
    

    pub fn new_struct(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Self {
        let mut this = Self {
            base: Table::new_struct(),
            logUtil: None,
            weblisketSession: None,
            request: None,
            value: String::new(),
            portion: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.request = (jcall!(pageContext).getRequest()).clone() /* as HttpServletRequest */;
        this.weblisketSession = WeblisketSession::new2(hashMap.clone(), pageContext.clone());
        this.portion = Portion::new(hashMap.clone());
        this.getFormData();
        this
    }

    pub fn new(mut hashMap: Option<Rc<RefCell<HashMap>>>, mut pageContext: Option<Rc<RefCell<PageContext>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(hashMap, pageContext))))
    }
    

    pub fn getFormData(&mut self) {
        self.value = jcall!(self.request).getParameter(String::from("VALUE"));
    }
    

    pub fn insert(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successfully Added Payment");
            let mut paymentEntity: Option<Rc<RefCell<PaymentEntity>>> = jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance();
            let mut paymentInterface: Option<Rc<RefCell<dyn PaymentInterface>>> = PaymentFactory::getInstance(self.request.clone());
            jcall!(paymentEntity).add(jcall!(self.weblisketSession).getUserName(), paymentInterface.clone());
            jcall!(paymentEntity).setDefault(jcall!(self.weblisketSession).getUserName(), Some((jcall!(paymentEntity).getLastId(jcall!(self.weblisketSession).getUserName())) as i32));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addPayment()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to add Payment");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("addPayment()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn select(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successfully Selected Payment");
            let mut paymentEntity: Option<Rc<RefCell<PaymentEntity>>> = jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance();
            jcall!(paymentEntity).setDefault(jcall!(self.weblisketSession).getUserName(), Some((self.value.clone()) as i32));
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selectPayment()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to select Payment");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("selectPayment()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn delete(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Successfully Removed Payment");
            let mut paymentEntity: Option<Rc<RefCell<PaymentEntity>>> = jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance();
            jcall!(paymentEntity).remove(jcall!(self.weblisketSession).getUserName(), Some((self.value.clone()) as i32));
            let mut paymentInterface: Option<Rc<RefCell<dyn PaymentInterface>>> = jcall!(paymentEntity).getDefault(jcall!(self.weblisketSession).getUserName());
            if paymentInterface.is_none() {
                jcall!(paymentEntity).setDefault(jcall!(self.weblisketSession).getUserName(), Some((jcall!(paymentEntity).getLastId(jcall!(self.weblisketSession).getUserName())) as i32));
            }
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removePayment()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to remove Payment");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("removePayment()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn drop(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentEntity: Option<Rc<RefCell<PaymentEntity>>> = jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance();
            let mut success: String = jcall!(paymentEntity).dropTable();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).DROP.clone());
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to drop Admin table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).DROP.clone(), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn create(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut paymentEntity: Option<Rc<RefCell<PaymentEntity>>> = jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance();
            let mut success: String = jcall!(paymentEntity).createTable();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"));
            }
            return success.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to create user table");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("create()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn restore(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Restore Successful");
            let mut result: String = jcall!(AbSqlTableUtil::getInstance()).restoreTable(jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance(), self.portion.clone());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to restore backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("restore()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    pub fn backup(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut success: String = String::from("Backup Successful");
            let mut result: String = jcall!(AbSqlTableUtil::getInstance()).backupTable(jcall!(PaymentEntityFactory::getInstance()).getPaymentEntityInstance());
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGS.clone()) {
                jcall!(self.logUtil).putF(success.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"));
            }
            return result.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to make backup");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("backup()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
    

    // throws LicensingException
    pub fn update(&mut self) -> String {
        return jcall!(self.base.commonStrings).NOT_IMPLEMENTED.clone();
    }
}
impl std::ops::Deref for PaymentHelper {
    type Target = Table;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for PaymentHelper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::admin::taghelpers::Table::TableDyn for PaymentHelper {
    fn as_Table(&self) -> &crate::admin::taghelpers::Table::Table { &self.base }
    fn as_Table_mut(&mut self) -> &mut crate::admin::taghelpers::Table::Table { &mut self.base }
    fn drop(&mut self) -> String { Self::drop(self) }
    fn create(&mut self) -> String { Self::create(self) }
    fn restore(&mut self) -> String { Self::restore(self) }
    fn backup(&mut self) -> String { Self::backup(self) }
    fn insert(&mut self) -> String { Self::insert(self) }
    fn delete(&mut self) -> String { Self::delete(self) }
    fn update(&mut self) -> String { Self::update(self) }
}


