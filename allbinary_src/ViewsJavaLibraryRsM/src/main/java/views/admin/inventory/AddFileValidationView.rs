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
// package views.admin.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontInterface::StoreFrontInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemValidation::BasicItemValidation;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::globals::GLOBALS2::GLOBALS2;
use crate::org::allbinary::globals::URLGLOBALS::URLGLOBALS;
use crate::org::allbinary::logic::communication::http::file::upload::HttpFileUploadUtil::HttpFileUploadUtil;
use crate::org::allbinary::logic::communication::http::file::upload::media::UploadMediaSingleton::UploadMediaSingleton;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::io::file::AbFile::AbFile;
use crate::org::allbinary::logic::io::file::FileData::FileData;
use crate::org::allbinary::logic::io::path::AbPathData::AbPathData;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonLabels::CommonLabels;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::InventoryItemView::InventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AddFileValidationView {
    pub base: InventoryItemView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
}

thread_local! { pub static AddFileValidationView_ADDPRODUCT: String = String::from("Add Product"); }
thread_local! { pub static AddFileValidationView_NEXTSTEP: String = format!("{}{}", jcall!(CommonLabels::getInstance()).NEXT, String::from(" Step")); }
// impl ValidationComponentInterface for AddFileValidationView with the methods of the trait

