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
// package views.generic.inventory
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use super::*;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tree::dom::document::XmlDocumentHelper::XmlDocumentHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchRequest::SearchRequest;
use crate::org::allbinary::logic::system::security::licensing::AbeClientInformationInterface::AbeClientInformationInterface;
use crate::org::allbinary::logic::visual::transform::StoreTransformer::StoreTransformer;
use crate::org::allbinary::logic::visual::transform::data::TransformDocumentInterface::TransformDocumentInterface;
use crate::org::allbinary::logic::visual::transform::data::TransformStoreDocumentFactory::TransformStoreDocumentFactory;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpSearch::TransformInfoHttpSearch;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::allbinary::string::CommonStrings::CommonStrings;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventorySearchProductUtil {
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub commonStrings: Option<Rc<RefCell<CommonStrings>>>,
    pub ITEM_NOT_FOUND: String,
}

thread_local! { pub static InventorySearchProductUtil_instance: Option<Rc<RefCell<InventorySearchProductUtil>>> = InventorySearchProductUtil::new(); }

impl InventorySearchProductUtil {

    pub fn new_struct() -> Self {
        let mut this = Self {
            logUtil: None,
            commonStrings: None,
            ITEM_NOT_FOUND: String::new(),
        };
        this.logUtil = LogUtil::getInstance();
        this.commonStrings = CommonStrings::getInstance();
        this.ITEM_NOT_FOUND = String::from("Item Not Found.");
        this
    }

    pub fn new() -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct())))
    }
    

    pub fn getInstance() -> Option<Rc<RefCell<InventorySearchProductUtil>>> {
        return InventorySearchProductUtil_instance.with(Clone::clone).clone();
    }
    

    // throws Exception
    pub fn getProduct(&mut self, mut abeClientInformation: Option<Rc<RefCell<dyn AbeClientInformationInterface>>>, mut searchRequest: Option<Rc<RefCell<SearchRequest>>>, mut product: String) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventoryEntityInterface: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntityInterface).getItem(product.clone());
            if itemInterface.is_some() {
                let mut viewDocumentInterface: Option<Rc<RefCell<dyn TransformDocumentInterface>>> = TransformStoreDocumentFactory::getInstance(searchRequest.clone());
                jcall!(jcall!(viewDocumentInterface).getBaseNode()).appendChild(jcall!(BasicItemView::new(itemInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>))).toXmlNode(jcall!(viewDocumentInterface).getDoc()));
                let mut success: String = XmlDocumentHelper::toString(jcall!(viewDocumentInterface).getDoc());
                let mut outputStr: String = jcall!(StoreTransformer::new(abeClientInformation.clone(), (TransformInfoHttpSearch::new(searchRequest.clone())).clone() /* as TransformInfoInterface */)).translate(success.clone());
                return outputStr.clone();
            } else {
                return self.ITEM_NOT_FOUND.clone();
            }
        })) {
            Ok(_) => {}
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).PRODUCTSEARCHLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("getProduct"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("Failed to getProduct"))));
            }
        }
    }
}

