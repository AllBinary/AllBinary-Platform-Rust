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
// package org.allbinary.business.user.commerce.inventory.item
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData;
use crate::org::allbinary::logic::control::validate::Validation::Validation;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::ItemInterface::ItemInterface;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct BasicItemValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub MAXWEIGHT: i32,
    pub MAXPRICE: i32,
    pub itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>,
}

impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for BasicItemValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl BasicItemValidation {
    

    pub fn new_struct(mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            MAXWEIGHT: 0,
            MAXPRICE: 0,
            itemInterface: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.MAXWEIGHT = 14;
        this.MAXPRICE = 14;
        this.itemInterface = itemInterface.clone();
        this
    }

    pub fn new(mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(itemInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.itemInterface).getId().is_none() || jcall!(jcall!(self.itemInterface).getId()).length() < 1 || jcall!(jcall!(self.itemInterface).getId()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if jcall!(self.itemInterface).getId().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getId()) {
                return false;
            }
            if jcall!(self.itemInterface).getNumber().is_none() || jcall!(jcall!(self.itemInterface).getNumber()).length() < 1 || jcall!(jcall!(self.itemInterface).getNumber()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getNumber().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getNumber()) {
                return false;
            }
            if jcall!(self.itemInterface).getInBaskets().is_none() || jcall!(jcall!(self.itemInterface).getInBaskets()).length() < 1 || jcall!(jcall!(self.itemInterface).getInBaskets()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getInBaskets().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getInBaskets()) {
                return false;
            }
            if jcall!(self.itemInterface).getWeight().is_none() || jcall!(jcall!(self.itemInterface).getWeight()).length() < 1 || jcall!(jcall!(self.itemInterface).getWeight()).length() > self.MAXWEIGHT {
                return false;
            }
            if jcall!(self.itemInterface).getWeight().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getWeight()) {
                return false;
            }
            if jcall!(self.itemInterface).getEnabled().is_none() || jcall!(jcall!(self.itemInterface).getEnabled()).length() < 1 || jcall!(jcall!(self.itemInterface).getEnabled()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getNewOrUsed().is_none() || jcall!(jcall!(self.itemInterface).getNewOrUsed()).length() < 1 || jcall!(jcall!(self.itemInterface).getNewOrUsed()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getSummary().is_none() || jcall!(jcall!(self.itemInterface).getSummary()).length() < 1 || jcall!(jcall!(self.itemInterface).getSummary()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getDescription().is_none() || jcall!(jcall!(self.itemInterface).getDescription()).length() < 1 || jcall!(jcall!(self.itemInterface).getDescription()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getKeywords().is_none() || jcall!(jcall!(self.itemInterface).getKeywords()).length() < 1 || jcall!(jcall!(self.itemInterface).getKeywords()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getCategory().is_none() || jcall!(jcall!(self.itemInterface).getCategory()).length() < 1 || jcall!(jcall!(self.itemInterface).getCategory()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getType().is_none() || jcall!(jcall!(self.itemInterface).getType()).length() < 1 || jcall!(jcall!(self.itemInterface).getType()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getPrice().is_none() || !jcall!(jcall!(self.itemInterface).getPrice()).isValid() {
                return false;
            }
            if jcall!(self.itemInterface).getCustoms().is_none() || jcall!(jcall!(self.itemInterface).getCustoms()).length() < 1 || jcall!(jcall!(self.itemInterface).getCustoms()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getDownloads().is_none() || jcall!(jcall!(self.itemInterface).getDownloads()).length() < 1 || jcall!(jcall!(self.itemInterface).getDownloads()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getGroups().is_none() || jcall!(jcall!(self.itemInterface).getGroups()).length() < 1 || jcall!(jcall!(self.itemInterface).getGroups()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getOptions().is_none() || jcall!(jcall!(self.itemInterface).getOptions()).length() < 1 || jcall!(jcall!(self.itemInterface).getOptions()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getPermissions().is_none() || jcall!(jcall!(self.itemInterface).getPermissions()).length() < 1 || jcall!(jcall!(self.itemInterface).getPermissions()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.itemInterface).getSpecials().is_none() || jcall!(jcall!(self.itemInterface).getSpecials()).length() < 1 || jcall!(jcall!(self.itemInterface).getSpecials()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).SQLTAGSERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to validate form"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            if jcall!(self.itemInterface).getId().is_none() || jcall!(jcall!(self.itemInterface).getId()).length() < 1 || jcall!(jcall!(self.itemInterface).getId()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Id is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            let mut stringValidationUtil: Option<Rc<RefCell<StringValidationUtil>>> = StringValidationUtil::getInstance();
            if jcall!(self.itemInterface).getId().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getId()) {
                jcall!(stringBuffer).append(String::from("Id is invalid. Must be > 0.<br />"));
            }
            if jcall!(self.itemInterface).getNumber().is_none() || jcall!(jcall!(self.itemInterface).getNumber()).length() < 1 || jcall!(jcall!(self.itemInterface).getNumber()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Number of items is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getNumber().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getNumber()) {
                jcall!(stringBuffer).append(String::from("Number of items value is not a valid number.<br />"));
            }
            if jcall!(self.itemInterface).getInBaskets().is_none() || jcall!(jcall!(self.itemInterface).getInBaskets()).length() < 1 || jcall!(jcall!(self.itemInterface).getInBaskets()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Number in baskets is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getInBaskets().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getInBaskets()) {
                jcall!(stringBuffer).append(String::from("In Baskets value is not a valid number.<br />"));
            }
            if jcall!(self.itemInterface).getWeight().is_none() || jcall!(jcall!(self.itemInterface).getWeight()).length() < 1 || jcall!(jcall!(self.itemInterface).getWeight()).length() > self.MAXWEIGHT {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Weight value is invalid. Must be < "), self.MAXWEIGHT), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getWeight().is_some() && !jcall!(stringValidationUtil).isNumber(jcall!(self.itemInterface).getWeight()) {
                jcall!(stringBuffer).append(String::from("Weight value is not a valid number.<br />"));
            }
            if jcall!(self.itemInterface).getEnabled().is_none() || jcall!(jcall!(self.itemInterface).getEnabled()).length() < 1 || jcall!(jcall!(self.itemInterface).getEnabled()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("For Sale value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getNewOrUsed().is_none() || jcall!(jcall!(self.itemInterface).getNewOrUsed()).length() < 1 || jcall!(jcall!(self.itemInterface).getNewOrUsed()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("New Or Used value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getSummary().is_none() || jcall!(jcall!(self.itemInterface).getSummary()).length() < 1 || jcall!(jcall!(self.itemInterface).getSummary()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Summary value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getDescription().is_none() || jcall!(jcall!(self.itemInterface).getDescription()).length() < 1 || jcall!(jcall!(self.itemInterface).getDescription()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Description value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getKeywords().is_none() || jcall!(jcall!(self.itemInterface).getKeywords()).length() < 1 || jcall!(jcall!(self.itemInterface).getKeywords()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Keywords value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getCategory().is_none() || jcall!(jcall!(self.itemInterface).getCategory()).length() < 1 || jcall!(jcall!(self.itemInterface).getCategory()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Category value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getType().is_none() || jcall!(jcall!(self.itemInterface).getType()).length() < 1 || jcall!(jcall!(self.itemInterface).getType()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Type value is invalid. Must be < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone)), String::from(" and > 0 characters long.<br />")));
            }
            if jcall!(self.itemInterface).getPrice().is_none() || !jcall!(jcall!(self.itemInterface).getPrice()).isValid() {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Price value is invalid. Must be < "), self.MAXPRICE), String::from(" and > 0 in length.<br />")));
            }
            if jcall!(self.itemInterface).getCustoms().is_none() || jcall!(jcall!(self.itemInterface).getCustoms()).length() < 1 || jcall!(jcall!(self.itemInterface).getCustoms()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Custom items value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getDownloads().is_none() || jcall!(jcall!(self.itemInterface).getDownloads()).length() < 1 || jcall!(jcall!(self.itemInterface).getDownloads()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Downloadable item value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getGroups().is_none() || jcall!(jcall!(self.itemInterface).getGroups()).length() < 1 || jcall!(jcall!(self.itemInterface).getGroups()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Group item value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getOptions().is_none() || jcall!(jcall!(self.itemInterface).getOptions()).length() < 1 || jcall!(jcall!(self.itemInterface).getOptions()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Option item value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getPermissions().is_none() || jcall!(jcall!(self.itemInterface).getPermissions()).length() < 1 || jcall!(jcall!(self.itemInterface).getPermissions()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Permission item value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
            }
            if jcall!(self.itemInterface).getSpecials().is_none() || jcall!(jcall!(self.itemInterface).getSpecials()).length() < 1 || jcall!(jcall!(self.itemInterface).getSpecials()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(format!("{}{}", format!("{}{}", String::from("Special item value is invalid. Must be number < "), crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone)), String::from(" and > 0 digits.<br />")));
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
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
}
impl std::ops::Deref for BasicItemValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasicItemValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for BasicItemValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


