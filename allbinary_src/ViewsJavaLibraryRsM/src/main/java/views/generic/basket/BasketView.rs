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
// package views.generic.basket
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused, unused_mut, static_mut_refs)]

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use super::*;
// use crate::java::util::HashMap::HashMap;
// use crate::java::util::Set::Set;
use crate::org::allbinary::util::BasicArrayList::BasicArrayList;
use crate::org::allbinary::util::BasicArrayListD::BasicArrayListD;
// use crate::javax::servlet::http::HttpServletRequest::HttpServletRequest;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData;
use crate::org::allbinary::business::user::commerce::inventory::basket::BasketInterface::BasketInterface;
use crate::org::allbinary::business::user::commerce::inventory::item::BasicItemView::BasicItemView;
use crate::org::allbinary::business::user::commerce::inventory::item::ItemInterface::ItemInterface;
use crate::org::allbinary::business::user::commerce::money::Money::Money;
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

pub struct BasketView {
    pub base: HttpStoreComponentView,
    pub logUtil: Option<Rc<RefCell<LogUtil>>>,
    pub request: Option<Rc<RefCell<HttpServletRequest>>>,
}

impl DomNodeInterface for BasketView {
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { Self::toXmlNode(self, document) }
}

impl BasketView {
    

    pub fn new_struct(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Self {
        let mut this = Self {
            base: HttpStoreComponentView::new_struct(transformInfoInterface.clone()),
            logUtil: None,
            request: None,
        };
        this.logUtil = LogUtil::getInstance();
        // super(...) is in the struct literal above
        this.request = (jcall!(this.getPageContext()).getRequest()).clone() /* as HttpServletRequest */;
        this
    }

    pub fn new(mut transformInfoInterface: Option<Rc<RefCell<dyn TransformInfoInterface>>>) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(Self::new_struct(transformInfoInterface))))
    }
    

    // throws Exception
    pub fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut inventoryEntity: Option<Rc<RefCell<InventoryEntity>>> = jcall!(InventoryEntityFactory::getInstance()).getInventoryEntityInstance();
            let mut basketInterface: Option<Rc<RefCell<dyn BasketInterface>>> = jcall!(jcall!(self.getWeblisketSession()).getOrder()).getBasket();
            let mut basketNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_BASKET.with(Clone::clone).clone());
            let mut itemsAndNumberInBasket: Option<Rc<RefCell<HashMap>>> = jcall!(basketInterface).getItems();
            let mut numberOfResults: i32 = 1;
            let mut items: Option<Rc<RefCell<dyn Set>>> = jcall!(itemsAndNumberInBasket).keySet();
            let mut productArray: Vec<Option<Rc<RefCell<Box<dyn std::any::Any>>>>> = jcall!(items).toArray();
            let mut size: i32 = (productArray.len() as i32).clone();
            for index in (0..size) {
                let mut product: String = (productArray[(index) as usize]).clone() /* as String */.clone();
                let mut itemInterface: Option<Rc<RefCell<dyn ItemInterface>>> = jcall!(inventoryEntity).getItem(product.clone());
                if itemInterface.is_some() {
                    let mut basicItemView: Option<Rc<RefCell<BasicItemView>>> = BasicItemView::new(itemInterface.clone(), BasicArrayListD::new().map(|reference| reference as Rc<RefCell<dyn crate::org::allbinary::util::BasicArrayListD::BasicArrayListDDyn>>));
                    let mut node: Option<Rc<RefCell<Node>>> = jcall!(basicItemView).toXmlNode(document.clone());
                    let mut numberInBasket: String = jcall!(jcall!(basketInterface).getNumberOf(product.clone())).toString();
                    jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ITEMTOTALINBASKET.with(Clone::clone).clone(), numberInBasket.clone()));
                    let mut itemPrice: Option<Rc<RefCell<Money>>> = jcall!(itemInterface).getPrice();
                    let mut itemTotal: Option<Rc<RefCell<Money>>> = Money::new1(itemPrice.clone());
                    jcall!(itemTotal).multiply(((jcall!(basketInterface).getNumberOf(product.clone())).unwrap() as i32));
                    jcall!(node).appendChild(ModDomHelper::createNameValueNodes(document.clone(), crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_ITEMTOTAL.with(Clone::clone).clone(), jcall!(itemTotal).toString()));
                    jcall!(basketNode).appendChild(node.clone());
                } else {
                    jcall!(self.logUtil).putF(format!("{}{}", String::from("Product Failed: "), product), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"));
                }
                numberOfResults += 1;
            }
            let mut totalNumberNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::logic::control::search::SearchData::SearchData_TOTAL_NUMBER_ITEMS.with(Clone::clone).clone());
            let mut totalNumberTextNode: Option<Rc<RefCell<Node>>> = jcall!(document).createTextNode(jcall!(jcall!(basketInterface).getNumberOfItems()).toString());
            jcall!(totalNumberNode).appendChild(totalNumberTextNode.clone());
            let mut totalWeightNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_TOTALWEIGHT.with(Clone::clone).clone());
            let mut totalWeightTextNode: Option<Rc<RefCell<Node>>> = jcall!(document).createTextNode(jcall!(jcall!(basketInterface).getTotalWeight()).toString());
            jcall!(totalWeightNode).appendChild(totalWeightTextNode.clone());
            let mut subTotalNode: Option<Rc<RefCell<Node>>> = jcall!(document).createElement(crate::org::allbinary::business::user::commerce::inventory::basket::BasketData::BasketData_SUBTOTAL.with(Clone::clone).clone());
            let mut subTotalTextNode: Option<Rc<RefCell<Node>>> = jcall!(document).createTextNode(jcall!(jcall!(basketInterface).getSubTotal()).toString());
            jcall!(subTotalNode).appendChild(subTotalTextNode.clone());
            jcall!(basketNode).appendChild(totalNumberNode.clone());
            jcall!(basketNode).appendChild(totalWeightNode.clone());
            jcall!(basketNode).appendChild(subTotalNode.clone());
            return basketNode.clone();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).XSLLOGGINGERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).FAILURE.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("toXmlNode"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
    

    pub fn addDomNodeInterfaces(&mut self) {
        self.addDomNodeInterface((self).clone() /* as DomNodeInterface */);
    }
    

    // throws Exception
    pub fn view(&mut self) -> String {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.addDomNodeInterfaces();
            return self.base.view();
        })) {
            Ok(java_return) => return java_return,
            Err(e) => {
            let e: Option<Rc<RefCell<Box<dyn std::any::Any>>>> = Some(Rc::new(RefCell::new(e as Box<dyn std::any::Any>)));

            if jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypes).LOGGING).contains(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(jcall!(unsafe { org_allbinary.clone() }).logic).communication).log).config).r#type).LogConfigTypeFactory).getInstance()).TAGHELPERERROR.clone()) {
                jcall!(self.logUtil).put(jcall!(self.commonStrings).EXCEPTION.clone(), crate::java_into_object(String::from(std::any::type_name::<Self>())), String::from("view()"), crate::java_into_object(e.clone()));
            }
            panic!("{:?}", e);
            }
        }
    }
}
impl std::ops::Deref for BasketView {
    type Target = HttpStoreComponentView;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl std::ops::DerefMut for BasketView {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

pub trait BasketViewDyn: crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn {
    fn as_BasketView(&self) -> &BasketView;
    fn as_BasketView_mut(&mut self) -> &mut BasketView;
    fn toXmlNode(&mut self, mut document: Option<Rc<RefCell<Document>>>) -> Option<Rc<RefCell<Node>>> { BasketView::toXmlNode(self.as_BasketView_mut(), document) }
    fn addDomNodeInterfaces(&mut self) { BasketView::addDomNodeInterfaces(self.as_BasketView_mut()) }
}
impl BasketViewDyn for BasketView {
    fn as_BasketView(&self) -> &BasketView { self }
    fn as_BasketView_mut(&mut self) -> &mut BasketView { self }
}

impl crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentViewDyn for BasketView {
    fn as_HttpStoreComponentView(&self) -> &crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &self.base }
    fn as_HttpStoreComponentView_mut(&mut self) -> &mut crate::views::business::context::modules::storefront::HttpStoreComponentView::HttpStoreComponentView { &mut self.base }
}
impl crate::views::HttpComponentView::HttpComponentViewDyn for BasketView {
    fn as_HttpComponentView(&self) -> &crate::views::HttpComponentView::HttpComponentView { &self.base.base }
    fn as_HttpComponentView_mut(&mut self) -> &mut crate::views::HttpComponentView::HttpComponentView { &mut self.base.base }
    fn view(&mut self) -> String { Self::view(self) }
}


