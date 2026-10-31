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
// package org.allbinary.business.user.commerce.inventory.item.download
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
use super::DownloadableItem::DownloadableItem;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct DownloadableItemValidation {
    pub base: Validation,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub downloadableItem: Option<Rc<RefCell<DownloadableItem>>>,
}

impl crate::org::allbinary::logic::control::validate::ValidationInterface::ValidationInterface for DownloadableItemValidation {
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}

impl DownloadableItemValidation {
    

    pub fn new_struct(mut downloadableItem: Option<Rc<RefCell<DownloadableItem>>>) -> Self {
        let mut this = Self {
            base: Validation::new_struct(),
            logUtil: None,
            downloadableItem: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.downloadableItem = downloadableItem.clone();
        this
    }

    pub fn new(mut downloadableItem: Option<Rc<RefCell<DownloadableItem>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(downloadableItem))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(self.downloadableItem).getId().is_none() || jcall!(jcall!(self.downloadableItem).getId()).length() < 1 || jcall!(jcall!(self.downloadableItem).getId()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getId().is_some() && !jcall!(StringValidationUtil::getInstance()).isNumber(jcall!(self.downloadableItem).getId()) {
                return false;
            }
            if jcall!(self.downloadableItem).getSystem().is_none() || jcall!(jcall!(self.downloadableItem).getSystem()).length() < 1 || jcall!(jcall!(self.downloadableItem).getSystem()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getPlatform().is_none() || jcall!(jcall!(self.downloadableItem).getPlatform()).length() < 1 || jcall!(jcall!(self.downloadableItem).getPlatform()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getSpecialName().is_none() || jcall!(jcall!(self.downloadableItem).getSpecialName()).length() < 0 || jcall!(jcall!(self.downloadableItem).getSpecialName()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getVersion().is_none() || jcall!(jcall!(self.downloadableItem).getVersion()).length() < 1 || jcall!(jcall!(self.downloadableItem).getVersion()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getChanges().is_none() || jcall!(jcall!(self.downloadableItem).getChanges()).length() < 1 || jcall!(jcall!(self.downloadableItem).getChanges()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                return false;
            }
            if jcall!(self.downloadableItem).getSize().is_none() {
                return false;
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getSize()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    return false;
                }
            }
            if jcall!(self.downloadableItem).getValidTime().is_none() {
                return false;
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getValidTime()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    return false;
                }
            }
            if jcall!(self.downloadableItem).getRetries().is_none() {
                return false;
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getRetries()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    return false;
                }
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
            if jcall!(self.downloadableItem).getId().is_none() || jcall!(jcall!(self.downloadableItem).getId()).length() < 1 || jcall!(jcall!(self.downloadableItem).getId()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("Id is invalid. Must be number < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and > 0 digits.<br/>"));
            }
            if jcall!(self.downloadableItem).getId().is_some() && !jcall!(StringValidationUtil::getInstance()).isNumber(jcall!(self.downloadableItem).getId()) {
                jcall!(stringBuffer).append(String::from("Id is invalid. Must be > 0.<br/>"));
            }
            if jcall!(self.downloadableItem).getSystem().is_none() || jcall!(jcall!(self.downloadableItem).getSystem()).length() < 1 || jcall!(jcall!(self.downloadableItem).getSystem()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("System value is invalid. Must be < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters long.<br/>"));
            }
            if jcall!(self.downloadableItem).getPlatform().is_none() || jcall!(jcall!(self.downloadableItem).getPlatform()).length() < 1 || jcall!(jcall!(self.downloadableItem).getPlatform()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("Platform value is invalid. Must be < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters long.<br/>"));
            }
            if jcall!(self.downloadableItem).getSpecialName().is_none() || jcall!(jcall!(self.downloadableItem).getSpecialName()).length() < 0 || jcall!(jcall!(self.downloadableItem).getSpecialName()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("Special Name value is invalid. Must be < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters long.<br/>"));
            }
            if jcall!(self.downloadableItem).getVersion().is_none() || jcall!(jcall!(self.downloadableItem).getVersion()).length() < 1 || jcall!(jcall!(self.downloadableItem).getVersion()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("Version value is invalid. Must be < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXSTRING.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and > 0 characters long.<br/>"));
            }
            if jcall!(self.downloadableItem).getChanges().is_none() || jcall!(jcall!(self.downloadableItem).getChanges()).length() < 0 || jcall!(jcall!(self.downloadableItem).getChanges()).length() > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone) {
                jcall!(stringBuffer).append(String::from("Changes value is invalid. Must be < "));
                jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXBLOB.with(Clone::clone).clone());
                jcall!(stringBuffer).append(String::from(" and >= 0 characters long.<br/>"));
            }
            if jcall!(self.downloadableItem).getSize().is_none() {
                jcall!(stringBuffer).append(String::from("File Size is null"));
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getSize()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    jcall!(stringBuffer).append(String::from("File Size is invalid. Must be number < "));
                    jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone).clone());
                    jcall!(stringBuffer).append(String::from(" and > 0 digits.<br/>"));
                }
            }
            if jcall!(self.downloadableItem).getValidTime().is_none() {
                jcall!(stringBuffer).append(String::from("Valid Time is null"));
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getValidTime()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    jcall!(stringBuffer).append(String::from("Valid Time is invalid. Must be number < "));
                    jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone).clone());
                    jcall!(stringBuffer).append(String::from(" and > 0 digits.<br/>"));
                }
            }
            if jcall!(self.downloadableItem).getRetries().is_none() {
                jcall!(stringBuffer).append(String::from("Retries is null"));
            } else {
                let mut size: String = jcall!(jcall!(self.downloadableItem).getRetries()).toString();
                if (size.chars().count() as i32) < 1 || (size.chars().count() as i32) > crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone) {
                    jcall!(stringBuffer).append(String::from("Retries is invalid. Must be number < "));
                    jcall!(stringBuffer).appendint(crate::org::allbinary::logic::communication::sql::AbSqlData::AbSqlData_MAXNUM.with(Clone::clone).clone());
                    jcall!(stringBuffer).append(String::from(" and > 0 digits.<br/>"));
                }
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
impl std::ops::Deref for DownloadableItemValidation {
    type Target = Validation;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for DownloadableItemValidation {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::org::allbinary::logic::control::validate::Validation::ValidationDyn for DownloadableItemValidation {
    fn as_Validation(&self) -> &crate::org::allbinary::logic::control::validate::Validation::Validation { &self.base }
    fn as_Validation_mut(&mut self) -> &mut crate::org::allbinary::logic::control::validate::Validation::Validation { &mut self.base }
    fn isValid(&mut self) -> Option<bool> { Self::isValid(self) }
    fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toValidationInfoNode(self, document) }
    fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> { Self::toValidationInfoDoc(self) }
    fn validationInfo(&mut self) -> String { Self::validationInfo(self) }
}


