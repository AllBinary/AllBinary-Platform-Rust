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
// package views.admin.inventory.download
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemValidation::BasicItemValidation;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItem::DownloadableItem;
use crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadableItemValidation::DownloadableItemValidation;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::file::upload::HttpFileUploadUtil::HttpFileUploadUtil;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileData::FileData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::DownloadableInventoryItemView::DownloadableInventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpdateFileValidationView {
    pub base: DownloadableInventoryItemView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
}

// impl ValidationComponentInterface for UpdateFileValidationView with the methods of the trait

impl UpdateFileValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: DownloadableInventoryItemView::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
            commonStrings: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut command: String = (jcall!(self.getRequestHashMap()).get(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone())).clone() /* as String */;
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || (command.cmp(&jcall!(self.commonStrings).UPDATE.clone()) as i32) != 0 {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Invalid AdminCommand="), command), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
                self.itemInterface = jcall!(inventoryEntity).getItem(self.id.clone());
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).put(String::from("Existing Item With MoneyException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
                }
                }
            }
            if self.itemInterface.is_none() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Item Does Not Exist"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            if match (&jcall!(BasicItemValidation::new(self.itemInterface.clone())).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("BasicItem is not valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getWeblisketSession()).getStoreName());
            let mut fullCategory: String = jcall!(StringMaker::new()).append(URLGLOBALS::getWebappPath()).append(jcall!(storeFrontInterface).getCurrentHostNamePath()).append(jcall!(self.itemInterface).getCategory()).toString();
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Category Does Not Exist: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            let mut downloadableFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(downloadableFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (downloadableFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log(fileItem.clone());
                if match (&self.isValid2(fileName.clone(), size.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return false;
                } else {
                    self.processFile(fileItem.clone());
                    jcall!(self.getRequestHashMap()).put(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_SIZE.with(Clone::clone).clone(), (size.clone()).to_string());
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone());
            }
            let mut licenseFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(licenseFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (licenseFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log(fileItem.clone());
                if match (&self.isValid2(fileName.clone(), size.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                } else {
                    self.processFile(fileItem.clone());
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone());
            }
            let mut extraFilesObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(extraFilesObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (extraFilesObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log(fileItem.clone());
                if match (&self.isValid2(fileName.clone(), size.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                } else {
                    self.processFile(fileItem.clone());
                    self.unzip(fileItem.clone());
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone());
            }
            let mut hashMap: Option<Rc<RefCell<HashMap>>> = self.getRequestHashMap();
            jcall!(hashMap).put(jcall!(self.base.basicItemData).ID.clone(), jcall!(self.itemInterface).getId());
            let mut licenseObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone());
            if licenseObject.is_some() {
                jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone(), jcall!(((licenseObject).clone() /* as FileItem */)).getName());
            } else {
                jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
            }
            let mut fileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone());
            if fileObject.is_some() {
                jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone(), jcall!(((fileObject).clone() /* as FileItem */)).getName());
            } else {
                jcall!(hashMap).put(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone());
            }
            self.downloadableItem = DownloadableItem::new1(hashMap.clone());
            let mut downloadableItemValidation: Option<Rc<RefCell<DownloadableItemValidation>>> = DownloadableItemValidation::new(self.downloadableItem.clone());
            if match (&jcall!(downloadableItemValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("DownloadableItem is not valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            return true;
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Exception in validation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            }
        }
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn validationInfo(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut command: String = (jcall!(self.getRequestHashMap()).get(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone())).clone() /* as String */;
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || (command.cmp(&jcall!(self.commonStrings).UPDATE.clone()) as i32) != 0 {
                return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
            }
            if self.itemInterface.is_none() {
                let mut ITEM_NOT_FOUND: String = String::from("Item Does Not Exist");
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(ITEM_NOT_FOUND.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
                }
                return ITEM_NOT_FOUND.clone();
            }
            let mut basicItemValidation: Option<Rc<RefCell<BasicItemValidation>>> = BasicItemValidation::new(self.itemInterface.clone());
            if match (&jcall!(basicItemValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                jcall!(stringBuffer).append(jcall!(basicItemValidation).validationInfo());
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getWeblisketSession()).getStoreName());
            let mut fullCategory: String = jcall!(StringMaker::new()).append(URLGLOBALS::getWebappPath()).append(jcall!(storeFrontInterface).getCurrentHostNamePath()).append(jcall!(self.itemInterface).getCategory()).toString();
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Category: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
            }
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                jcall!(stringBuffer).append(String::from("Category "));
                jcall!(stringBuffer).append(jcall!(self.itemInterface).getCategory());
                jcall!(stringBuffer).append(String::from(" does not exist."));
                jcall!(stringBuffer).append(String::from("<br />"));
            }
            let mut downloadableFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(downloadableFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (downloadableFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
                jcall!(stringBuffer).append(String::from("Download File Missing"));
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
            let mut licenseFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(licenseFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (licenseFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
            }
            let mut extraFilesObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(extraFilesObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (extraFilesObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
            }
            if self.downloadableItem.is_some() {
                let mut downloadableItemValidation: Option<Rc<RefCell<DownloadableItemValidation>>> = DownloadableItemValidation::new(self.downloadableItem.clone());
                if match (&jcall!(downloadableItemValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    let mut log: String = jcall!(downloadableItemValidation).validationInfo();
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        jcall!(self.logUtil).putF(log.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                    }
                    jcall!(stringBuffer).append(log.clone());
                    jcall!(stringBuffer).append(String::from("<br/>"));
                }
            }
            return jcall!(stringBuffer).toString();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            }
        }
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn isValid2(&mut self, mut fileName: String, mut size: i64) -> Option<bool> {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > ((jcall!(fileData).MINDOWNLOADABLEFILESIZE) as i64) {
            if size < ((jcall!(fileData).MAXDOWNLOADABLEFILESIZE) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), jcall!(fileData).MINLEN.clone(), jcall!(fileData).MAXLEN.clone()) {
                    if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("File name is not valid: "));
                        jcall!(stringBuffer).append(fileName.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    return false;
                } else {
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("File Size To Large: "));
                    jcall!(stringBuffer).appendlong(size.clone());
                    jcall!(stringBuffer).append(String::from(">"));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("File Size To Small: "));
                jcall!(stringBuffer).appendlong(size.clone());
                jcall!(stringBuffer).append(String::from(">"));
                jcall!(stringBuffer).appendint(jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return false;
        }
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn validationInfo3(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>, mut fileName: String, mut size: i64) {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > ((jcall!(fileData).MINDOWNLOADABLEFILESIZE) as i64) {
            if size < ((jcall!(fileData).MAXDOWNLOADABLEFILESIZE) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), jcall!(fileData).MINLEN.clone(), jcall!(fileData).MAXLEN.clone()) {
                    jcall!(stringBuffer).append(String::from("File Name must be more than "));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MINLEN.clone());
                    jcall!(stringBuffer).append(String::from(" and less than "));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MAXLEN.clone());
                    jcall!(stringBuffer).append(String::from(" characters in length: "));
                    jcall!(stringBuffer).append(fileName.clone());
                    jcall!(stringBuffer).append(String::from("<br/>"));
                } else {
                }
            } else {
                jcall!(stringBuffer).append(String::from("File Is Not The Right Size. "));
                jcall!(stringBuffer).appendint(jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone());
                jcall!(stringBuffer).append(String::from("< >"));
                jcall!(stringBuffer).appendint(jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone());
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
        }
    }
}
impl std::ops::Deref for UpdateFileValidationView {
    type Target = DownloadableInventoryItemView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpdateFileValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn for UpdateFileValidationView {
    fn as_DownloadableInventoryItemView(&self) -> &crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView { &self.base }
    fn as_DownloadableInventoryItemView_mut(&mut self) -> &mut crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for UpdateFileValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for UpdateFileValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn getTypeId(&mut self) -> i32 { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView::getTypeId(crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn::as_DownloadableInventoryItemView_mut(self)) }
    fn view(&mut self) -> String { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView::view(crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn::as_DownloadableInventoryItemView_mut(self)) }
}


