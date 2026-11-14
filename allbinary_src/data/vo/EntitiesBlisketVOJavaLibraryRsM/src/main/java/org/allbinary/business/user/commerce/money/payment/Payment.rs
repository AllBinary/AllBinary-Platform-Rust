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
// package org.allbinary.business.user.commerce.money.payment
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::java::util::HashMap::HashMap;
// use crate::javax::servlet::ServletRequest::ServletRequest;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::control::crypt::SuperCrypt::SuperCrypt;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::PaymentInterface::PaymentInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct Payment {
    pub name: String,
    pub expiration: String,
    pub encryption: String,
    pub number: String,
    pub id: String,
    pub r#type: String,
    pub tenderType: String,
    pub transactionType: String,
    pub aba: String,
    pub account: String,
    pub accountType: String,
    pub authorizationCode: String,
    pub checkNumber: String,
    pub checkType: String,
    pub driversLicense: String,
    pub magneticInkCheckReader: String,
    pub isDefault: bool,
}

impl PaymentInterface for Payment {
    fn setName(&mut self, mut value: String) { Self::setName(self, value) }
    fn setType(&mut self, mut value: String) { Self::setType(self, value) }
    fn setExpiration(&mut self, mut value: String) { Self::setExpiration(self, value) }
    fn setNumber(&mut self, mut value: String) { Self::setNumber(self, value) }
    fn getId(&mut self) -> String { Self::getId(self) }
    fn getName(&mut self) -> String { Self::getName(self) }
    fn getType(&mut self) -> String { Self::getType(self) }
    fn getExpiration(&mut self) -> String { Self::getExpiration(self) }
    fn getEncryption(&mut self) -> String { Self::getEncryption(self) }
    fn getNumber(&mut self) -> String { Self::getNumber(self) }
    fn getLastFour(&mut self) -> String { Self::getLastFour(self) }
    fn isDefault(&mut self) -> bool { Self::isDefault(self) }
}
impl DomNodeInterface for Payment {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl Payment {

    pub fn toString(&mut self) -> String {
        String::from(std::any::type_name::<Self>())
    }
    

    pub fn new_struct1(mut payment: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            expiration: String::new(),
            encryption: String::new(),
            number: String::new(),
            id: String::new(),
            r#type: String::new(),
            tenderType: String::new(),
            transactionType: String::new(),
            aba: String::new(),
            account: String::new(),
            accountType: String::new(),
            authorizationCode: String::new(),
            checkNumber: String::new(),
            checkType: String::new(),
            driversLicense: String::new(),
            magneticInkCheckReader: String::new(),
            isDefault: false,
        };
        this.isDefault = false;
        this.name = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_NAME.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.r#type = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_TYPE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.expiration = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_EXPIRATION.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.encryption = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).ENCRYPTION.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.number = { let java_value = ((jcall!(SuperCrypt::new(((Some((this.encryption.clone()).trim().parse::<i32>().expect("NumberFormatException"))).unwrap() as i32))).decrypt({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_NUMBER.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() })).clone() /* as String */).clone(); java_value };
        this.id = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_ID.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        let mut def: String = { let java_object = (jcall!(payment).get(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).DEFAULT.clone(); java_field }.clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if true && (def.cmp(&{ let java_field = jcall!(EntryData::getInstance()).DEFAULT.clone(); java_field }.clone()) as i32) == 0 {
            this.isDefault = true;
        } else {
            this.isDefault = false;
        }
        this.tenderType = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_TENDERTYPE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.transactionType = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_TRANSACTIONTYPE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.aba = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_ABA.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.account = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_ACCOUNT.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.accountType = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_ACCOUNTTYPE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.authorizationCode = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_AUTHORIZATIONCODE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.checkNumber = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_CHECKNUMBER.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.checkType = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_CHECKTYPE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.driversLicense = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_DRIVERSLICENSE.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this.magneticInkCheckReader = { let java_value = ({ let java_object = (jcall!(payment).get(crate::java_into_object(PaymentData_MAGNETICINKCHECKREADER.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() }).clone(); java_value };
        this
    }

    pub fn new1(mut payment: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(payment))))
    }
    

