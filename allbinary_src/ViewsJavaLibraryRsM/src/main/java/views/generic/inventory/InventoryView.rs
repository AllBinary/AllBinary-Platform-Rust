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
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
use crate::org::allbinary::business::context::modules::storefront::StoreFrontFactory::StoreFrontFactory;
use crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntity::InventoryEntity;
use crate::org::allbinary::data::tables::user::commerce::inventory::item::InventoryEntityFactory::InventoryEntityFactory;
use crate::org::allbinary::data::tree::dom::DomNodeInterface::DomNodeInterface;
use crate::org::allbinary::data::tree::dom::ModDomHelper::ModDomHelper;
use crate::org::allbinary::logic::communication::log::LogUtil::LogUtil;
use crate::org::allbinary::logic::control::search::SearchData::SearchData;
use crate::org::allbinary::logic::visual::transform::info::TransformInfoInterface::TransformInfoInterface;
use crate::org::w3c::dom::Document::Document;
use crate::org::w3c::dom::Node::Node;
use crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView;

// Borrows the object of a Java reference
macro_rules! jcall {
    ($value:expr) => {
        $value.as_ref().expect("null reference").borrow_mut()
    };
}

pub struct InventoryView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
}

impl DomNodeInterface for InventoryView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl InventoryView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEW.clone()) {
                jcall!(self.logUtil).putF(String::from("Starting"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
            }
            let mut inventoryEntityInterface: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut inventoryNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::InventoryData::InventoryData_INVENTORY.with(Clone::clone).clone());
            let mut itemVector: Option<Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayList::BasicArrayListDyn>>> = jcall!(inventoryEntityInterface).getItems(StoreFrontFactory::getInstance(jcall!(self.getTransformInfoInterface()).getStoreName()));
            jcall!(inventoryNode).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS_ON_THIS_PAGE.with(Clone::clone).clone(), jcall!(Some((jcall!(itemVector).size()) as i32)).toString()));
            let mut size: i32 = jcall!(itemVector).size();
            for index in (0..size) {
                let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = { let java_object = jcall!(itemVector).get(index.clone()); java_object.as_ref().and_then(|object| object.borrow().downcast_ref::<Option<Rc<RefCell<dyn ItemInterface>>>>().expect("ClassCastException").clone()) };
                if itemInterface.is_some() {
                    let mut node: Option<Rc<RefCell<Node>>> = jcall!(BasicItemView::new(itemInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>))).toXmlNode(document.clone());
                    jcall!(inventoryNode).appendChild(node.clone());
                } else {
                    jcall!(self.logUtil).putF(String::from("Inventory"), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
                }
            }
            return inventoryNode.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).VIEWERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            let mut error: String = String::from("Failed to view Inventory");
            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            return error.clone();
            }
        }
    }
}
impl std::ops::Deref for InventoryView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for InventoryView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait InventoryViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_InventoryView(&self) -> &InventoryView;
    fn as_InventoryView_mut(&mut self) -> &mut InventoryView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { InventoryView::toXmlNode(self.as_InventoryView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { InventoryView::addDomNodeInterfaces(self.as_InventoryView_mut()) }
}
impl InventoryViewDyn for InventoryView {
    fn as_InventoryView(&self) -> &InventoryView { self }
    fn as_InventoryView_mut(&mut self) -> &mut InventoryView { self }
}

impl crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpCompositeDyn for InventoryView {
    fn as_TransformInfoHttpComposite(&self) -> &crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &self.base.base.base }
    fn as_TransformInfoHttpComposite_mut(&mut self) -> &mut crate::org::allbinary::logic::visual::transform::info::TransformInfoHttpComposite::TransformInfoHttpComposite { &mut self.base.base.base }
}
impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for InventoryView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for InventoryView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


