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
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemData::BasicItemData;
use crate::org::allbinary::business::user::commerce::money::MoneyException::MoneyException;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::downloads::DownloadItemsEntity::DownloadItemsEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::downloads::DownloadItemsEntityFactory::DownloadItemsEntityFactory;
use crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParams;
use crate::org::allbinary::logic::control::validate::ValidationComponentInterface::ValidationComponentInterface;
use crate::org::allbinary::logic::string::StringMaker::StringMaker;
use crate::org::allbinary::logic::string::StringUtil::StringUtil;
use crate::org::allbinary::logic::string::StringValidationUtil::StringValidationUtil;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use super::InventoryItemView::InventoryItemView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct EditValidationView {
    pub base: InventoryItemView,
    pub id: String,
}

// impl ValidationComponentInterface for EditValidationView with the methods of the trait

impl EditValidationView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: InventoryItemView::new_struct2(transformInfoInterface.clone(), jcall!(StringUtil::getInstance()).EMPTY_STRING.clone()),
            id: String::new(),
        };
        // super(...) is in the struct literal above
        this.getFormData();
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn getFormData(&mut self) {
        let mut hashMap: Option<Rc<RefCell<HashMap>>> = jcall!(RequestParams::new1(self.base.request.clone()).map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::logic::communication::http::request::RequestParams::RequestParamsDyn>>)).toHashMap();
        if hashMap.is_none() {
            panic!("{:?}", format!("{}: {:?}", "Exception", (String::from("No Request Params Found"))));
        }
        self.id = (jcall!(hashMap).get(jcall!(self.base.basicItemData).ID.clone())).clone() /* as String */;
    }
    

    // throws Exception
    pub fn isValid(&mut self) -> Option<bool> {
        if false || !jcall!(StringValidationUtil::getInstance()).isNumber(self.id.clone()) {
            return false;
        }
        self.itemInterface = jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(self.id.clone());
        if self.itemInterface.is_none() {
            return false;
        }
        let mut downloadable: i32 = jcall!(self.itemInterface).getDownloads().parse::<i32>().unwrap();
        if downloadable != 0 {
            let mut downloadItemsEntity: Option<Rc<RefCell<DownloadItemsEntity>>> = jcall!(DownloadItemsEntityFactory::getInstance()).getDownloadItemsEntityInstance();
            self.downloadableItemVector = jcall!(downloadItemsEntity).getForItem(jcall!(self.itemInterface).getId());
        }
        return true;
    }
    

    pub fn toValidationInfoDoc(&mut self) -> Option<Rc<RefCell<Document>>> {
        return None;
    }
    

    pub fn toValidationInfoNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        return None;
    }
    

    // throws MoneyException
    pub fn validationInfo(&mut self) -> String {
        let mut stringBuffer: Option<Rc<RefCell<StringMaker>>> = StringMaker::new();
        if false || !jcall!(StringValidationUtil::getInstance()).isNumber(self.id.clone()) {
            jcall!(stringBuffer).append(String::from("Id is not valid.<br />"));
        }
        if jcall!(jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance()).getItem(self.id.clone()).is_none() {
            jcall!(stringBuffer).append(String::from("Item does not exist.<br />"));
        }
        return jcall!(stringBuffer).toString();
    }
}
impl std::ops::Deref for EditValidationView {
    type Target = InventoryItemView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for EditValidationView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

impl crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn for EditValidationView {
    fn as_InventoryItemView(&self) -> &crate::views::admin::inventory::InventoryItemView::InventoryItemView { &self.base }
    fn as_InventoryItemView_mut(&mut self) -> &mut crate::views::admin::inventory::InventoryItemView::InventoryItemView { &mut self.base }
    fn getFormData(&mut self) { Self::getFormData(self) }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for EditValidationView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for EditValidationView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base.base }
    fn getTypeId(&mut self) -> i32 { crate::views::admin::inventory::InventoryItemView::InventoryItemView::getTypeId(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
    fn view(&mut self) -> String { crate::views::admin::inventory::InventoryItemView::InventoryItemView::view(crate::views::admin::inventory::InventoryItemView::InventoryItemViewDyn::as_InventoryItemView_mut(self)) }
}


