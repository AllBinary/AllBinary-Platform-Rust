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
use super::*;
use crate::java::util::HashMap::HashMap;
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
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::DownloadableInventoryItemView::DownloadableInventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct AddFileValidationView {
    pub base: DownloadableInventoryItemView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub basicItemData: Option<Rc<RefCell<BasicItemData>>>,
}

thread_local! { pub static AddFileValidationView_ADDPRODUCT: String = String::from("Finish"); }
thread_local! { pub static AddFileValidationView_ADDANOTHERDOWNLOADABLE: String = String::from("Add Another"); }
// impl ValidationComponentInterface for AddFileValidationView with the methods of the trait
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for AddFileValidationView {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite::getTransformInfoInterface(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn::as_TransformInfoHttpComposite_mut(self)) }
}

impl AddFileValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: DownloadableInventoryItemView::new_struct1(transformInfoInterface.clone()),
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
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut command: String = { let java_object = (jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || ((command.cmp(&AddFileValidationView_ADDPRODUCT.with(Clone::clone).clone()) as i32) != 0 && (command.cmp(&AddFileValidationView_ADDANOTHERDOWNLOADABLE.with(Clone::clone).clone()) as i32) != 0) {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Invalid AdminCommand="), command), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
                self.itemInterface = { let java_value = (jcall!(inventoryEntity).getItem(self.id.clone())).clone(); java_value };
            })) {
                Ok(_) => {}
                Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).put(String::from("Existing Item With MoneyException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
                }
                }
            }
            if self.itemInterface.is_none() {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(String::from("Item Does Not Exist"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            if jcall!(BasicItemValidation::new(self.itemInterface.clone())).isValid() == false {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(String::from("BasicItem is not valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance({ let java_argument = jcall!(self.getWeblisketSession()).getStoreName(); java_argument }).map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
            let mut fullCategory: String = jcall!(StringMaker::new()).append(URLGLOBALS::getWebappPath()).append(jcall!(storeFrontInterface).getCurrentHostNamePath()).append(jcall!(self.itemInterface).getCategory()).toString();
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Category Does Not Exist: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            let mut downloadableFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(downloadableFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (downloadableFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log_FileItem(fileItem.clone());
                if self.isValid2(fileName.clone(), size.clone()) == false {
                    return Some(false);
                } else {
                    self.processFile(fileItem.clone());
                    jcall!(self.getRequestHashMap()).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_SIZE.with(Clone::clone).clone()), crate::java_into_object((size.clone()).to_string()));
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()));
            }
            let mut licenseFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(licenseFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (licenseFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log_FileItem(fileItem.clone());
                if self.isValid2(fileName.clone(), size.clone()) == false {
                } else {
                    self.processFile(fileItem.clone());
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()));
            }
            let mut extraFilesObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(extraFilesObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (extraFilesObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                HttpFileUploadUtil::log_FileItem(fileItem.clone());
                if self.isValid2(fileName.clone(), size.clone()) == false {
                } else {
                    self.processFile(fileItem.clone());
                    self.unzip(fileItem.clone());
                }
            } else {
                jcall!(self.getRequestHashMap()).remove(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone()));
            }
            let mut hashMap: Option<Rc<RefCell<HashMap<crate::JavaObject, crate::JavaObject>>>> = self.getRequestHashMap();
            jcall!(hashMap).put(crate::java_into_object({ let java_field = jcall!(self.basicItemData).ID.clone(); java_field }.clone()), crate::java_into_object(jcall!(self.itemInterface).getId()));
            let mut licenseObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()));
            if licenseObject.is_some() {
                jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()), crate::java_into_object(jcall!(((licenseObject).clone() /* as FileItem */)).getName()));
            } else {
                jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()), crate::java_into_object({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()));
            }
            let mut fileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(hashMap).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()));
            if fileObject.is_some() {
                jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()), crate::java_into_object(jcall!(((fileObject).clone() /* as FileItem */)).getName()));
            } else {
                jcall!(hashMap).put(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()), crate::java_into_object({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone()));
            }
            self.downloadableItem = DownloadableItem::new1_HashMap(hashMap.clone());
            let mut downloadableItemValidation: Option<Rc<RefCell<DownloadableItemValidation>>> = DownloadableItemValidation::new(self.downloadableItem.clone());
            if jcall!(downloadableItemValidation).isValid() == false {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(String::from("DownloadableItem is not valid"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Exception in validation"), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(), crate::java_into_object(e.clone()));
            }
            return false;
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    pub fn validationInfo(&mut self) -> String {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            let mut command: String = { let java_object = (jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            if jcall!(StringValidationUtil::getInstance()).isEmpty(command.clone()) || ((command.cmp(&AddFileValidationView_ADDPRODUCT.with(Clone::clone).clone()) as i32) != 0 && (command.cmp(&AddFileValidationView_ADDANOTHERDOWNLOADABLE.with(Clone::clone).clone()) as i32) != 0) {
                return Some({ let java_field = jcall!(StringUtil::getInstance()).EMPTY_STRING.clone(); java_field }.clone());
            }
            if self.itemInterface.is_none() {
                let mut ITEM_NOT_FOUND: String = String::from("Item Does Not Exist");
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(ITEM_NOT_FOUND.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
                }
                return Some(ITEM_NOT_FOUND.clone());
            }
            let mut basicItemValidation: Option<Rc<RefCell<BasicItemValidation>>> = BasicItemValidation::new(self.itemInterface.clone());
            if jcall!(basicItemValidation).isValid() == false {
                jcall!(stringBuffer).append(jcall!(basicItemValidation).validationInfo());
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
            let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance({ let java_argument = jcall!(self.getWeblisketSession()).getStoreName(); java_argument }).map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
            let mut fullCategory: String = jcall!(StringMaker::new()).append(URLGLOBALS::getWebappPath()).append(jcall!(storeFrontInterface).getCurrentHostNamePath()).append(jcall!(self.itemInterface).getCategory()).toString();
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(format!("{}{}", String::from("Category: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
            }
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                jcall!(stringBuffer).append(String::from("Category "));
                jcall!(stringBuffer).append(jcall!(self.itemInterface).getCategory());
                jcall!(stringBuffer).append(String::from(" does not exist."));
                jcall!(stringBuffer).append(String::from("<br />"));
            }
            let mut downloadableFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_FILE.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(downloadableFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (downloadableFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
                jcall!(stringBuffer).append(String::from("Download File Missing"));
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
            let mut licenseFileObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_LICENSE_FILE.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(licenseFileObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (licenseFileObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
            }
            let mut extraFilesObject: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::business::user::commerce::inventory::item::download::DownloadItemData::DownloadItemData_EXTRA_FILES.with(Clone::clone).clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(extraFilesObject.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (extraFilesObject).clone() /* as FileItem */;
                let mut fileName: String = jcall!(fileItem).getName();
                let mut size: i64 = jcall!(fileItem).getSize();
                self.validationInfo3(stringBuffer.clone(), fileName.clone(), size.clone());
            } else {
            }
            if self.downloadableItem.is_some() {
                let mut downloadableItemValidation: Option<Rc<RefCell<DownloadableItemValidation>>> = DownloadableItemValidation::new(self.downloadableItem.clone());
                if jcall!(downloadableItemValidation).isValid() == false {
                    let mut log: String = jcall!(downloadableItemValidation).validationInfo();
                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                        jcall!(self.logUtil).putF(log.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                    }
                    jcall!(stringBuffer).append(log.clone());
                    jcall!(stringBuffer).append(String::from("<br/>"));
                }
            }
            return Some(jcall!(stringBuffer).toString());
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEWERROR.clone(); java_field }.clone()) {
                jcall!(self.logUtil).put(String::from("Failed to generate validation error info"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"), crate::java_into_object(e.clone()));
            }
            return String::from("Error Getting Validation Info");
            None
            }
        };if let Some(java_return) = java_try_result { return java_return; }unreachable!("try statement")
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn isValid2(&mut self, mut fileName: String, mut size: i64) -> Option<bool> {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > (({ let java_field = jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone(); java_field }) as i64) {
            if size < (({ let java_field = jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone(); java_field }) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), { let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone(), { let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone()) {
                    if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                        jcall!(stringBuffer).append(String::from("File name is not valid: "));
                        jcall!(stringBuffer).append(fileName.clone());
                        { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                    }
                    return false;
                } else {
                }
            } else {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("File Size To Large: "));
                    jcall!(stringBuffer).appendlong(size.clone());
                    jcall!(stringBuffer).append(String::from(">"));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone(); java_field }.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
        } else {
            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                jcall!(stringBuffer).append(String::from("File Size To Small: "));
                jcall!(stringBuffer).appendlong(size.clone());
                jcall!(stringBuffer).append(String::from(">"));
                jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone(); java_field }.clone());
                { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
            }
            return false;
        }
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn validationInfo3(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>, mut fileName: String, mut size: i64) {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > (({ let java_field = jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone(); java_field }) as i64) {
            if size < (({ let java_field = jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone(); java_field }) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), { let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone(), { let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone()) {
                    jcall!(stringBuffer).append(String::from("File Name must be more than "));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(String::from(" and less than "));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(String::from(" characters in length: "));
                    jcall!(stringBuffer).append(fileName.clone());
                    jcall!(stringBuffer).append(String::from("<br/>"));
                } else {
                }
            } else {
                jcall!(stringBuffer).append(String::from("File Is Not The Right Size. "));
                jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MINDOWNLOADABLEFILESIZE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(String::from("< >"));
                jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXDOWNLOADABLEFILESIZE.clone(); java_field }.clone());
                jcall!(stringBuffer).append(String::from("<br/>"));
            }
        }
    }
}
impl std::ops::Deref for AddFileValidationView {
    type Target = DownloadableInventoryItemView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for AddFileValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn for AddFileValidationView {
    fn as_DownloadableInventoryItemView(&self) -> &crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView { &self.base }
    fn as_DownloadableInventoryItemView_mut(&mut self) -> &mut crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for AddFileValidationView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for AddFileValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for AddFileValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn getTypeId(&mut self) -> i32 { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView::getTypeId(crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn::as_DownloadableInventoryItemView_mut(self)) }
    fn view(&mut self) -> String { crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemView::view(crate::views::admin::inventory::download::DownloadableInventoryItemView::DownloadableInventoryItemViewDyn::as_DownloadableInventoryItemView_mut(self)) }
}