    pub fn new_struct1_ServletRequest(mut request: Option<Rc<RefCell<ServletRequest>>>) -> Self {
        let mut this = Self {
            name: String::new(),
            expiration: String::new(),
            encryption: String::new(),
            number: String::new(),
            id: String::new(),
            r#type: String::new(),
            tenderType: String::new(),
            transactionType: String::new(),
            aba: String::new(),
            account: String::new(),
            accountType: String::new(),
            authorizationCode: String::new(),
            checkNumber: String::new(),
            checkType: String::new(),
            driversLicense: String::new(),
            magneticInkCheckReader: String::new(),
            isDefault: false,
        };
        this.isDefault = false;
        this.name = { let java_value = (jcall!(request).getParameter(PaymentData_NAME.with(Clone::clone).clone())).clone(); java_value };
        this.r#type = { let java_value = (jcall!(request).getParameter(PaymentData_TYPE.with(Clone::clone).clone())).clone(); java_value };
        this.expiration = { let java_value = (jcall!(request).getParameter(PaymentData_EXPIRATION.with(Clone::clone).clone())).clone(); java_value };
        this.number = { let java_value = (jcall!(request).getParameter(PaymentData_NUMBER.with(Clone::clone).clone())).clone(); java_value };
        this.tenderType = { let java_value = (jcall!(request).getParameter(PaymentData_TENDERTYPE.with(Clone::clone).clone())).clone(); java_value };
        this.transactionType = { let java_value = (jcall!(request).getParameter(PaymentData_TRANSACTIONTYPE.with(Clone::clone).clone())).clone(); java_value };
        this.aba = { let java_value = (jcall!(request).getParameter(PaymentData_ABA.with(Clone::clone).clone())).clone(); java_value };
        this.account = { let java_value = (jcall!(request).getParameter(PaymentData_ACCOUNT.with(Clone::clone).clone())).clone(); java_value };
        this.accountType = { let java_value = (jcall!(request).getParameter(PaymentData_ACCOUNTTYPE.with(Clone::clone).clone())).clone(); java_value };
        this.authorizationCode = { let java_value = (jcall!(request).getParameter(PaymentData_AUTHORIZATIONCODE.with(Clone::clone).clone())).clone(); java_value };
        this.checkNumber = { let java_value = (jcall!(request).getParameter(PaymentData_CHECKNUMBER.with(Clone::clone).clone())).clone(); java_value };
        this.checkType = { let java_value = (jcall!(request).getParameter(PaymentData_CHECKTYPE.with(Clone::clone).clone())).clone(); java_value };
        this.driversLicense = { let java_value = (jcall!(request).getParameter(PaymentData_DRIVERSLICENSE.with(Clone::clone).clone())).clone(); java_value };
        this.magneticInkCheckReader = { let java_value = (jcall!(request).getParameter(PaymentData_MAGNETICINKCHECKREADER.with(Clone::clone).clone())).clone(); java_value };
        this
    }