impl AddFileValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: InventoryItemView::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
            basicItemData: None,
        };
        this.logUtil = LogUtil::getInstance();
        this.basicItemData = BasicItemData::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut command: String = (jcall!(self.getRequestHashMap()).get(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone())).clone() /* as String */;
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || ((command.cmp(&AddFileValidationView_ADDPRODUCT.with(Clone::clone).clone()) as i32) != 0 && (command.cmp(&AddFileValidationView_NEXTSTEP.with(Clone::clone).clone()) as i32) != 0) {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("Invalid AdminCommand: "));
                    jcall!(stringBuffer).append(command.clone());
                    jcall!(stringBuffer).append(String::from(" must be: "));
                    jcall!(stringBuffer).append(AddFileValidationView_ADDPRODUCT.with(Clone::clone).clone());
                    jcall!(stringBuffer).append(String::from(" or "));
                    jcall!(stringBuffer).append(AddFileValidationView_NEXTSTEP.with(Clone::clone).clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            let mut basicItemValidation: Option<Rc<RefCell<BasicItemValidation>>> = BasicItemValidation::new(self.itemInterface.clone());
            if match (&jcall!(basicItemValidation).isValid(), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(basicItemValidation).validationInfo(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance(jcall!(self.getWeblisketSession()).getStoreName());
            { let java_receiver = self.itemInterface.clone(); let java_arguments = (jcall!(storeFrontInterface).getCategoryPath() + jcall!(self.itemInterface).getCategory(),); jcall!(java_receiver).setCategory(java_arguments.0); };
            let mut fullCategory: String = jcall!(StringMaker::new()).append(URLGLOBALS::getWebappPath()).append(jcall!(storeFrontInterface).getCurrentHostNamePath()).append(jcall!(self.itemInterface).getCategory()).toString();
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Category Does Not Exist: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            if jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(jcall!(self.itemInterface).getId()).is_some() {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Item Already Exists"), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone());
                }
                return false;
            }
            let mut imageFileItemObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(jcall!(self.basicItemData).IMAGE.clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(imageFileItemObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (imageFileItemObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log(fileItem.clone());
                if match (&self.isValid2(fileName.clone(), size.clone()), &false) { (Some(left), Some(right)) => std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right)), (None, None) => true, _ => false } {
                    return false;
                }
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
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || ((command.cmp(&AddFileValidationView_ADDPRODUCT.with(Clone::clone).clone()) as i32) != 0 && (command.cmp(&AddFileValidationView_NEXTSTEP.with(Clone::clone).clone()) as i32) != 0) {
                return jcall!(StringUtil::getInstance()).EMPTY_STRING.clone();
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
                jcall!(stringBuffer).append(String::from(" does not exist.<br />"));
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(jcall!(self.itemInterface).getId()).is_some() {
                    jcall!(stringBuffer).append(String::from("Id is already in use. Refresh the page for a valid id.<br/>"));
                }
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    jcall!(self.logUtil).putF(String::from("Existing Item With MoneyException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
                }
                }
            }
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(jcall!(self.basicItemData).IMAGE.clone());
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(anyType.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (anyType).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut fileItemFieldName: String = jcall!(fileItem).getFieldName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo4(stringBuffer.clone(), fileName.clone(), fileItemFieldName.clone(), size.clone());
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
        if size > ((jcall!(fileData).MINIMAGEFILESIZE) as i64) {
            if size < ((jcall!(fileData).MAXIMAGEFILESIZE) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), jcall!(fileData).MINLEN.clone(), jcall!(fileData).MAXLEN.clone()) {
                    return false;
                } else {
                    let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
                    let mut extension: String = jcall!(pathData).getExtension(fileName.clone());
                    let mut uploadMedia: Option<Rc<RefCell<UploadMediaSingleton>>> = UploadMediaSingleton::getInstance();
                    if !jcall!(uploadMedia).isWriterSupported(extension.clone()) && !jcall!(uploadMedia).isReaderSupported(extension.clone()) {
                        return false;
                    }
                }
            } else {
                if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("File Size To Large: "));
                    jcall!(stringBuffer).appendlong(size.clone());
                    jcall!(stringBuffer).append(String::from(">"));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MAXIMAGEFILESIZE.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            self.processImageFiles();
        } else {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("File Size To Small: "));
                jcall!(stringBuffer).appendlong(size.clone());
                jcall!(stringBuffer).append(String::from(">"));
                jcall!(stringBuffer).appendint(jcall!(fileData).MINIMAGEFILESIZE.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), jcall!(self.commonStrings).IS_VALID.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return false;
        }
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn validationInfo4(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>, mut fileName: String, mut fileItemFieldName: String, mut size: i64) {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > ((jcall!(fileData).MINIMAGEFILESIZE) as i64) {
            if (fileItemFieldName.cmp(&jcall!(self.basicItemData).IMAGE.clone()) as i32) == 0 {
                if size < ((jcall!(fileData).MAXIMAGEFILESIZE) as i64) {
                    if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), jcall!(fileData).MINLEN.clone(), jcall!(fileData).MAXLEN.clone()) {
                        jcall!(stringBuffer).append(String::from("FileName must be >"));
                        jcall!(stringBuffer).appendint(jcall!(fileData).MINLEN.clone());
                        jcall!(stringBuffer).append(String::from(" and <"));
                        jcall!(stringBuffer).appendint(jcall!(fileData).MAXLEN.clone());
                        jcall!(stringBuffer).append(String::from("<br/>"));
                    } else {
                        let mut pathData: Option<Rc<RefCell<AbPathData>>> = AbPathData::getInstance();
                        let mut extension: String = jcall!(pathData).getExtension(fileName.clone());
                        let mut uploadMedia: Option<Rc<RefCell<UploadMediaSingleton>>> = UploadMediaSingleton::getInstance();
                        if !jcall!(uploadMedia).isWriterSupported(extension.clone()) && !jcall!(uploadMedia).isReaderSupported(extension.clone()) {
                            jcall!(stringBuffer).append(String::from("Image type: "));
                            jcall!(stringBuffer).append(extension.clone());
                            jcall!(stringBuffer).append(String::from(" not supported.<br />"));
                        }
                    }
                } else {
                    jcall!(stringBuffer).append(String::from("Image File Is To Large. "));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MINIMAGEFILESIZE.clone());
                    jcall!(stringBuffer).append(String::from(" < > "));
                    jcall!(stringBuffer).appendint(jcall!(fileData).MAXIMAGEFILESIZE.clone());
                    jcall!(stringBuffer).append(String::from("<br/>"));
                }
            }
        } else {
            jcall!(stringBuffer).append(String::from("Image File Is To Small. "));
            jcall!(stringBuffer).appendint(jcall!(fileData).MINIMAGEFILESIZE.clone());
            jcall!(stringBuffer).append(String::from(" < > "));
            jcall!(stringBuffer).appendint(jcall!(fileData).MAXIMAGEFILESIZE.clone());
            jcall!(stringBuffer).append(String::from("<br/>"));
        }
    }
}
impl std::ops::Deref for AddFileValidationView {
    type Target = InventoryItemView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AddFileValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn for AddFileValidationView {
    fn as_InventoryItemView(&self) -> &crate::views::admin::inventory::InventoryItemView::InventoryItemView { &self.base }
    fn as_InventoryItemView_mut(&mut self) -> &mut crate::views::admin::inventory::InventoryItemView::InventoryItemView { &mut self.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for AddFileValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for AddFileValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn getTypeId(&mut self) -> i32 { crate::views::admin::inventory::InventoryItemView::InventoryItemView::getTypeId(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
    fn view(&mut self) -> String { crate::views::admin::inventory::InventoryItemView::InventoryItemView::view(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
}


