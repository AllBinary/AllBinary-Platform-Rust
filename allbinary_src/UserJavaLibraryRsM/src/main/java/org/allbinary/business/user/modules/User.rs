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
// package org.allbinary.business.user.modules
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
use crate::org::allbinary::business::context::modules::storefront::StoreFrontData::StoreFrontData;
use crate::org::allbinary::business::entry::EntryData::EntryData;
use crate::org::allbinary::business::user::UserData::UserData;
use crate::org::allbinary::business::user::UserInterface::UserInterface;
use crate::org::allbinary::business::user::modules::configuration::UserConfigurationDomDocumentMapping::UserConfigurationDomDocumentMapping;
use crate::org::allbinary::business::user::modules::configuration::UserConfigurationInterface::UserConfigurationInterface;
use crate::org::allbinary::business::user::modules::configuration::UserConfigurationInterfaceFactory::UserConfigurationInterfaceFactory;
use crate::org::allbinary::business::user::password::Password::Password;
use crate::org::allbinary::business::user::role::UserRole::UserRole;
use crate::org::allbinary::business::user::role::UserRoleB::UserRoleB;
use crate::org::allbinary::business::user::role::UserRoleData::UserRoleData;
use crate::org::allbinary::business::user::username::UserName::UserName;
use crate::org::allbinary::logic::StdUtil::StdUtil;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData;
use crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionInterface::WeblisketSessionInterface;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct User {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub userName: String,
    pub prefixName: String,
    pub firstName: String,
    pub lastName: String,
    pub middleName: String,
    pub suffixName: String,
    pub company: String,
    pub positionAtCompany: String,
    pub mainEmail: String,
    pub secondaryEmail: String,
    pub homePhone: String,
    pub cellPhone: String,
    pub workPhone: String,
    pub otherContact: String,
    pub electronicDevice: String,
    pub fax: String,
    pub role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>,
    pub encryption: String,
    pub secret: String,
    pub password: Option<Rc<RefCell<Password>>>,
    pub enable: String,
    pub permissions: String,
    pub userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>,
}