    pub fn new1_ServletRequest(mut request: Option<Rc<RefCell<ServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1_ServletRequest(request))))
    }
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            name: String::new(),
            expiration: String::new(),
            encryption: String::new(),
            number: String::new(),
            id: String::new(),
            r#type: String::new(),
            tenderType: String::new(),
            transactionType: String::new(),
            aba: String::new(),
            account: String::new(),
            accountType: String::new(),
            authorizationCode: String::new(),
            checkNumber: String::new(),
            checkType: String::new(),
            driversLicense: String::new(),
            magneticInkCheckReader: String::new(),
            isDefault: false,
        };
        this.isDefault = false;
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut aFalse: Option<bool> = false;
        if false || (self.name.cmp(&{ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()) as i32) == 0 {
            return aFalse.clone();
        }
        if false || (self.r#type.cmp(&{ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()) as i32) == 0 {
            return aFalse.clone();
        }
        if false || (self.expiration.cmp(&{ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()) as i32) == 0 {
            return aFalse.clone();
        }
        if false || (self.number.cmp(&{ let java_field = jcall!(stringUtil).EMPTY_STRING.clone(); java_field }.clone()) as i32) == 0 {
            return aFalse.clone();
        }
        if (self.number.chars().count() as i32) > { let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).business.clone(); java_field }).user.clone(); java_field }).commerce.clone(); java_field }).money.clone(); java_field }).payment.clone(); java_field }).types.clone(); java_field }).CreditCardType.clone(); java_field }).MAXLEN.clone(); java_field } {
            return aFalse.clone();
        }
        return true;
    }
    

    pub fn setTenderType(&mut self, mut value: String) {
        self.tenderType = value.clone();
    }
    

    pub fn setTransactionType(&mut self, mut value: String) {
        self.transactionType = value.clone();
    }
    

    pub fn setAuthorizationCode(&mut self, mut value: String) {
        self.authorizationCode = value.clone();
    }
    

    pub fn setCheckType(&mut self, mut value: String) {
        self.checkType = value.clone();
    }
    

    pub fn setCheckNumber(&mut self, mut value: String) {
        self.checkNumber = value.clone();
    }
    

    pub fn setDriversLicense(&mut self, mut value: String) {
        self.driversLicense = value.clone();
    }
    

    pub fn setMagneticInkCheckReader(&mut self, mut value: String) {
        self.magneticInkCheckReader = value.clone();
    }
    

    pub fn setAba(&mut self, mut value: String) {
        self.aba = value.clone();
    }
    

    pub fn setAccountType(&mut self, mut value: String) {
        self.accountType = value.clone();
    }
    

    pub fn setName(&mut self, mut value: String) {
        self.name = value.clone();
    }
    

    pub fn setType(&mut self, mut value: String) {
        self.r#type = value.clone();
    }
    

    pub fn setExpiration(&mut self, mut value: String) {
        self.expiration = value.clone();
    }
    

    pub fn setNumber(&mut self, mut value: String) {
        self.number = value.clone();
    }
    

    pub fn getId(&mut self) -> String {
        return self.id.clone();
    }
    

    pub fn getName(&mut self) -> String {
        return self.name.clone();
    }
    

    pub fn getType(&mut self) -> String {
        return self.r#type.clone();
    }
    

    pub fn getExpiration(&mut self) -> String {
        return self.expiration.clone();
    }
    

    pub fn getEncryption(&mut self) -> String {
        return self.encryption.clone();
    }
    

    pub fn getNumber(&mut self) -> String {
        return self.number.clone();
    }
    

    pub fn getTenderType(&mut self) -> String {
        return self.tenderType.clone();
    }
    

    pub fn getTransactionType(&mut self) -> String {
        return self.transactionType.clone();
    }
    

    pub fn getAba(&mut self) -> String {
        return self.aba.clone();
    }
    

    pub fn getAccount(&mut self) -> String {
        return self.account.clone();
    }
    

    pub fn getAccountType(&mut self) -> String {
        return self.accountType.clone();
    }
    

    pub fn getAuthorizationCode(&mut self) -> String {
        return self.authorizationCode.clone();
    }
    

    pub fn getCheckNumber(&mut self) -> String {
        return self.checkNumber.clone();
    }
    

    pub fn getCheckType(&mut self) -> String {
        return self.checkType.clone();
    }
    

    pub fn getDriversLicense(&mut self) -> String {
        return self.driversLicense.clone();
    }
    

    pub fn getMagneticInkCheckReader(&mut self) -> String {
        return self.magneticInkCheckReader.clone();
    }
    

    pub fn getLastFour(&mut self) -> String {
        if (self.number.chars().count() as i32) > PaymentData_MIN.with(Clone::clone) && (self.number.chars().count() as i32) <= PaymentData_MAX.with(Clone::clone) {
            return self.number[((self.number.chars().count() as i32) - 4) as usize..].to_string();
        } else {
            return String::from("Invalid #");
        }
    }
    

    pub fn isDefault(&mut self) -> bool {
        return self.isDefault.clone();
    }
    

    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(hashMap).put(crate::java_into_object(PaymentData_NAME.with(Clone::clone).clone()), crate::java_into_object(self.name.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_TYPE.with(Clone::clone).clone()), crate::java_into_object(self.r#type.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_EXPIRATION.with(Clone::clone).clone()), crate::java_into_object(self.expiration.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_NUMBER.with(Clone::clone).clone()), crate::java_into_object(self.getLastFour()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_TENDERTYPE.with(Clone::clone).clone()), crate::java_into_object(self.tenderType.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_TRANSACTIONTYPE.with(Clone::clone).clone()), crate::java_into_object(self.transactionType.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_ABA.with(Clone::clone).clone()), crate::java_into_object(self.aba.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_ACCOUNT.with(Clone::clone).clone()), crate::java_into_object(self.account.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_ACCOUNTTYPE.with(Clone::clone).clone()), crate::java_into_object(self.accountType.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_AUTHORIZATIONCODE.with(Clone::clone).clone()), crate::java_into_object(self.authorizationCode.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_CHECKNUMBER.with(Clone::clone).clone()), crate::java_into_object(self.checkNumber.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_DRIVERSLICENSE.with(Clone::clone).clone()), crate::java_into_object(self.driversLicense.clone()));
        jcall!(hashMap).put(crate::java_into_object(PaymentData_MAGNETICINKCHECKREADER.with(Clone::clone).clone()), crate::java_into_object(self.magneticInkCheckReader.clone()));
        if self.isDefault {
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).DEFAULT.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).DEFAULT.clone(); java_field }.clone()));
        } else {
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(EntryData::getInstance()).DEFAULT.clone(); java_field }.clone()), crate::java_into_object({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()));
        }
        return hashMap.clone();
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.toHashMap();
        let mut node: Option<Rc<RefCell<Node>>> = ModDomHelper::createNameValueNodes_Document_String_HashMap(document.clone(), PaymentData_PAYMENT.with(Clone::clone).clone(), hashMap.clone());
        return node.clone();
    }
    

    pub fn toXmlDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn view(&mut self) -> String {
        return None;
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn validationInfo(&mut self) -> String {
        return None;
    }
}

