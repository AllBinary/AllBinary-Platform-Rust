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
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonSeps::CommonSeps;
use crate::org::allbinary::string::CommonStrings::CommonStrings;
use crate::org::apache::commons::fileupload::FileItem::FileItem;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::java::lang::Exception::*;
use super::InventoryItemView::InventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct UpdateFileValidationView {
    pub base: InventoryItemView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

thread_local! { pub static UpdateFileValidationView_UPDATEPRODUCT: String = { let java_field = jcall!(CommonStrings::getInstance()).UPDATE.clone(); java_field }.clone(); }
// impl ValidationComponentInterface for UpdateFileValidationView with the methods of the trait
impl crate::org::allbinary::logic::visual::transform::info::CompositeTransformInfoInterface::CompositeTransformInfoInterface for UpdateFileValidationView {
    fn getTransformInfoInterface(&mut self) -> Option<Rc<RefCell<dyn crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface>>> { crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite::getTransformInfoInterface(crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn::as_TransformInfoHttpComposite_mut(self)) }
}

impl UpdateFileValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: InventoryItemView::new_struct1(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    pub fn isValid(&mut self) -> Option<bool> {
        let java_try_result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut command: String = { let java_object = (jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
            if false || (command.cmp(&UpdateFileValidationView_UPDATEPRODUCT.with(Clone::clone).clone()) as i32) != 0 {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Invalid AdminCommand="), command), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
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
            let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
            jcall!(stringBuffer).append(URLGLOBALS::getWebappPath());
            jcall!(stringBuffer).append(jcall!(storeFrontInterface).getCurrentHostNamePath());
            jcall!(stringBuffer).append(jcall!(self.itemInterface).getCategory());
            let mut fullCategory: String = jcall!(stringBuffer).toString();
            let mut categoryFile: Option<Rc<RefCell<AbFile>>> = AbFile::createAbFile(fullCategory.clone());
            if !jcall!(categoryFile).isDirectory() {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Category Does Not Exist: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone());
                }
                return Some(false);
            }
            let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            if jcall!(inventoryEntity).getItem(jcall!(self.itemInterface).getId()).is_none() {
                return Some(false);
            }
            let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object({ let java_field = jcall!(self.base.basicItemData).IMAGE.clone(); java_field }.clone()));
            if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(anyType.clone())) {
                let mut fileItem: Option<Rc<RefCell<FileItem>>> = (anyType).clone() /* as FileItem */;
                let mut size: i64 = jcall!(fileItem).getSize();
                let mut fileName: String = jcall!(fileItem).getName();
                HttpFileUploadUtil::log_FileItem(fileItem.clone());
                if self.isValid2(fileName.clone(), size.clone()) == false {
                    return Some(false);
                }
            }
            return Some(true);
        })) {
            Ok(java_return) => java_return,
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
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
    

    // throws Exception
    pub fn validationInfo(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        let mut command: String = { let java_object = (jcall!(self.getRequestHashMap()).get(crate::java_into_object(crate::org::allbinary::globals::GLOBALS2::GLOBALS2_ADMINCOMMAND.with(Clone::clone).clone()))).clone(); java_object.map(|object| object.borrow().downcast_ref::<String>().cloned().expect("ClassCastException: not a String")).unwrap_or_default() };
        if false || (command.cmp(&UpdateFileValidationView_UPDATEPRODUCT.with(Clone::clone).clone()) as i32) != 0 {
            return { let java_field = jcall!(CommonSeps::getInstance()).SPACE.clone(); java_field }.clone();
        }
        jcall!(stringBuffer).append(jcall!(BasicItemValidation::new(self.itemInterface.clone())).validationInfo());
        let mut storeFrontInterface: Option<Rc<RefCell<dyn StoreFrontInterface>>> = StoreFrontFactory::getInstance({ let java_argument = jcall!(self.getWeblisketSession()).getStoreName(); java_argument }).map(|reference| reference as Rc<RefCell<dyn StoreFrontInterface>>);
        let mut fullCategory: String = format!("{}{}", format!("{}{}", (URLGLOBALS::getWebappPath()).clone() /* as String */, jcall!(storeFrontInterface).getCurrentHostNamePath()), jcall!(self.itemInterface).getCategory());
        if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
            jcall!(self.logUtil).putF(format!("{}{}", String::from("Category: "), fullCategory), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
        }
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(jcall!(self.itemInterface).getId()).is_none() {
                jcall!(stringBuffer).append(String::from("Item does not exist.<br>"));
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: String = crate::java::lang::Exception::java_panic_message(&e);

            if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                jcall!(self.logUtil).putF(String::from("Existing Item With MoneyException"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("validationInfo()"));
            }
            }
        }
        let mut anyType: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = jcall!(self.getRequestHashMap()).get(crate::java_into_object({ let java_field = jcall!(self.base.basicItemData).IMAGE.clone(); java_field }.clone()));
        if jcall!(HttpFileUploadUtil::getInstance()).isValid(crate::java_into_object(anyType.clone())) {
            let mut fileItem: Option<Rc<RefCell<FileItem>>> = (anyType).clone() /* as FileItem */;
            let mut size: i64 = jcall!(fileItem).getSize();
            let mut fileName: String = jcall!(fileItem).getName();
            let mut fileItemFieldName: String = jcall!(fileItem).getFieldName();
            self.validationInfo4(stringBuffer.clone(), fileName.clone(), fileItemFieldName.clone(), size.clone());
        }
        return jcall!(stringBuffer).toString();
    }
    

    // throws Exception
    // Java overloaded method: Rust needs a different name
    pub fn isValid2(&mut self, mut fileName: String, mut size: i64) -> Option<bool> {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > (({ let java_field = jcall!(fileData).MINIMAGEFILESIZE.clone(); java_field }) as i64) {
            if size < (({ let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field }) as i64) {
                if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), { let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone(), { let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone()) {
                    return false;
                }
            } else {
                if jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypes.clone(); java_field }).LOGGING.clone(); java_field }).contains({ let java_field = jcall!(jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!({ let java_field = jcall!(unsafe { org_allbinary.clone() }).logic.clone(); java_field }).communication.clone(); java_field }).log.clone(); java_field }).config.clone(); java_field }).r#type.clone(); java_field }).LogConfigTypeFactory.clone(); java_field }).getInstance()).VIEW.clone(); java_field }.clone()) {
                    let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
                    jcall!(stringBuffer).append(String::from("File Size To Large: "));
                    jcall!(stringBuffer).appendlong(size.clone());
                    jcall!(stringBuffer).append(String::from(">"));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field }.clone());
                    { let java_receiver = self.logUtil.clone(); let java_arguments = (jcall!(stringBuffer).toString(), crate::java_into_object(String::from(std::any::type_name::<Self>())), { let java_field = jcall!(self.commonStrings).IS_VALID.clone(); java_field }.clone(),); jcall!(java_receiver).putF(java_arguments.0, java_arguments.1, java_arguments.2); };
                }
                return false;
            }
            self.processImageFiles();
        }
        return true;
    }
    

    // Java overloaded method: Rust needs a different name
    pub fn validationInfo4(&mut self, mut stringBuffer: Option<Rc<RefCell<StringMaker>>>, mut fileName: String, mut fileItemFieldName: String, mut size: i64) {
        let mut fileData: Option<Rc<RefCell<FileData>>> = FileData::getInstance();
        if size > (({ let java_field = jcall!(fileData).MINIMAGEFILESIZE.clone(); java_field }) as i64) {
            if (fileItemFieldName.cmp(&{ let java_field = jcall!(self.base.basicItemData).IMAGE.clone(); java_field }.clone()) as i32) == 0 {
                if size < (({ let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field }) as i64) {
                    if !jcall!(StringValidationUtil::getInstance()).isValidRequired(fileName.clone(), { let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone(), { let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone()) {
                        jcall!(stringBuffer).append(String::from("FileName must be >"));
                        jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MINLEN.clone(); java_field }.clone());
                        jcall!(stringBuffer).append(String::from("and <"));
                        jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXLEN.clone(); java_field }.clone());
                        jcall!(stringBuffer).append(String::from("<br />"));
                    }
                } else {
                    jcall!(stringBuffer).append(String::from("Image File Is Not The Right Size. "));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MINIMAGEFILESIZE.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(String::from("< > "));
                    jcall!(stringBuffer).appendint({ let java_field = jcall!(fileData).MAXIMAGEFILESIZE.clone(); java_field }.clone());
                    jcall!(stringBuffer).append(String::from("<br />"));
                }
            }
        }
    }
}
impl std::ops::Deref for UpdateFileValidationView {
    type Target = InventoryItemView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for UpdateFileValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn for UpdateFileValidationView {
    fn as_InventoryItemView(&self) -> &crate::views::admin::inventory::InventoryItemView::InventoryItemView { &self.base }
    fn as_InventoryItemView_mut(&mut self) -> &mut crate::views::admin::inventory::InventoryItemView::InventoryItemView { &mut self.base }
}
impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for UpdateFileValidationView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for UpdateFileValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for UpdateFileValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn getTypeId(&mut self) -> i32 { crate::views::admin::inventory::InventoryItemView::InventoryItemView::getTypeId(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
    fn view(&mut self) -> String { crate::views::admin::inventory::InventoryItemView::InventoryItemView::view(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
}