impl UserInterface for User {
    fn setUserName(&mut self, mut value: String) { Self::setUserName(self, value) }
    fn setPrefixName(&mut self, mut value: String) { Self::setPrefixName(self, value) }
    fn setFirstName(&mut self, mut value: String) { Self::setFirstName(self, value) }
    fn setLastName(&mut self, mut value: String) { Self::setLastName(self, value) }
    fn setMiddleName(&mut self, mut value: String) { Self::setMiddleName(self, value) }
    fn setSuffixName(&mut self, mut value: String) { Self::setSuffixName(self, value) }
    fn setCompany(&mut self, mut value: String) { Self::setCompany(self, value) }
    fn setPositionAtCompany(&mut self, mut value: String) { Self::setPositionAtCompany(self, value) }
    fn setMainEmail(&mut self, mut value: String) { Self::setMainEmail(self, value) }
    fn setSecondaryEmail(&mut self, mut value: String) { Self::setSecondaryEmail(self, value) }
    fn setHomePhone(&mut self, mut value: String) { Self::setHomePhone(self, value) }
    fn setCellPhone(&mut self, mut value: String) { Self::setCellPhone(self, value) }
    fn setWorkPhone(&mut self, mut value: String) { Self::setWorkPhone(self, value) }
    fn setOtherContact(&mut self, mut value: String) { Self::setOtherContact(self, value) }
    fn setElectronicContact(&mut self, mut value: String) { Self::setElectronicContact(self, value) }
    fn setFax(&mut self, mut value: String) { Self::setFax(self, value) }
    fn setRole(&mut self, mut role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) { Self::setRole(self, role) }
    fn setEncryption(&mut self, mut value: String) { Self::setEncryption(self, value) }
    fn setSecret(&mut self, mut value: String) { Self::setSecret(self, value) }
    fn setPassword(&mut self, mut value: String) { Self::setPassword(self, value) }
    fn setPermissions(&mut self, mut value: String) { Self::setPermissions(self, value) }
    fn enable(&mut self) { Self::enable(self) }
    fn setEnable(&mut self, mut enable: String) { Self::setEnable(self, enable) }
    fn getUserName(&mut self) -> String { Self::getUserName(self) }
    fn getPrefixName(&mut self) -> String { Self::getPrefixName(self) }
    fn getFirstName(&mut self) -> String { Self::getFirstName(self) }
    fn getLastName(&mut self) -> String { Self::getLastName(self) }
    fn getMiddleName(&mut self) -> String { Self::getMiddleName(self) }
    fn getSuffixName(&mut self) -> String { Self::getSuffixName(self) }
    fn getCompany(&mut self) -> String { Self::getCompany(self) }
    fn getPositionAtCompany(&mut self) -> String { Self::getPositionAtCompany(self) }
    fn getMainEmail(&mut self) -> String { Self::getMainEmail(self) }
    fn getSecondaryEmail(&mut self) -> String { Self::getSecondaryEmail(self) }
    fn getHomePhone(&mut self) -> String { Self::getHomePhone(self) }
    fn getCellPhone(&mut self) -> String { Self::getCellPhone(self) }
    fn getWorkPhone(&mut self) -> String { Self::getWorkPhone(self) }
    fn getOtherContact(&mut self) -> String { Self::getOtherContact(self) }
    fn getElectronicContact(&mut self) -> String { Self::getElectronicContact(self) }
    fn getFax(&mut self) -> String { Self::getFax(self) }
    fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> { Self::getRole(self) }
    fn getEncryption(&mut self) -> String { Self::getEncryption(self) }
    fn getSecret(&mut self) -> String { Self::getSecret(self) }
    fn getPassword(&mut self) -> String { Self::getPassword(self) }
    fn getPermissions(&mut self) -> String { Self::getPermissions(self) }
    fn getUserConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> { Self::getUserConfigurationInterface(self) }
    fn setUserConfigurationInterface(&mut self, mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>) { Self::setUserConfigurationInterface(self, userConfigurationInterface) }
    fn toPasswordHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toPasswordHashMap(self) }
    fn validateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { Self::validateSession(self, weblisketSession) }
    fn updateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { Self::updateSession(self, weblisketSession) }
    fn isSessionValid(&mut self) -> Option<bool> { Self::isSessionValid(self) }
}
impl crate::org::allbinary::data::tables::TableMappingInterface::TableMappingInterface for User {
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { Self::getKey(self) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { Self::toVector(self) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { Self::toHashMap(self) }
}
impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for User {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl User {
    

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            userName: String::new(),
            prefixName: String::new(),
            firstName: String::new(),
            lastName: String::new(),
            middleName: String::new(),
            suffixName: String::new(),
            company: String::new(),
            positionAtCompany: String::new(),
            mainEmail: String::new(),
            secondaryEmail: String::new(),
            homePhone: String::new(),
            cellPhone: String::new(),
            workPhone: String::new(),
            otherContact: String::new(),
            electronicDevice: String::new(),
            fax: String::new(),
            role: None,
            encryption: String::new(),
            secret: String::new(),
            password: None,
            enable: String::new(),
            permissions: String::new(),
            userConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.userConfigurationInterface = UserConfigurationInterfaceFactory::getInstance().map(|reference| reference as Rc<RefCell<dyn UserConfigurationInterface>>);
        this.password = Password::new(jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn new_struct1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            userName: String::new(),
            prefixName: String::new(),
            firstName: String::new(),
            lastName: String::new(),
            middleName: String::new(),
            suffixName: String::new(),
            company: String::new(),
            positionAtCompany: String::new(),
            mainEmail: String::new(),
            secondaryEmail: String::new(),
            homePhone: String::new(),
            cellPhone: String::new(),
            workPhone: String::new(),
            otherContact: String::new(),
            electronicDevice: String::new(),
            fax: String::new(),
            role: None,
            encryption: String::new(),
            secret: String::new(),
            password: None,
            enable: String::new(),
            permissions: String::new(),
            userConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        { let java_arguments = (jcall!(RequestParams::new1(request.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap(),); this.getFormData(java_arguments.0) };
        this.userConfigurationInterface = UserConfigurationInterfaceFactory::getInstance1(this.getRole());
        this
    }

    pub fn new1(mut request: Option<Rc<RefCell<HttpServletRequest>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(request))))
    }
    

    pub fn new_struct1(mut userHashMap: Option<Rc<RefCell<HashMap>>>) -> Self {
        let mut this = Self {
            logUtil: None,
            userName: String::new(),
            prefixName: String::new(),
            firstName: String::new(),
            lastName: String::new(),
            middleName: String::new(),
            suffixName: String::new(),
            company: String::new(),
            positionAtCompany: String::new(),
            mainEmail: String::new(),
            secondaryEmail: String::new(),
            homePhone: String::new(),
            cellPhone: String::new(),
            workPhone: String::new(),
            otherContact: String::new(),
            electronicDevice: String::new(),
            fax: String::new(),
            role: None,
            encryption: String::new(),
            secret: String::new(),
            password: None,
            enable: String::new(),
            permissions: String::new(),
            userConfigurationInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.getFormData(userHashMap.clone());
        this.userConfigurationInterface = UserConfigurationInterfaceFactory::getInstance1(this.getRole());
        this
    }

    pub fn new1(mut userHashMap: Option<Rc<RefCell<HashMap>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct1(userHashMap))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self, mut userHashMap: Option<Rc<RefCell<HashMap>>>) {
        self.userName = jcall!(UserName::new1(userHashMap.clone())).get();
        let mut stringUtil: Option<Rc<RefCell<StringUtil>>> = StringUtil::getInstance();
        let mut passwordString: String = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_PASSWORD.with(Clone::clone).clone())).clone() /* as String */);
        let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
        if jcall!(stringValidationUtil).isEmpty(self.userName.clone()) && jcall!(stringValidationUtil).isEmpty(passwordString.clone()) {
            self.userName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEUSERNAME.with(Clone::clone).clone())).clone() /* as String */);
            passwordString = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::logic::communication::http::request::session::WeblisketSessionData::WeblisketSessionData_REMOVABLEPASSWORD.with(Clone::clone).clone())).clone() /* as String */);
        }
        let mut encryption: String = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(jcall!(EntryData::getInstance()).ENCRYPTION.clone())).clone() /* as String */);
        let mut secret: String = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_SECRET.with(Clone::clone).clone())).clone() /* as String */);
        self.password = Password::new(passwordString.clone());
        self.prefixName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_PREFIXNAME.with(Clone::clone).clone())).clone() /* as String */);
        self.firstName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_FIRSTNAME.with(Clone::clone).clone())).clone() /* as String */);
        self.lastName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_LASTNAME.with(Clone::clone).clone())).clone() /* as String */);
        self.middleName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_MIDDLENAME.with(Clone::clone).clone())).clone() /* as String */);
        self.suffixName = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_SUFFIXNAME.with(Clone::clone).clone())).clone() /* as String */);
        self.company = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_COMPANY.with(Clone::clone).clone())).clone() /* as String */);
        self.positionAtCompany = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_POSITIONATCOMPANY.with(Clone::clone).clone())).clone() /* as String */);
        self.mainEmail = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_MAINEMAIL.with(Clone::clone).clone())).clone() /* as String */);
        self.secondaryEmail = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_SECONDARYEMAIL.with(Clone::clone).clone())).clone() /* as String */);
        self.homePhone = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_HOMEPHONE.with(Clone::clone).clone())).clone() /* as String */);
        self.cellPhone = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_CELLPHONE.with(Clone::clone).clone())).clone() /* as String */);
        self.workPhone = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_WORKPHONE.with(Clone::clone).clone())).clone() /* as String */);
        self.otherContact = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_OTHERCONTACT.with(Clone::clone).clone())).clone() /* as String */);
        self.electronicDevice = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_ELECTRONICDEVICE.with(Clone::clone).clone())).clone() /* as String */);
        self.fax = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_FAX.with(Clone::clone).clone())).clone() /* as String */);
        self.role = UserRoleB::getRole((jcall!(userHashMap).get(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone())).clone() /* as String */);
        self.permissions = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(crate::org::allbinary::business::user::UserData::UserData_PERMISSIONS.with(Clone::clone).clone())).clone() /* as String */);
        if !jcall!(stringValidationUtil).isEmpty(self.permissions.clone()) && (self.permissions.cmp(&jcall!(StoreFrontData::getInstance()).NAME.clone()) as i32) == 0 {
            self.permissions = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(jcall!(StoreFrontData::getInstance()).NAME.clone())).clone() /* as String */);
        } else if false {
            self.permissions = String::from("No Permissions");
        }
        self.enable = jcall!(stringUtil).getNonNull((jcall!(userHashMap).get(jcall!(EntryData::getInstance()).ENABLE.clone())).clone() /* as String */);
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut valid: Option<bool> = true;
            if !jcall!(UserName::getInstance()).isValid1(self.userName.clone()).unwrap() {
                valid = false;
            }
            if !jcall!(self.password).isValid().unwrap() {
                valid = false;
            }
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if !jcall!(stringValidationUtil).isValidRequired(self.firstName.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                valid = false;
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.lastName.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                valid = false;
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.mainEmail.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) || self.mainEmail.find(&String::from("@")).map(|index| index as i32).unwrap_or(-1) == -1 {
                valid = false;
            }
            return valid.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VALIDATIONERROR.clone()) {
                let mut commonStrings: Option<Rc<RefCell<CommonStrings>>> = CommonStrings::getInstance();
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(UserName::getValidationInfo(self.userName.clone()));
            jcall!(stringBuffer).append(jcall!(self.password).getValidationInfo());
            if !jcall!(stringValidationUtil).isValidRequired(self.firstName.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                jcall!(stringBuffer).append(String::from("Please enter a valid First Name.<br />"));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.lastName.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) {
                jcall!(stringBuffer).append(String::from("Please enter a valid Last Name.<br />"));
            }
            if !jcall!(stringValidationUtil).isValidRequired(self.mainEmail.clone(), 1, unsafe { crate::org::allbinary::business::user::UserData::UserData_MAXLEN.clone() }.clone()) || self.mainEmail.find(&String::from("@")).map(|index| index as i32).unwrap_or(-1) == -1 {
                jcall!(stringBuffer).append(String::from("Please enter a valid email address.<br />"));
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Validating Form");
            }
        }
    }
    

    pub fn setUserName(&mut self, mut value: String) {
        self.userName = value.clone();
    }
    

    pub fn setPrefixName(&mut self, mut value: String) {
        self.prefixName = value.clone();
    }
    

    pub fn setFirstName(&mut self, mut value: String) {
        self.firstName = value.clone();
    }
    

    pub fn setLastName(&mut self, mut value: String) {
        self.lastName = value.clone();
    }
    

    pub fn setMiddleName(&mut self, mut value: String) {
        self.middleName = value.clone();
    }
    

    pub fn setSuffixName(&mut self, mut value: String) {
        self.suffixName = value.clone();
    }
    

    pub fn setCompany(&mut self, mut value: String) {
        self.company = value.clone();
    }
    

    pub fn setPositionAtCompany(&mut self, mut value: String) {
        self.positionAtCompany = value.clone();
    }
    

    pub fn setMainEmail(&mut self, mut value: String) {
        self.mainEmail = value.clone();
    }
    

    pub fn setSecondaryEmail(&mut self, mut value: String) {
        self.secondaryEmail = value.clone();
    }
    

    pub fn setHomePhone(&mut self, mut value: String) {
        self.homePhone = value.clone();
    }
    

    pub fn setCellPhone(&mut self, mut value: String) {
        self.cellPhone = value.clone();
    }
    

    pub fn setWorkPhone(&mut self, mut value: String) {
        self.workPhone = value.clone();
    }
    

    pub fn setOtherContact(&mut self, mut value: String) {
        self.otherContact = value.clone();
    }
    

    pub fn setElectronicContact(&mut self, mut value: String) {
        self.electronicDevice = value.clone();
    }
    

    pub fn setFax(&mut self, mut value: String) {
        self.fax = value.clone();
    }
    

    pub fn setRole(&mut self, mut role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) {
        self.role = role.clone();
    }
    

    pub fn setPermissions(&mut self, mut value: String) {
        self.permissions = value.clone();
    }
    

    pub fn setEncryption(&mut self, mut value: String) {
        self.encryption = value.clone();
    }
    

    pub fn setSecret(&mut self, mut value: String) {
        self.secret = value.clone();
    }
    

    pub fn setPassword(&mut self, mut value: String) {
        jcall!(self.password).set(value.clone());
    }
    

    pub fn enable(&mut self) {
        self.setEnable(String::from("Yes"));
    }
    

    pub fn disable(&mut self) {
        self.setEnable(String::from("No"));
    }
    

    pub fn setEnable(&mut self, mut enable: String) {
        self.enable = enable.clone();
    }
    

    pub fn getUserName(&mut self) -> String {
        return self.userName.clone();
    }
    

    pub fn getPrefixName(&mut self) -> String {
        return self.prefixName.clone();
    }
    

    pub fn getFirstName(&mut self) -> String {
        return self.firstName.clone();
    }
    

    pub fn getLastName(&mut self) -> String {
        return self.lastName.clone();
    }
    

    pub fn getMiddleName(&mut self) -> String {
        return self.middleName.clone();
    }
    

    pub fn getSuffixName(&mut self) -> String {
        return self.suffixName.clone();
    }
    

    pub fn getCompany(&mut self) -> String {
        return self.company.clone();
    }
    

    pub fn getPositionAtCompany(&mut self) -> String {
        return self.positionAtCompany.clone();
    }
    

    pub fn getMainEmail(&mut self) -> String {
        return self.mainEmail.clone();
    }
    

    pub fn getSecondaryEmail(&mut self) -> String {
        return self.secondaryEmail.clone();
    }
    

    pub fn getHomePhone(&mut self) -> String {
        return self.homePhone.clone();
    }
    

    pub fn getCellPhone(&mut self) -> String {
        return self.cellPhone.clone();
    }
    

    pub fn getWorkPhone(&mut self) -> String {
        return self.workPhone.clone();
    }
    

    pub fn getOtherContact(&mut self) -> String {
        return self.otherContact.clone();
    }
    

    pub fn getElectronicContact(&mut self) -> String {
        return self.electronicDevice.clone();
    }
    

    pub fn getFax(&mut self) -> String {
        return self.fax.clone();
    }
    

    pub fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> {
        return self.role.clone();
    }
    

    pub fn getPermissions(&mut self) -> String {
        return self.permissions.clone();
    }
    

    pub fn getEncryption(&mut self) -> String {
        return self.encryption.clone();
    }
    

    pub fn getSecret(&mut self) -> String {
        return self.secret.clone();
    }
    

    pub fn getPassword(&mut self) -> String {
        return jcall!(self.password).get();
    }
    

    // throws Exception
    pub fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> {
        let mut values: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>);
        jcall!(values).add(crate::java_into_object(self.userName.clone()));
        jcall!(values).add(crate::java_into_object(self.prefixName.clone()));
        jcall!(values).add(crate::java_into_object(self.firstName.clone()));
        jcall!(values).add(crate::java_into_object(self.lastName.clone()));
        jcall!(values).add(crate::java_into_object(self.middleName.clone()));
        jcall!(values).add(crate::java_into_object(self.suffixName.clone()));
        jcall!(values).add(crate::java_into_object(self.company.clone()));
        jcall!(values).add(crate::java_into_object(self.positionAtCompany.clone()));
        jcall!(values).add(crate::java_into_object(self.mainEmail.clone()));
        jcall!(values).add(crate::java_into_object(self.secondaryEmail.clone()));
        jcall!(values).add(crate::java_into_object(self.homePhone.clone()));
        jcall!(values).add(crate::java_into_object(self.cellPhone.clone()));
        jcall!(values).add(crate::java_into_object(self.workPhone.clone()));
        jcall!(values).add(crate::java_into_object(self.otherContact.clone()));
        jcall!(values).add(crate::java_into_object(self.electronicDevice.clone()));
        jcall!(values).add(crate::java_into_object(self.fax.clone()));
        jcall!(values).add(crate::java_into_object(jcall!(self.getRole()).toString()));
        let mut userConfigurationDomDocumentMapping: Option<Rc<RefCell<UserConfigurationDomDocumentMapping>>> = UserConfigurationDomDocumentMapping::new(self.getUserConfigurationInterface());
        jcall!(values).add(crate::java_into_object(jcall!(userConfigurationDomDocumentMapping).toDomDocumentString()));
        jcall!(values).add(crate::java_into_object(self.permissions.clone()));
        jcall!(values).addAllList(jcall!(self.password).toVector(self.secret.clone()));
        jcall!(values).add(crate::java_into_object(self.enable.clone()));
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(values).add(crate::java_into_object(time.clone()));
        jcall!(values).add(crate::java_into_object(time.clone()));
        return values.clone();
    }
    

    // throws Exception
    pub fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        let mut values: Option<Rc<RefCell<HashMap>>> = jcall!(StdUtil::getInstance()).createHashMap();
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_USERNAME.with(Clone::clone).clone(), self.userName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_PREFIXNAME.with(Clone::clone).clone(), self.prefixName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_FIRSTNAME.with(Clone::clone).clone(), self.firstName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_LASTNAME.with(Clone::clone).clone(), self.lastName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_MIDDLENAME.with(Clone::clone).clone(), self.middleName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_SUFFIXNAME.with(Clone::clone).clone(), self.suffixName.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_COMPANY.with(Clone::clone).clone(), self.company.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_POSITIONATCOMPANY.with(Clone::clone).clone(), self.positionAtCompany.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_MAINEMAIL.with(Clone::clone).clone(), self.mainEmail.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_SECONDARYEMAIL.with(Clone::clone).clone(), self.secondaryEmail.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_HOMEPHONE.with(Clone::clone).clone(), self.homePhone.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_CELLPHONE.with(Clone::clone).clone(), self.cellPhone.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_WORKPHONE.with(Clone::clone).clone(), self.workPhone.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_OTHERCONTACT.with(Clone::clone).clone(), self.otherContact.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_ELECTRONICDEVICE.with(Clone::clone).clone(), self.electronicDevice.clone());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_FAX.with(Clone::clone).clone(), self.fax.clone());
        if self.getRole().is_some() {
            jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::role::UserRoleData::UserRoleData_NAME.with(Clone::clone).clone(), jcall!(self.getRole()).toString());
        }
        let mut userConfigurationDomDocumentMapping: Option<Rc<RefCell<UserConfigurationDomDocumentMapping>>> = UserConfigurationDomDocumentMapping::new(self.getUserConfigurationInterface());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_CONFIGURATION.with(Clone::clone).clone(), jcall!(userConfigurationDomDocumentMapping).toDomDocumentString());
        jcall!(values).as_BasicArrayList_mut().put(crate::org::allbinary::business::user::UserData::UserData_PERMISSIONS.with(Clone::clone).clone(), self.permissions.clone());
        jcall!(values).as_BasicArrayList_mut().putAll(jcall!(self.password).toHashMap(self.secret.clone()));
        jcall!(values).as_BasicArrayList_mut().put(jcall!(EntryData::getInstance()).ENABLE.clone(), self.enable.clone());
        let mut calendar: Option<Rc<RefCell<Calendar>>> = Calendar::getInstance();
        let mut time: String = jcall!(Some((jcall!(calendar).getTimeInMillis()) as i64)).toString().clone();
        jcall!(values).as_BasicArrayList_mut().put(jcall!(EntryData::getInstance()).LASTMODIFIED.clone(), time.clone());
        return values.clone();
    }
    

    pub fn toPasswordHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> {
        return jcall!(self.password).toHashMap(self.secret.clone());
    }
    

    pub fn validateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) {
        jcall!(weblisketSession).setAuthenticated();
        jcall!(weblisketSession).setRole(self.getRole());
        jcall!(weblisketSession).setUserName(self.getUserName());
    }
    

    pub fn updateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) {
    }
    

    pub fn isSessionValid(&mut self) -> Option<bool> {
        return true;
    }
    

    // throws Exception
    pub fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> {
        return (self.userName).clone() /* as Any */;
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn getUserConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> {
        return self.userConfigurationInterface.clone();
    }
    

    pub fn setUserConfigurationInterface(&mut self, mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>) {
        self.userConfigurationInterface = userConfigurationInterface.clone();
    }
}
pub trait UserDyn {
    fn as_User(&self) -> &User;
    fn as_User_mut(&mut self) -> &mut User;
    fn getFormData(&mut self, mut userHashMap: Option<Rc<RefCell<HashMap>>>) { User::getFormData(self.as_User_mut(), userHashMap) }
    fn isValid(&mut self) -> Option<bool> { User::isValid(self.as_User_mut()) }
    fn validationInfo(&mut self) -> String { User::validationInfo(self.as_User_mut()) }
    fn setUserName(&mut self, mut value: String) { User::setUserName(self.as_User_mut(), value) }
    fn setPrefixName(&mut self, mut value: String) { User::setPrefixName(self.as_User_mut(), value) }
    fn setFirstName(&mut self, mut value: String) { User::setFirstName(self.as_User_mut(), value) }
    fn setLastName(&mut self, mut value: String) { User::setLastName(self.as_User_mut(), value) }
    fn setMiddleName(&mut self, mut value: String) { User::setMiddleName(self.as_User_mut(), value) }
    fn setSuffixName(&mut self, mut value: String) { User::setSuffixName(self.as_User_mut(), value) }
    fn setCompany(&mut self, mut value: String) { User::setCompany(self.as_User_mut(), value) }
    fn setPositionAtCompany(&mut self, mut value: String) { User::setPositionAtCompany(self.as_User_mut(), value) }
    fn setMainEmail(&mut self, mut value: String) { User::setMainEmail(self.as_User_mut(), value) }
    fn setSecondaryEmail(&mut self, mut value: String) { User::setSecondaryEmail(self.as_User_mut(), value) }
    fn setHomePhone(&mut self, mut value: String) { User::setHomePhone(self.as_User_mut(), value) }
    fn setCellPhone(&mut self, mut value: String) { User::setCellPhone(self.as_User_mut(), value) }
    fn setWorkPhone(&mut self, mut value: String) { User::setWorkPhone(self.as_User_mut(), value) }
    fn setOtherContact(&mut self, mut value: String) { User::setOtherContact(self.as_User_mut(), value) }
    fn setElectronicContact(&mut self, mut value: String) { User::setElectronicContact(self.as_User_mut(), value) }
    fn setFax(&mut self, mut value: String) { User::setFax(self.as_User_mut(), value) }
    fn setRole(&mut self, mut role: Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>>) { User::setRole(self.as_User_mut(), role) }
    fn setPermissions(&mut self, mut value: String) { User::setPermissions(self.as_User_mut(), value) }
    fn setEncryption(&mut self, mut value: String) { User::setEncryption(self.as_User_mut(), value) }
    fn setSecret(&mut self, mut value: String) { User::setSecret(self.as_User_mut(), value) }
    fn setPassword(&mut self, mut value: String) { User::setPassword(self.as_User_mut(), value) }
    fn enable(&mut self) { User::enable(self.as_User_mut()) }
    fn disable(&mut self) { User::disable(self.as_User_mut()) }
    fn setEnable(&mut self, mut enable: String) { User::setEnable(self.as_User_mut(), enable) }
    fn getUserName(&mut self) -> String { User::getUserName(self.as_User_mut()) }
    fn getPrefixName(&mut self) -> String { User::getPrefixName(self.as_User_mut()) }
    fn getFirstName(&mut self) -> String { User::getFirstName(self.as_User_mut()) }
    fn getLastName(&mut self) -> String { User::getLastName(self.as_User_mut()) }
    fn getMiddleName(&mut self) -> String { User::getMiddleName(self.as_User_mut()) }
    fn getSuffixName(&mut self) -> String { User::getSuffixName(self.as_User_mut()) }
    fn getCompany(&mut self) -> String { User::getCompany(self.as_User_mut()) }
    fn getPositionAtCompany(&mut self) -> String { User::getPositionAtCompany(self.as_User_mut()) }
    fn getMainEmail(&mut self) -> String { User::getMainEmail(self.as_User_mut()) }
    fn getSecondaryEmail(&mut self) -> String { User::getSecondaryEmail(self.as_User_mut()) }
    fn getHomePhone(&mut self) -> String { User::getHomePhone(self.as_User_mut()) }
    fn getCellPhone(&mut self) -> String { User::getCellPhone(self.as_User_mut()) }
    fn getWorkPhone(&mut self) -> String { User::getWorkPhone(self.as_User_mut()) }
    fn getOtherContact(&mut self) -> String { User::getOtherContact(self.as_User_mut()) }
    fn getElectronicContact(&mut self) -> String { User::getElectronicContact(self.as_User_mut()) }
    fn getFax(&mut self) -> String { User::getFax(self.as_User_mut()) }
    fn getRole(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::business::user::role::UserRole::UserRoleDyn>>> { User::getRole(self.as_User_mut()) }
    fn getPermissions(&mut self) -> String { User::getPermissions(self.as_User_mut()) }
    fn getEncryption(&mut self) -> String { User::getEncryption(self.as_User_mut()) }
    fn getSecret(&mut self) -> String { User::getSecret(self.as_User_mut()) }
    fn getPassword(&mut self) -> String { User::getPassword(self.as_User_mut()) }
    fn toVector(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> { User::toVector(self.as_User_mut()) }
    fn toHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { User::toHashMap(self.as_User_mut()) }
    fn toPasswordHashMap(&mut self) -> Option<Rc<RefCell<HashMap>>> { User::toPasswordHashMap(self.as_User_mut()) }
    fn validateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { User::validateSession(self.as_User_mut(), weblisketSession) }
    fn updateSession(&mut self, mut weblisketSession: Option<Rc<RefCell<dyn WeblisketSessionInterface>>>) { User::updateSession(self.as_User_mut(), weblisketSession) }
    fn isSessionValid(&mut self) -> Option<bool> { User::isSessionValid(self.as_User_mut()) }
    fn getKey(&mut self) -> Option<Rc<RefCell<Box<dyn std::any::Any>>>> { User::getKey(self.as_User_mut()) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { User::toValidationInfoDoc(self.as_User_mut()) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { User::toValidationInfoNode(self.as_User_mut(), document) }
    fn getUserConfigurationInterface(&mut self) -> Option<Rc<RefCell<dyn UserConfigurationInterface>>> { User::getUserConfigurationInterface(self.as_User_mut()) }
    fn setUserConfigurationInterface(&mut self, mut userConfigurationInterface: Option<Rc<RefCell<dyn UserConfigurationInterface>>>) { User::setUserConfigurationInterface(self.as_User_mut(), userConfigurationInterface) }
}
impl UserDyn for User {
    fn as_User(&self) -> &User { self }
    fn as_User_mut(&mut self) -> &mut User { self }
}


